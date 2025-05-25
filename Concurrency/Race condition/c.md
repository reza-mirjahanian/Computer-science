# Race Conditions in Concurrency

## **Fundamental Definition**

A **race condition** occurs when the behavior of a program depends on the relative timing or interleaving of multiple concurrent threads or processes accessing shared resources. The outcome becomes unpredictable because it "races" on which thread executes first.

## **Core Characteristics**

- **Non-deterministic behavior**: Same input may produce different outputs
- **Timing dependency**: Results depend on execution order
- **Shared resource access**: Multiple threads modify shared data
- **Lack of synchronization**: No proper coordination between threads

## **Basic Example - The Classic Counter Problem**

### Rust Implementation
```rust
use std::thread;
use std::sync::Arc;
use std::time::Duration;

// UNSAFE VERSION - Contains race condition
static mut COUNTER: i32 = 0;

fn unsafe_increment() {
    unsafe {
        let temp = COUNTER;           // Read
        thread::sleep(Duration::from_nanos(1)); // Simulate processing
        COUNTER = temp + 1;           // Write
    }
}

fn demonstrate_race_condition() {
    let mut handles = vec![];
    
    for _ in 0..10 {
        let handle = thread::spawn(|| {
            unsafe_increment();
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    unsafe {
        println!("Counter value: {}", COUNTER); // May not be 10!
    }
}
```

### Go Implementation
```go
package main

import (
    "fmt"
    "sync"
    "time"
)

var counter int

func unsafeIncrement() {
    temp := counter                    // Read
    time.Sleep(1 * time.Nanosecond)   // Simulate processing
    counter = temp + 1                 // Write
}

func demonstrateRaceCondition() {
    var wg sync.WaitGroup
    
    for i := 0; i < 10; i++ {
        wg.Add(1)
        go func() {
            defer wg.Done()
            unsafeIncrement()
        }()
    }
    
    wg.Wait()
    fmt.Printf("Counter value: %d\n", counter) // May not be 10!
}
```

### C++ Implementation
```cpp
#include <iostream>
#include <thread>
#include <vector>
#include <chrono>

int counter = 0;

void unsafeIncrement() {
    int temp = counter;                           // Read
    std::this_thread::sleep_for(std::chrono::nanoseconds(1)); // Simulate processing
    counter = temp + 1;                          // Write
}

void demonstrateRaceCondition() {
    std::vector<std::thread> threads;
    
    for (int i = 0; i < 10; ++i) {
        threads.emplace_back(unsafeIncrement);
    }
    
    for (auto& t : threads) {
        t.join();
    }
    
    std::cout << "Counter value: " << counter << std::endl; // May not be 10!
}
```

## **Types of Race Conditions**

### **1. Data Race**
Multiple threads access shared data with at least one write operation without proper synchronization.

### **2. Logic Race**
Incorrect program logic due to assumptions about execution order.

### **3. Resource Race**
Competition for limited resources (file handles, network connections).

## **Common Race Condition Patterns**

| Pattern | Description | Risk Level |
|---------|-------------|------------|
| **Check-Then-Act** | Checking condition, then acting based on stale information | High |
| **Read-Modify-Write** | Non-atomic operations on shared variables | Critical |
| **Lazy Initialization** | Multiple threads initializing same resource | Medium |
| **Producer-Consumer** | Unsynchronized queue operations | High |

## **Check-Then-Act Race Condition**

### Rust Example
```rust
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

// UNSAFE VERSION
fn unsafe_lazy_init(cache: Arc<Mutex<HashMap<String, String>>>, key: &str) -> String {
    {
        let cache_guard = cache.lock().unwrap();
        if cache_guard.contains_key(key) {  // Check
            return cache_guard[key].clone();
        }
    } // Lock released here!
    
    // Another thread might initialize here!
    let value = expensive_computation(key);  // Act
    
    {
        let mut cache_guard = cache.lock().unwrap();
        cache_guard.insert(key.to_string(), value.clone());  // Race condition!
    }
    
    value
}

fn expensive_computation(key: &str) -> String {
    thread::sleep(std::time::Duration::from_millis(100));
    format!("computed_{}", key)
}
```

### Go Example
```go
package main

import (
    "fmt"
    "sync"
    "time"
)

type SafeCache struct {
    mu    sync.RWMutex
    cache map[string]string
}

// UNSAFE VERSION
func (c *SafeCache) unsafeGet(key string) string {
    c.mu.RLock()
    if value, exists := c.cache[key]; exists {  // Check
        c.mu.RUnlock()
        return value
    }
    c.mu.RUnlock()  // Lock released!
    
    // Another goroutine might initialize here!
    value := expensiveComputation(key)  // Act
    
    c.mu.Lock()
    c.cache[key] = value  // Race condition!
    c.mu.Unlock()
    
    return value
}

func expensiveComputation(key string) string {
    time.Sleep(100 * time.Millisecond)
    return fmt.Sprintf("computed_%s", key)
}
```

## **Producer-Consumer Race Conditions**

### C++ Example
```cpp
#include <queue>
#include <thread>
#include <iostream>

class UnsafeQueue {
private:
    std::queue<int> queue_;
    
public:
    void push(int item) {
        queue_.push(item);  // Race condition!
    }
    
    bool pop(int& item) {
        if (queue_.empty()) {  // Check
            return false;
        }
        item = queue_.front(); // Act - may crash if another thread popped!
        queue_.pop();
        return true;
    }
    
    size_t size() const {
        return queue_.size();  // Race condition with push/pop!
    }
};

void demonstrateProducerConsumerRace() {
    UnsafeQueue queue;
    
    // Producer thread
    std::thread producer([&queue]() {
        for (int i = 0; i < 1000; ++i) {
            queue.push(i);
        }
    });
    
    // Consumer thread
    std::thread consumer([&queue]() {
        int item;
        while (queue.size() > 0) {  // Race condition!
            if (queue.pop(item)) {
                std::cout << "Consumed: " << item << std::endl;
            }
        }
    });
    
    producer.join();
    consumer.join();
}
```

## **Detection Techniques**

### **1. Static Analysis Tools**

| Language | Tool | Command |
|----------|------|---------|
| **Rust** | Clippy | `cargo clippy -- -W clippy::all` |
| **Go** | Race Detector | `go run -race program.go` |
| **C++** | ThreadSanitizer | `g++ -fsanitize=thread -g program.cpp` |

### **2. Runtime Detection Example (Go)**
```go
// race_detect.go
package main

import (
    "fmt"
    "sync"
)

var data int

func increment() {
    data++  // Race condition detected at runtime
}

func main() {
    var wg sync.WaitGroup
    for i := 0; i < 100; i++ {
        wg.Add(1)
        go func() {
            defer wg.Done()
            increment()
        }()
    }
    wg.Wait()
    fmt.Println("Data:", data)
}

// Run with: go run -race race_detect.go
```

## **Solutions and Prevention**

### **1. Mutex (Mutual Exclusion)**

#### Rust - Correct Implementation
```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn safe_counter_example() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    
    for _ in 0..10 {
        let counter_clone = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter_clone.lock().unwrap();
            *num += 1;  // Atomic operation under mutex
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("Counter: {}", *counter.lock().unwrap()); // Always 10
}
```

#### Go - Correct Implementation
```go
package main

import (
    "fmt"
    "sync"
)

type SafeCounter struct {
    mu    sync.Mutex
    value int
}

func (c *SafeCounter) Increment() {
    c.mu.Lock()
    defer c.mu.Unlock()
    c.value++
}

func (c *SafeCounter) Value() int {
    c.mu.Lock()
    defer c.mu.Unlock()
    return c.value
}

func safeCounterExample() {
    counter := &SafeCounter{}
    var wg sync.WaitGroup
    
    for i := 0; i < 10; i++ {
        wg.Add(1)
        go func() {
            defer wg.Done()
            counter.Increment()
        }()
    }
    
    wg.Wait()
    fmt.Printf("Counter: %d\n", counter.Value()) // Always 10
}
```

### **2. Atomic Operations**

#### Rust Atomic Example
```rust
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;

fn atomic_counter_example() {
    let counter = Arc::new(AtomicUsize::new(0));
    let mut handles = vec![];
    
    for _ in 0..10 {
        let counter_clone = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            counter_clone.fetch_add(1, Ordering::SeqCst);  // Atomic operation
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("Atomic counter: {}", counter.load(Ordering::SeqCst)); // Always 10
}
```

#### C++ Atomic Example
```cpp
#include <atomic>
#include <thread>
#include <vector>
#include <iostream>

void atomicCounterExample() {
    std::atomic<int> counter{0};
    std::vector<std::thread> threads;
    
    for (int i = 0; i < 10; ++i) {
        threads.emplace_back([&counter]() {
            counter.fetch_add(1, std::memory_order_seq_cst);  // Atomic operation
        });
    }
    
    for (auto& t : threads) {
        t.join();
    }
    
    std::cout << "Atomic counter: " << counter.load() << std::endl; // Always 10
}
```

### **3. Lock-Free Data Structures**

#### Rust Lock-Free Queue
```rust
use crossbeam::queue::SegQueue;
use std::sync::Arc;
use std::thread;

fn lock_free_queue_example() {
    let queue = Arc::new(SegQueue::new());
    let mut handles = vec![];
    
    // Producer threads
    for i in 0..5 {
        let queue_clone = Arc::clone(&queue);
        let handle = thread::spawn(move || {
            for j in 0..10 {
                queue_clone.push(i * 10 + j);
            }
        });
        handles.push(handle);
    }
    
    // Consumer threads
    for _ in 0..3 {
        let queue_clone = Arc::clone(&queue);
        let handle = thread::spawn(move || {
            while let Some(item) = queue_clone.pop() {
                println!("Consumed: {}", item);
            }
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
}
```

## **Advanced Race Condition Patterns**

### **ABA Problem**
```cpp
#include <atomic>
#include <memory>

template<typename T>
class LockFreeStack {
private:
    struct Node {
        T data;
        Node* next;
        Node(T const& data_) : data(data_), next(nullptr) {}
    };
    
    std::atomic<Node*> head;
    
public:
    void push(T const& data) {
        Node* new_node = new Node(data);
        new_node->next = head.load();
        while (!head.compare_exchange_weak(new_node->next, new_node));
    }
    
    // ABA problem can occur here!
    std::shared_ptr<T> pop() {
        Node* old_head = head.load();
        while (old_head && 
               !head.compare_exchange_weak(old_head, old_head->next));
        
        return old_head ? std::make_shared<T>(old_head->data) : nullptr;
    }
};
```

### **Memory Ordering Issues**
```rust
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::thread;

static FLAG: AtomicBool = AtomicBool::new(false);
static DATA: AtomicUsize = AtomicUsize::new(0);

fn memory_ordering_example() {
    let producer = thread::spawn(|| {
        DATA.store(42, Ordering::Relaxed);
        FLAG.store(true, Ordering::Release);  // Release ordering
    });
    
    let consumer = thread::spawn(|| {
        while !FLAG.load(Ordering::Acquire) {  // Acquire ordering
            thread::yield_now();
        }
        let value = DATA.load(Ordering::Relaxed);
        println!("Consumed: {}", value);  // Guaranteed to see 42
    });
    
    producer.join().unwrap();
    consumer.join().unwrap();
}
```

## **Performance Considerations**

| Synchronization Method | Performance | Scalability | Complexity |
|------------------------|-------------|-------------|------------|
| **Mutex** | Medium | Poor | Low |
| **RWLock** | Good (read-heavy) | Medium | Medium |
| **Atomic Operations** | High | Good | Medium |
| **Lock-Free Structures** | Highest | Excellent | High |
| **Actor Model** | Good | Excellent | High |

## **Best Practices**

### **1. Design Principles**
- **Minimize shared state**: Prefer message passing over shared memory
- **Immutable data**: Use immutable structures when possible
- **Single responsibility**: One thread should own each piece of data
- **Clear ownership**: Use language features like Rust's ownership system

### **2. Testing Strategies**
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::thread;
    
    #[test]
    fn test_concurrent_operations() {
        let counter = Arc::new(SafeCounter::new());
        let mut handles = vec![];
        
        for _ in 0..100 {
            let counter_clone = Arc::clone(&counter);
            let handle = thread::spawn(move || {
                for _ in 0..100 {
                    counter_clone.increment();
                }
            });
            handles.push(handle);
        }
        
        for handle in handles {
            handle.join().unwrap();
        }
        
        assert_eq!(counter.value(), 10000);
    }
}
```

## **Language-Specific Features**

### **Rust Ownership System**
```rust
// Rust prevents many race conditions at compile time
fn rust_safety_example() {
    let data = vec![1, 2, 3, 4, 5];
    
    // This won't compile - borrowing rules prevent races
    /*
    let handle1 = thread::spawn(|| {
        data.push(6);  // Error: data moved
    });
    
    let handle2 = thread::spawn(|| {
        println!("{:?}", data);  // Error: data already moved
    });
    */
    
    // Correct approach using Arc and Mutex
    let data = Arc::new(Mutex::new(vec![1, 2, 3, 4, 5]));
    let data1 = Arc::clone(&data);
    let data2 = Arc::clone(&data);
    
    let handle1 = thread::spawn(move || {
        data1.lock().unwrap().push(6);
    });
    
    let handle2 = thread::spawn(move || {
        println!("{:?}", *data2.lock().unwrap());
    });
    
    handle1.join().unwrap();
    handle2.join().unwrap();
}
```

### **Go Channels**
```go
package main

import (
    "fmt"
    "sync"
)

// Channel-based solution avoiding shared state
func channelBasedCounter() {
    const numWorkers = 10
    const numIncrements = 100
    
    incrementChan := make(chan struct{}, numWorkers*numIncrements)
    resultChan := make(chan int)
    
    // Counter goroutine
    go func() {
        count := 0
        for range incrementChan {
            count++
        }
        resultChan <- count
    }()
    
    // Worker goroutines
    var wg sync.WaitGroup
    for i := 0; i < numWorkers; i++ {
        wg.Add(1)
        go func() {
            defer wg.Done()
            for j := 0; j < numIncrements; j++ {
                incrementChan <- struct{}{}
            }
        }()
    }
    
    go func() {
        wg.Wait()
        close(incrementChan)
    }()
    
    result := <-resultChan
    fmt.Printf("Final count: %d\n", result) // Always 1000
}
```

Race conditions are fundamental challenges in concurrent programming that require deep understanding of memory models, synchronization primitives, and language-specific features. Mastery comes through recognizing patterns, understanding the underlying hardware and software models, and choosing appropriate solutions based on performance and correctness requirements.