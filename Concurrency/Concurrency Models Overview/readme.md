## **Concurrency Models Overview**

### **1. Shared Memory with Locks**

**Description**: Multiple threads share memory and use synchronization primitives (mutexes, semaphores) to coordinate access.

**Implementation Examples:**

**C++ (std::mutex)**
```cpp
#include <mutex>
#include <thread>
#include <vector>

class Counter {
    mutable std::mutex mtx;
    int value = 0;
public:
    void increment() {
        std::lock_guard<std::mutex> lock(mtx);
        ++value;
    }
    
    int get() const {
        std::lock_guard<std::mutex> lock(mtx);
        return value;
    }
};

// Deadlock example - always lock in same order
void transfer(Account& from, Account& to, int amount) {
    // Lock accounts by ID to prevent deadlock
    if (from.id < to.id) {
        std::lock_guard<std::mutex> lock1(from.mtx);
        std::lock_guard<std::mutex> lock2(to.mtx);
        from.balance -= amount;
        to.balance += amount;
    } else {
        std::lock_guard<std::mutex> lock1(to.mtx);
        std::lock_guard<std::mutex> lock2(from.mtx);
        from.balance -= amount;
        to.balance += amount;
    }
}
```

**Rust (Mutex + Arc)**
```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("Result: {}", *counter.lock().unwrap());
}

// RwLock for multiple readers
use std::sync::RwLock;

let lock = RwLock::new(5);
{
    let r1 = lock.read().unwrap();
    let r2 = lock.read().unwrap(); // Multiple readers OK
    assert_eq!(*r1, 5);
    assert_eq!(*r2, 5);
} // read locks dropped here

{
    let mut w = lock.write().unwrap();
    *w += 1;
}
```

| **Pros** | **Cons** |
|----------|----------|
| Direct memory access | Deadlock potential |
| Fine-grained control | Race conditions |
| Low overhead for simple cases | Complex debugging |
| Familiar programming model | Lock contention |

**Complexity**: $O(1)$ for lock/unlock operations, but contention can degrade to $O(n)$

### **2. Message Passing (Actor Model)**

**Description**: Processes communicate by sending messages through channels, avoiding shared state.

**Go (Channels)**
```go
package main

import (
    "fmt"
    "time"
)

// Basic channel usage
func worker(id int, jobs <-chan int, results chan<- int) {
    for j := range jobs {
        fmt.Printf("worker %d processing job %d\n", id, j)
        time.Sleep(time.Second)
        results <- j * 2
    }
}

func main() {
    jobs := make(chan int, 100)
    results := make(chan int, 100)
    
    // Start workers
    for w := 1; w <= 3; w++ {
        go worker(w, jobs, results)
    }
    
    // Send jobs
    for j := 1; j <= 5; j++ {
        jobs <- j
    }
    close(jobs)
    
    // Collect results
    for a := 1; a <= 5; a++ {
        <-results
    }
}

// Select statement for non-blocking ops
func nonBlockingReceive() {
    messages := make(chan string)
    signals := make(chan bool)
    
    select {
    case msg := <-messages:
        fmt.Println("received message", msg)
    case sig := <-signals:
        fmt.Println("received signal", sig)
    default:
        fmt.Println("no activity")
    }
}

// Buffered vs Unbuffered
func channelTypes() {
    // Unbuffered - synchronous
    ch1 := make(chan int)
    
    // Buffered - asynchronous up to buffer size
    ch2 := make(chan int, 10)
    
    // Directional channels
    var send chan<- int = ch1
    var recv <-chan int = ch1
}
```

**Rust (mpsc channels)**
```rust
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    // Multiple producer, single consumer
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();
    
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("thread"),
        ];
        
        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    
    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
        ];
        
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    
    for received in rx {
        println!("Got: {}", received);
    }
}

// Sync channel (bounded)
use std::sync::mpsc::sync_channel;

let (tx, rx) = sync_channel(3); // buffer size 3
```

| **Pros** | **Cons** |
|----------|----------|
| No shared state | Message copying overhead |
| Deadlock-free by design | Indirect data access |
| Easy to reason about | Potential for message queue overflow |
| Natural distribution | Serialization costs |

**Complexity**: $O(1)$ for send/receive, $O(n)$ for broadcast

### **3. Lock-Free Programming**

**Description**: Uses atomic operations and careful algorithm design to avoid locks.

**C++ (Atomic Operations)**
```cpp
#include <atomic>
#include <thread>

// Lock-free counter
class LockFreeCounter {
    std::atomic<int> value{0};
public:
    void increment() {
        value.fetch_add(1, std::memory_order_relaxed);
    }
    
    int get() const {
        return value.load(std::memory_order_relaxed);
    }
};

// Compare-and-swap (CAS) loop
template<typename T>
class LockFreeStack {
    struct Node {
        T data;
        Node* next;
    };
    
    std::atomic<Node*> head{nullptr};
    
public:
    void push(T data) {
        Node* new_node = new Node{std::move(data), nullptr};
        new_node->next = head.load();
        
        while (!head.compare_exchange_weak(new_node->next, new_node)) {
            // CAS failed, retry
        }
    }
    
    bool pop(T& result) {
        Node* old_head = head.load();
        
        while (old_head && 
               !head.compare_exchange_weak(old_head, old_head->next)) {
            // CAS failed, retry
        }
        
        if (old_head) {
            result = std::move(old_head->data);
            delete old_head;
            return true;
        }
        return false;
    }
};

// Memory ordering examples
void memory_ordering_demo() {
    std::atomic<bool> flag{false};
    std::atomic<int> data{0};
    
    // Producer
    std::thread producer([&]() {
        data.store(42, std::memory_order_relaxed);
        flag.store(true, std::memory_order_release);
    });
    
    // Consumer
    std::thread consumer([&]() {
        while (!flag.load(std::memory_order_acquire))
            ;
        assert(data.load(std::memory_order_relaxed) == 42);
    });
    
    producer.join();
    consumer.join();
}
```

**Rust (Atomics)**
```rust
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;

fn main() {
    let counter = Arc::new(AtomicUsize::new(0));
    let mut handles = vec![];
    
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for _ in 0..1000 {
                counter.fetch_add(1, Ordering::SeqCst);
            }
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("Result: {}", counter.load(Ordering::SeqCst));
}

// Spinlock implementation
use std::sync::atomic::{AtomicBool, Ordering};
use std::cell::UnsafeCell;

pub struct SpinLock<T> {
    locked: AtomicBool,
    data: UnsafeCell<T>,
}

unsafe impl<T: Send> Sync for SpinLock<T> {}
unsafe impl<T: Send> Send for SpinLock<T> {}

impl<T> SpinLock<T> {
    pub fn new(data: T) -> Self {
        SpinLock {
            locked: AtomicBool::new(false),
            data: UnsafeCell::new(data),
        }
    }
    
    pub fn lock(&self) -> SpinLockGuard<T> {
        while self.locked.compare_exchange_weak(
            false,
            true,
            Ordering::Acquire,
            Ordering::Relaxed
        ).is_err() {
            std::hint::spin_loop();
        }
        SpinLockGuard { lock: self }
    }
}
```

| **Pros** | **Cons** |
|----------|----------|
| No blocking | Complex to implement correctly |
| High performance | Limited use cases |
| No priority inversion | ABA problem |
| Predictable latency | Memory ordering complexity |

**Complexity**: $O(1)$ best case, $O(∞)$ worst case for CAS loops

### **4. Software Transactional Memory (STM)**

**Description**: Treats memory operations as transactions that can be committed or rolled back.

**C++ (Conceptual STM)**
```cpp
// Pseudo-code for STM concept
template<typename T>
class TVar {
    std::atomic<Version> version;
    T value;
public:
    T read_in_transaction(Transaction& tx);
    void write_in_transaction(Transaction& tx, T new_value);
};

class Transaction {
    std::vector<ReadLog> reads;
    std::vector<WriteLog> writes;
    
public:
    template<typename F>
    auto atomically(F&& func) {
        while (true) {
            try {
                auto result = func(*this);
                if (validate() && commit()) {
                    return result;
                }
            } catch (const RetryException&) {
                // Retry transaction
            }
            rollback();
        }
    }
};

// Usage
void transfer(TVar<int>& from, TVar<int>& to, int amount) {
    atomically([&](Transaction& tx) {
        int from_balance = from.read_in_transaction(tx);
        int to_balance = to.read_in_transaction(tx);
        
        if (from_balance < amount) {
            throw RetryException();
        }
        
        from.write_in_transaction(tx, from_balance - amount);
        to.write_in_transaction(tx, to_balance + amount);
    });
}
```

### **5. Async/Await (Cooperative Concurrency)**

**Rust (async/await)**
```rust
use tokio::time::{sleep, Duration};
use tokio::join;

async fn fetch_data(id: u32) -> String {
    sleep(Duration::from_millis(100)).await;
    format!("Data {}", id)
}

#[tokio::main]
async fn main() {
    // Concurrent execution
    let (data1, data2, data3) = join!(
        fetch_data(1),
        fetch_data(2),
        fetch_data(3)
    );
    
    println!("{}, {}, {}", data1, data2, data3);
}

// Select for racing futures
use tokio::select;

async fn timeout_example() {
    let sleep1 = sleep(Duration::from_millis(100));
    let sleep2 = sleep(Duration::from_millis(200));
    
    select! {
        _ = sleep1 => println!("sleep1 completed first"),
        _ = sleep2 => println!("sleep2 completed first"),
    }
}

// Channels in async context
use tokio::sync::mpsc;

async fn async_channels() {
    let (tx, mut rx) = mpsc::channel(32);
    
    tokio::spawn(async move {
        for i in 0..10 {
            tx.send(i).await.unwrap();
        }
    });
    
    while let Some(i) = rx.recv().await {
        println!("got = {}", i);
    }
}
```

**Go (Goroutines)**
```go
package main

import (
    "context"
    "fmt"
    "sync"
    "time"
)

// WaitGroup for synchronization
func waitGroupExample() {
    var wg sync.WaitGroup
    
    for i := 0; i < 5; i++ {
        wg.Add(1)
        go func(id int) {
            defer wg.Done()
            time.Sleep(time.Second)
            fmt.Printf("Worker %d done\n", id)
        }(i)
    }
    
    wg.Wait()
    fmt.Println("All workers completed")
}

// Context for cancellation
func contextExample() {
    ctx, cancel := context.WithTimeout(
        context.Background(), 
        2*time.Second,
    )
    defer cancel()
    
    select {
    case <-time.After(1 * time.Second):
        fmt.Println("operation completed")
    case <-ctx.Done():
        fmt.Println("operation cancelled:", ctx.Err())
    }
}

// Worker pool pattern
func workerPool() {
    jobs := make(chan int, 100)
    results := make(chan int, 100)
    
    // Start workers
    for w := 1; w <= 3; w++ {
        go func(id int) {
            for job := range jobs {
                fmt.Printf("worker %d processing %d\n", id, job)
                time.Sleep(time.Millisecond * 500)
                results <- job * job
            }
        }(w)
    }
    
    // Send work
    for j := 1; j <= 10; j++ {
        jobs <- j
    }
    close(jobs)
    
    // Collect results
    for r := 1; r <= 10; r++ {
        <-results
    }
}
```

### **Comparison Table**

| **Model** | **Best For** | **Avoid When** | **Complexity** |
|-----------|--------------|----------------|----------------|
| **Shared Memory + Locks** | Fine-grained control, legacy code | Many threads contend | Medium |
| **Message Passing** | Distributed systems, isolation | High-frequency small updates | Low |
| **Lock-Free** | Ultra-low latency, high contention | Complex data structures | Very High |
| **STM** | Complex invariants, composability | High write contention | High |
| **Async/Await** | I/O bound tasks, many connections | CPU-bound computation | Medium |

### **Memory Models and Ordering**

**C++ Memory Ordering**
```cpp
// Relaxed - no synchronization
atomic_var.store(42, std::memory_order_relaxed);

// Acquire-Release - synchronizes with matching release
flag.store(true, std::memory_order_release);
while (!flag.load(std::memory_order_acquire));

// Sequential Consistency - total order
data.store(42, std::memory_order_seq_cst);
```

### **Common Pitfalls**

1. **Data Races**
```rust
// BAD - data race
let mut data = vec![1, 2, 3];
let handle = thread::spawn(|| {
    data.push(4); // Error: cannot borrow
});

// GOOD - use Arc<Mutex<T>>
let data = Arc::new(Mutex::new(vec![1, 2, 3]));
let data_clone = Arc::clone(&data);
let handle = thread::spawn(move || {
    data_clone.lock().unwrap().push(4);
});
```

2. **Deadlock Prevention**
```cpp
// Always acquire locks in same order
std::lock(mutex1, mutex2); // Acquires both atomically
std::lock_guard<std::mutex> lk1(mutex1, std::adopt_lock);
std::lock_guard<std::mutex> lk2(mutex2, std::adopt_lock);
```

3. **False Sharing**
```cpp
// BAD - false sharing
struct alignas(64) Counter {
    std::atomic<int> count{0};
    // Padding to prevent false sharing
    char padding[64 - sizeof(std::atomic<int>)];
};
```

### **Performance Characteristics**

| **Operation** | **Time Complexity** | **Space Complexity** |
|---------------|-------------------|-------------------|
| Mutex Lock/Unlock | $O(1)$ amortized | $O(1)$ |
| Channel Send/Recv | $O(1)$ | $O(1)$ |
| Atomic CAS | $O(1)$ best, $O(n)$ contended | $O(1)$ |
| STM Transaction | $O(r + w)$ (reads + writes) | $O(r + w)$ |

### **Next Steps Suggestion**

**Advanced Topic**: **Wait-Free Data Structures and Hazard Pointers** - Explore the design and implementation of wait-free algorithms that guarantee progress for every thread within a bounded number of steps, including memory reclamation techniques like hazard pointers and epoch-based reclamation for building truly scalable concurrent data structures.