## **Lock-Free Data Structures**

### **Fundamental Concepts**

**Lock-free programming** is a concurrency technique where threads can make progress without ever blocking on locks. A data structure is **lock-free** if at least one thread can make progress in a finite number of steps, regardless of what other threads are doing.

### **Key Properties**

- **Non-blocking**: Threads never wait for locks
- **Progress guarantee**: At least one thread always makes progress
- **No deadlocks**: Impossible by design
- **Better scalability**: Performance improves with more cores
- **Predictable latency**: No unpredictable blocking

### **Building Blocks**

#### **1. Atomic Operations**

The foundation of lock-free programming:

```cpp
#include <atomic>

// Basic atomic types
std::atomic<int> counter{0};
std::atomic<bool> flag{false};
std::atomic<void*> pointer{nullptr};

// Atomic operations
counter.fetch_add(1);  // Atomic increment
counter.compare_exchange_strong(expected, desired);  // CAS operation
```

#### **2. Memory Ordering**

| **Memory Order** | **Description** | **Use Case** |
|-----------------|-----------------|--------------|
| `memory_order_relaxed` | No synchronization | Counters, flags |
| `memory_order_acquire` | Prevents reordering after | Lock acquisition |
| `memory_order_release` | Prevents reordering before | Lock release |
| `memory_order_acq_rel` | Both acquire and release | Read-modify-write |
| `memory_order_seq_cst` | Total ordering | Default, safest |

### **Lock-Free Stack**

#### **Treiber Stack Implementation**

```rust
use std::sync::atomic::{AtomicPtr, Ordering};
use std::ptr;

pub struct Node<T> {
    data: T,
    next: *mut Node<T>,
}

pub struct LockFreeStack<T> {
    head: AtomicPtr<Node<T>>,
}

impl<T> LockFreeStack<T> {
    pub fn new() -> Self {
        LockFreeStack {
            head: AtomicPtr::new(ptr::null_mut()),
        }
    }

    pub fn push(&self, data: T) {
        let new_node = Box::into_raw(Box::new(Node {
            data,
            next: ptr::null_mut(),
        }));

        loop {
            let head = self.head.load(Ordering::Acquire);
            unsafe {
                (*new_node).next = head;
            }
            
            match self.head.compare_exchange_weak(
                head,
                new_node,
                Ordering::Release,
                Ordering::Acquire,
            ) {
                Ok(_) => break,
                Err(_) => continue,
            }
        }
    }

    pub fn pop(&self) -> Option<T> {
        loop {
            let head = self.head.load(Ordering::Acquire);
            if head.is_null() {
                return None;
            }

            let next = unsafe { (*head).next };
            
            match self.head.compare_exchange_weak(
                head,
                next,
                Ordering::Release,
                Ordering::Acquire,
            ) {
                Ok(_) => {
                    let data = unsafe { Box::from_raw(head).data };
                    return Some(data);
                }
                Err(_) => continue,
            }
        }
    }
}
```

#### **ABA Problem and Solution**

The **ABA problem** occurs when:
1. Thread 1 reads value A
2. Thread 2 changes A to B, then back to A
3. Thread 1's CAS succeeds incorrectly

**Solution with Hazard Pointers:**

```cpp
template<typename T>
class HazardPointerStack {
private:
    struct Node {
        T data;
        std::atomic<Node*> next;
    };
    
    std::atomic<Node*> head{nullptr};
    
    // Hazard pointer management
    static thread_local std::atomic<Node*>* hazard_pointer;
    static std::vector<std::atomic<Node*>*> all_hazard_pointers;
    
public:
    void push(T data) {
        Node* new_node = new Node{std::move(data), nullptr};
        Node* old_head;
        
        do {
            old_head = head.load();
            new_node->next.store(old_head);
        } while (!head.compare_exchange_weak(old_head, new_node));
    }
    
    std::optional<T> pop() {
        Node* old_head;
        
        do {
            old_head = head.load();
            if (!old_head) return std::nullopt;
            
            // Protect with hazard pointer
            hazard_pointer->store(old_head);
            
            // Double-check
            if (head.load() != old_head) continue;
            
        } while (!head.compare_exchange_weak(old_head, old_head->next.load()));
        
        T data = std::move(old_head->data);
        
        // Clear hazard pointer before retiring
        hazard_pointer->store(nullptr);
        retire_node(old_head);
        
        return data;
    }
    
private:
    void retire_node(Node* node) {
        // Check if any hazard pointer references this node
        for (auto& hp : all_hazard_pointers) {
            if (hp->load() == node) {
                // Defer deletion
                return;
            }
        }
        delete node;
    }
};
```

### **Lock-Free Queue**

#### **Michael & Scott Queue**

```go
package lockfree

import (
    "sync/atomic"
    "unsafe"
)

type node[T any] struct {
    value T
    next  unsafe.Pointer // *node[T]
}

type Queue[T any] struct {
    head unsafe.Pointer // *node[T]
    tail unsafe.Pointer // *node[T]
}

func NewQueue[T any]() *Queue[T] {
    dummy := &node[T]{}
    return &Queue[T]{
        head: unsafe.Pointer(dummy),
        tail: unsafe.Pointer(dummy),
    }
}

func (q *Queue[T]) Enqueue(value T) {
    newNode := &node[T]{value: value}
    
    for {
        tail := (*node[T])(atomic.LoadPointer(&q.tail))
        next := (*node[T])(atomic.LoadPointer(&tail.next))
        
        // Check tail consistency
        if tail != (*node[T])(atomic.LoadPointer(&q.tail)) {
            continue
        }
        
        if next == nil {
            // Try to link new node
            if atomic.CompareAndSwapPointer(&tail.next, 
                unsafe.Pointer(next), 
                unsafe.Pointer(newNode)) {
                // Try to update tail
                atomic.CompareAndSwapPointer(&q.tail,
                    unsafe.Pointer(tail),
                    unsafe.Pointer(newNode))
                return
            }
        } else {
            // Help update tail
            atomic.CompareAndSwapPointer(&q.tail,
                unsafe.Pointer(tail),
                unsafe.Pointer(next))
        }
    }
}

func (q *Queue[T]) Dequeue() (T, bool) {
    var zero T
    
    for {
        head := (*node[T])(atomic.LoadPointer(&q.head))
        tail := (*node[T])(atomic.LoadPointer(&q.tail))
        next := (*node[T])(atomic.LoadPointer(&head.next))
        
        // Check head consistency
        if head != (*node[T])(atomic.LoadPointer(&q.head)) {
            continue
        }
        
        if head == tail {
            if next == nil {
                // Queue is empty
                return zero, false
            }
            // Help update tail
            atomic.CompareAndSwapPointer(&q.tail,
                unsafe.Pointer(tail),
                unsafe.Pointer(next))
        } else {
            // Read value before CAS
            value := next.value
            
            if atomic.CompareAndSwapPointer(&q.head,
                unsafe.Pointer(head),
                unsafe.Pointer(next)) {
                return value, true
            }
        }
    }
}
```

### **Lock-Free Hash Table**

#### **Split-Ordered List Implementation**

```rust
use std::sync::atomic::{AtomicPtr, AtomicUsize, Ordering};
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

const INITIAL_BUCKETS: usize = 16;

struct Node<K, V> {
    key: K,
    value: V,
    hash: usize,
    next: AtomicPtr<Node<K, V>>,
}

pub struct LockFreeHashMap<K: Hash + Eq, V> {
    buckets: Vec<AtomicPtr<Node<K, V>>>,
    size: AtomicUsize,
}

impl<K: Hash + Eq, V> LockFreeHashMap<K, V> {
    pub fn new() -> Self {
        let mut buckets = Vec::with_capacity(INITIAL_BUCKETS);
        for _ in 0..INITIAL_BUCKETS {
            buckets.push(AtomicPtr::new(std::ptr::null_mut()));
        }
        
        LockFreeHashMap {
            buckets,
            size: AtomicUsize::new(0),
        }
    }
    
    fn hash_key(key: &K) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        hasher.finish() as usize
    }
    
    pub fn insert(&self, key: K, value: V) -> Option<V> {
        let hash = Self::hash_key(&key);
        let bucket_idx = hash & (self.buckets.len() - 1);
        
        let new_node = Box::into_raw(Box::new(Node {
            key,
            value,
            hash,
            next: AtomicPtr::new(std::ptr::null_mut()),
        }));
        
        loop {
            let mut prev_ptr = &self.buckets[bucket_idx];
            let mut current = prev_ptr.load(Ordering::Acquire);
            
            // Find insertion point (ordered by hash)
            unsafe {
                while !current.is_null() && (*current).hash < hash {
                    prev_ptr = &(*current).next;
                    current = prev_ptr.load(Ordering::Acquire);
                }
                
                // Check if key already exists
                if !current.is_null() && (*current).hash == hash && (*current).key == (*new_node).key {
                    // Update existing value
                    let old_value = std::ptr::replace(&mut (*current).value, (*new_node).value);
                    Box::from_raw(new_node); // Clean up
                    return Some(old_value);
                }
                
                // Insert new node
                (*new_node).next.store(current, Ordering::Relaxed);
                
                match prev_ptr.compare_exchange_weak(
                    current,
                    new_node,
                    Ordering::Release,
                    Ordering::Acquire,
                ) {
                    Ok(_) => {
                        self.size.fetch_add(1, Ordering::Relaxed);
                        return None;
                    }
                    Err(_) => continue,
                }
            }
        }
    }
    
    pub fn get(&self, key: &K) -> Option<&V> {
        let hash = Self::hash_key(key);
        let bucket_idx = hash & (self.buckets.len() - 1);
        
        let mut current = self.buckets[bucket_idx].load(Ordering::Acquire);
        
        unsafe {
            while !current.is_null() {
                if (*current).hash == hash && (*current).key == *key {
                    return Some(&(*current).value);
                }
                current = (*current).next.load(Ordering::Acquire);
            }
        }
        
        None
    }
}
```

### **Lock-Free Memory Management**

#### **Epoch-Based Reclamation (EBR)**

```cpp
template<typename T>
class EpochBasedReclamation {
private:
    struct ThreadData {
        std::atomic<uint64_t> epoch{0};
        std::vector<T*> retire_list[3];
    };
    
    std::atomic<uint64_t> global_epoch{0};
    static thread_local ThreadData* local_data;
    std::vector<ThreadData*> all_threads;
    
public:
    class Guard {
    private:
        ThreadData* data;
        uint64_t epoch;
        
    public:
        Guard() : data(local_data) {
            epoch = global_epoch.load(Ordering::Acquire);
            data->epoch.store(epoch, Ordering::Release);
        }
        
        ~Guard() {
            data->epoch.store(UINT64_MAX, Ordering::Release);
        }
    };
    
    void retire(T* ptr) {
        auto epoch = global_epoch.load(Ordering::Acquire);
        local_data->retire_list[epoch % 3].push_back(ptr);
    }
    
    void collect() {
        auto new_epoch = global_epoch.load() + 1;
        global_epoch.store(new_epoch, Ordering::Release);
        
        // Check if safe to collect old epoch
        auto safe_epoch = new_epoch - 2;
        if (safe_epoch < 2) return;
        
        bool all_advanced = true;
        for (auto* thread : all_threads) {
            auto thread_epoch = thread->epoch.load(Ordering::Acquire);
            if (thread_epoch != UINT64_MAX && thread_epoch <= safe_epoch) {
                all_advanced = false;
                break;
            }
        }
        
        if (all_advanced) {
            // Safe to delete
            for (auto* ptr : local_data->retire_list[safe_epoch % 3]) {
                delete ptr;
            }
            local_data->retire_list[safe_epoch % 3].clear();
        }
    }
};
```

### **Wait-Free Data Structures**

**Wait-free** is stronger than lock-free: **every** thread makes progress in bounded steps.

#### **Wait-Free Counter**

```go
type WaitFreeCounter struct {
    counters []atomic.Uint64
}

func NewWaitFreeCounter(numThreads int) *WaitFreeCounter {
    return &WaitFreeCounter{
        counters: make([]atomic.Uint64, numThreads),
    }
}

func (c *WaitFreeCounter) Increment(threadID int) {
    c.counters[threadID].Add(1)
}

func (c *WaitFreeCounter) Get() uint64 {
    var sum uint64
    for i := range c.counters {
        sum += c.counters[i].Load()
    }
    return sum
}
```

### **Comparison with Lock-Based Structures**

| **Aspect** | **Lock-Free** | **Lock-Based** |
|------------|---------------|----------------|
| **Progress Guarantee** | At least one thread | No guarantee |
| **Deadlock** | Impossible | Possible |
| **Priority Inversion** | Reduced | Common |
| **Complexity** | High | Low |
| **Cache Performance** | Better under contention | Worse under contention |
| **Debugging** | Difficult | Easier |

### **Performance Characteristics**

#### **Benchmark Example**

```rust
use std::sync::Arc;
use std::thread;
use std::time::Instant;

fn benchmark_lock_free_vs_mutex() {
    const NUM_THREADS: usize = 8;
    const OPS_PER_THREAD: usize = 1_000_000;
    
    // Lock-free stack benchmark
    let lock_free_stack = Arc::new(LockFreeStack::new());
    let start = Instant::now();
    
    let mut handles = vec![];
    for _ in 0..NUM_THREADS {
        let stack = Arc::clone(&lock_free_stack);
        handles.push(thread::spawn(move || {
            for i in 0..OPS_PER_THREAD {
                stack.push(i);
                stack.pop();
            }
        }));
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("Lock-free time: {:?}", start.elapsed());
    
    // Compare with mutex-based implementation
    use std::sync::Mutex;
    let mutex_stack = Arc::new(Mutex::new(Vec::new()));
    let start = Instant::now();
    
    let mut handles = vec![];
    for _ in 0..NUM_THREADS {
        let stack = Arc::clone(&mutex_stack);
        handles.push(thread::spawn(move || {
            for i in 0..OPS_PER_THREAD {
                {
                    let mut s = stack.lock().unwrap();
                    s.push(i);
                }
                {
                    let mut s = stack.lock().unwrap();
                    s.pop();
                }
            }
        }));
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("Mutex-based time: {:?}", start.elapsed());
}
```

### **Advanced Techniques**

#### **1. Helping Mechanism**

```cpp
template<typename T>
class HelpingQueue {
private:
    struct Operation {
        enum Type { ENQUEUE, DEQUEUE } type;
        T* data;
        std::atomic<bool> completed{false};
    };
    
    std::atomic<Operation*> pending_ops[MAX_THREADS];
    
    void help_others() {
        for (int i = 0; i < MAX_THREADS; ++i) {
            Operation* op = pending_ops[i].load();
            if (op && !op->completed.load()) {
                execute_operation(op);
            }
        }
    }
    
public:
    void enqueue(T data) {
        Operation op{Operation::ENQUEUE, &data, false};
        pending_ops[thread_id].store(&op);
        
        help_others();
        execute_operation(&op);
        
        pending_ops[thread_id].store(nullptr);
    }
};
```

#### **2. Elimination Backoff**

```rust
pub struct EliminationStack<T> {
    stack: LockFreeStack<T>,
    elimination_array: Vec<AtomicPtr<Option<T>>>,
}

impl<T> EliminationStack<T> {
    pub fn push(&self, value: T) {
        // Try elimination first
        let slot = thread_rng().gen_range(0..self.elimination_array.len());
        let exchange_value = Some(value);
        
        let ptr = Box::into_raw(Box::new(exchange_value));
        let old = self.elimination_array[slot].swap(ptr, Ordering::AcqRel);
        
        if !old.is_null() {
            // Successful elimination with a pop operation
            unsafe { Box::from_raw(ptr) }; // Clean up
            return;
        }
        
        // Wait briefly for elimination
        std::thread::yield_now();
        
        // Check if eliminated
        let current = self.elimination_array[slot].load(Ordering::Acquire);
        if current != ptr {
            // Eliminated!
            unsafe { Box::from_raw(ptr) }; // Clean up
            return;
        }
        
        // Failed elimination, use regular stack
        self.elimination_array[slot].store(std::ptr::null_mut(), Ordering::Release);
        let value = unsafe { Box::from_raw(ptr).unwrap() };
        self.stack.push(value);
    }
}
```

### **Correctness Verification**

#### **Linearizability Testing**

```go
type OperationLog struct {
    Op        string
    Value     interface{}
    StartTime int64
    EndTime   int64
}

func VerifyLinearizability(logs []OperationLog) bool {
    // Sort by start time
    sort.Slice(logs, func(i, j int) bool {
        return logs[i].StartTime < logs[j].StartTime
    })
    
    // Check all possible linearizations
    return checkLinearization(logs, 0, []OperationLog{})
}

func checkLinearization(logs []OperationLog, index int, sequence []OperationLog) bool {
    if index == len(logs) {
        return isValidSequence(sequence)
    }
    
    // Try all valid positions for current operation
    op := logs[index]
    for i := 0; i <= len(sequence); i++ {
        if canPlaceAt(sequence, i, op) {
            newSeq := make([]OperationLog, len(sequence)+1)
            copy(newSeq[:i], sequence[:i])
            newSeq[i] = op
            copy(newSeq[i+1:], sequence[i:])
            
            if checkLinearization(logs, index+1, newSeq) {
                return true
            }
        }
    }
    
    return false
}
```

### **Common Pitfalls and Solutions**

1. **Memory Ordering Bugs**
   - Always use appropriate memory ordering
   - Default to `seq_cst` when unsure
   - Use tools like ThreadSanitizer

2. **ABA Problem**
   - Use hazard pointers or epochs
   - Add version numbers to pointers
   - Use double-width CAS when available

3. **Memory Leaks**
   - Implement proper memory reclamation
   - Test with memory leak detectors
   - Consider reference counting

4. **Starvation**
   - Add helping mechanisms
   - Use fair algorithms (like ticket locks)
   - Monitor thread progress

5. **Performance Degradation**
   - Profile under realistic workloads
   - Consider cache effects
   - Use backoff strategies

### **Best Practices**

1. **Start with proven algorithms**
2. **Use memory reclamation schemes**
3. **Test extensively with race detectors**
4. **Benchmark against lock-based alternatives**
5. **Document memory ordering requirements**
6. **Consider hardware capabilities**
7. **Implement progress monitoring**
8. **Use formal verification when possible**
9. **Provide fallback mechanisms**
10. **Keep algorithms simple and understandable**