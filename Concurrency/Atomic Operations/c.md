 # Atomic Operations in Concurrent Programming

## **Fundamental Concepts**

### **What are Atomic Operations?**

Atomic operations are **indivisible** operations that complete in a single step relative to other threads. When one thread performs an atomic operation, no other thread can observe the operation in a partially-completed state.

**Key Properties:**
- **Indivisibility**: The operation executes as a single, uninterruptible unit
- **Visibility**: Changes are immediately visible to all threads
- **Ordering**: Provides guarantees about memory synchronization

### **Why Atomic Operations Matter**

Consider this **race condition** example:

**C++ (Non-Atomic)**
```cpp
int counter = 0;

// Thread 1
counter++;  // Read, Modify, Write

// Thread 2  
counter++;  // Read, Modify, Write
```

The `counter++` operation actually involves:
1. **Read** current value
2. **Modify** (increment)
3. **Write** new value back

Without atomicity, threads can interleave:
```
Thread 1: Read (0)
Thread 2: Read (0)
Thread 1: Increment (1)
Thread 2: Increment (1)
Thread 1: Write (1)
Thread 2: Write (1)
Final: counter = 1 (Wrong! Should be 2)
```

## **Memory Models and Ordering**

### **Memory Ordering Levels**

| **Ordering** | **Description** | **Use Case** |
|-------------|-----------------|--------------|
| **Relaxed** | No synchronization or ordering constraints | Simple counters, statistics |
| **Acquire** | Prevents memory reordering of reads/writes after this operation | Lock acquisition |
| **Release** | Prevents memory reordering of reads/writes before this operation | Lock release |
| **AcqRel** | Both Acquire and Release semantics | Read-modify-write operations |
| **SeqCst** | Sequential consistency - total order of all operations | Default safe choice |

### **Implementation Examples**

**Rust**
```rust
use std::sync::atomic::{AtomicI32, Ordering};

let counter = AtomicI32::new(0);

// Relaxed ordering
counter.fetch_add(1, Ordering::Relaxed);

// Sequential consistency
counter.store(42, Ordering::SeqCst);
let val = counter.load(Ordering::SeqCst);

// Acquire-Release pattern
let flag = AtomicBool::new(false);
// Producer
data = prepare_data();
flag.store(true, Ordering::Release);

// Consumer
while !flag.load(Ordering::Acquire) {}
// data is now safely accessible
```

**Go**
```go
import "sync/atomic"

var counter int64

// Add atomically
atomic.AddInt64(&counter, 1)

// Load atomically
val := atomic.LoadInt64(&counter)

// Store atomically
atomic.StoreInt64(&counter, 42)

// Compare and Swap
old := int64(10)
new := int64(20)
swapped := atomic.CompareAndSwapInt64(&counter, old, new)
```

**C++**
```cpp
#include <atomic>

std::atomic<int> counter{0};

// Different memory orderings
counter.fetch_add(1, std::memory_order_relaxed);
counter.store(42, std::memory_order_release);
int val = counter.load(std::memory_order_acquire);

// Sequential consistency (default)
counter++;
val = counter.load();
```

## **Core Atomic Operations**

### **1. Load and Store**

**Purpose**: Read or write a value atomically

**Rust**
```rust
let atomic_val = AtomicUsize::new(0);

// Store
atomic_val.store(100, Ordering::SeqCst);

// Load
let current = atomic_val.load(Ordering::SeqCst);
```

**Go**
```go
var value int64

// Store
atomic.StoreInt64(&value, 100)

// Load  
current := atomic.LoadInt64(&value)
```

**C++**
```cpp
std::atomic<int> value{0};

// Store
value.store(100);

// Load
int current = value.load();
```

### **2. Compare-and-Swap (CAS)**

**Purpose**: Conditionally update a value if it matches expected value

**Rust**
```rust
let atomic = AtomicI32::new(5);

// compare_exchange(expected, new, success_order, failure_order)
match atomic.compare_exchange(5, 10, Ordering::SeqCst, Ordering::Relaxed) {
    Ok(old) => println!("Swapped {} -> 10", old),
    Err(actual) => println!("Failed, actual value: {}", actual),
}

// Weak version (can spuriously fail)
atomic.compare_exchange_weak(10, 15, Ordering::SeqCst, Ordering::Relaxed);
```

**Go**
```go
var value int64 = 5

// Returns true if swap succeeded
swapped := atomic.CompareAndSwapInt64(&value, 5, 10)
```

**C++**
```cpp
std::atomic<int> value{5};

// Strong version
int expected = 5;
bool success = value.compare_exchange_strong(expected, 10);

// Weak version (can fail spuriously, but faster in loops)
expected = 10;
success = value.compare_exchange_weak(expected, 15);
```

### **3. Fetch-and-Modify Operations**

**Purpose**: Atomically modify and return previous value

**Rust**
```rust
let counter = AtomicI32::new(10);

let old_add = counter.fetch_add(5, Ordering::SeqCst);     // Returns 10, counter = 15
let old_sub = counter.fetch_sub(3, Ordering::SeqCst);     // Returns 15, counter = 12
let old_and = counter.fetch_and(0xFF, Ordering::SeqCst);  // Bitwise AND
let old_or = counter.fetch_or(0x01, Ordering::SeqCst);    // Bitwise OR
let old_xor = counter.fetch_xor(0x0F, Ordering::SeqCst);  // Bitwise XOR
```

**Go**
```go
var counter int64 = 10

oldAdd := atomic.AddInt64(&counter, 5)    // Returns 15 (new value)
oldSwap := atomic.SwapInt64(&counter, 20) // Returns 15, counter = 20

// For subtract, use negative add
atomic.AddInt64(&counter, -3)
```

**C++**
```cpp
std::atomic<int> counter{10};

int old_add = counter.fetch_add(5);   // Returns 10, counter = 15
int old_sub = counter.fetch_sub(3);   // Returns 15, counter = 12
int old_and = counter.fetch_and(0xFF); // Bitwise operations
int old_or = counter.fetch_or(0x01);
int old_xor = counter.fetch_xor(0x0F);

// C++20 additions
counter.fetch_max(20); // Atomic max
counter.fetch_min(5);  // Atomic min
```

## **Advanced Patterns and Use Cases**

### **1. Lock-Free Stack**

**C++**
```cpp
template<typename T>
class LockFreeStack {
    struct Node {
        T data;
        std::atomic<Node*> next;
        Node(T value) : data(std::move(value)), next(nullptr) {}
    };
    
    std::atomic<Node*> head{nullptr};
    
public:
    void push(T value) {
        Node* new_node = new Node(std::move(value));
        Node* old_head = head.load(std::memory_order_relaxed);
        
        do {
            new_node->next.store(old_head, std::memory_order_relaxed);
        } while (!head.compare_exchange_weak(old_head, new_node,
                                           std::memory_order_release,
                                           std::memory_order_relaxed));
    }
    
    std::optional<T> pop() {
        Node* old_head = head.load(std::memory_order_relaxed);
        
        do {
            if (!old_head) return std::nullopt;
        } while (!head.compare_exchange_weak(old_head, 
                                           old_head->next.load(),
                                           std::memory_order_acquire,
                                           std::memory_order_relaxed));
        
        T value = std::move(old_head->data);
        delete old_head;
        return value;
    }
};
```

### **2. Spinlock Implementation**

**Rust**
```rust
use std::sync::atomic::{AtomicBool, Ordering};
use std::hint;

pub struct SpinLock {
    locked: AtomicBool,
}

impl SpinLock {
    pub fn new() -> Self {
        SpinLock { locked: AtomicBool::new(false) }
    }
    
    pub fn lock(&self) {
        while self.locked.compare_exchange_weak(
            false,
            true,
            Ordering::Acquire,
            Ordering::Relaxed
        ).is_err() {
            // Spin with exponential backoff
            while self.locked.load(Ordering::Relaxed) {
                hint::spin_loop();
            }
        }
    }
    
    pub fn unlock(&self) {
        self.locked.store(false, Ordering::Release);
    }
}
```

### **3. Sequence Lock (SeqLock)**

**C++**
```cpp
class SeqLock {
    std::atomic<uint64_t> sequence{0};
    
public:
    void write_lock() {
        uint64_t seq = sequence.load(std::memory_order_relaxed);
        while (seq & 1 || !sequence.compare_exchange_weak(seq, seq + 1,
                                                         std::memory_order_acquire,
                                                         std::memory_order_relaxed)) {
            seq = sequence.load(std::memory_order_relaxed);
        }
    }
    
    void write_unlock() {
        sequence.fetch_add(1, std::memory_order_release);
    }
    
    template<typename F>
    auto read(F&& func) {
        uint64_t seq1, seq2;
        decltype(func()) result;
        
        do {
            seq1 = sequence.load(std::memory_order_acquire);
            while (seq1 & 1) {
                std::this_thread::yield();
                seq1 = sequence.load(std::memory_order_acquire);
            }
            
            result = func();
            
            seq2 = sequence.load(std::memory_order_acquire);
        } while (seq1 != seq2);
        
        return result;
    }
};
```

### **4. Atomic Reference Counting**

**Rust**
```rust
use std::sync::atomic::{AtomicUsize, Ordering};
use std::ptr::NonNull;

struct AtomicRc<T> {
    ptr: NonNull<RcInner<T>>,
}

struct RcInner<T> {
    value: T,
    strong_count: AtomicUsize,
}

impl<T> AtomicRc<T> {
    fn new(value: T) -> Self {
        let inner = Box::new(RcInner {
            value,
            strong_count: AtomicUsize::new(1),
        });
        
        AtomicRc {
            ptr: NonNull::new(Box::into_raw(inner)).unwrap(),
        }
    }
    
    fn clone(&self) -> Self {
        let inner = unsafe { self.ptr.as_ref() };
        inner.strong_count.fetch_add(1, Ordering::Relaxed);
        AtomicRc { ptr: self.ptr }
    }
}

impl<T> Drop for AtomicRc<T> {
    fn drop(&mut self) {
        let inner = unsafe { self.ptr.as_ref() };
        if inner.strong_count.fetch_sub(1, Ordering::Release) == 1 {
            // Synchronize with all previous decrements
            std::sync::atomic::fence(Ordering::Acquire);
            unsafe { Box::from_raw(self.ptr.as_ptr()); }
        }
    }
}
```

## **Performance Considerations**

### **Hardware Implementation**

| **Operation** | **x86-64** | **ARM** | **Notes** |
|--------------|------------|---------|-----------|
| **Load/Store** | MOV | LDR/STR with barriers | Natural alignment required |
| **CAS** | CMPXCHG | LDXR/STXR pair | ARM uses LL/SC |
| **Fetch-Add** | LOCK ADD | LDADD (ARMv8.1+) | Hardware acceleration |
| **Memory Fence** | MFENCE | DMB | Full barrier |

### **Cache Coherency Impact**

**False Sharing Example**
```cpp
// BAD: False sharing
struct alignas(64) Counters {
    std::atomic<int> counter1;  // Same cache line
    std::atomic<int> counter2;  // Causes contention
};

// GOOD: Prevent false sharing
struct alignas(64) Counter {
    std::atomic<int> value;
    char padding[60];  // Ensure different cache lines
};

Counter counters[2];  // Each on separate cache line
```

## **Common Pitfalls and Solutions**

### **1. ABA Problem**

**Problem**: Value changes from A→B→A between reads

**Solution using Tagged Pointers**:
```cpp
struct TaggedPtr {
    void* ptr;
    uint32_t tag;
};

std::atomic<TaggedPtr> tagged_head;

// CAS with version tag prevents ABA
TaggedPtr old_head = tagged_head.load();
TaggedPtr new_head = {new_ptr, old_head.tag + 1};
tagged_head.compare_exchange_strong(old_head, new_head);
```

### **2. Memory Ordering Mistakes**

**Wrong**:
```rust
// Using Relaxed for synchronization
data = prepare_data();
flag.store(true, Ordering::Relaxed); // Wrong!

// Consumer might see stale data
if flag.load(Ordering::Relaxed) {
    use_data(data); // Data race!
}
```

**Correct**:
```rust
// Producer uses Release
data = prepare_data();
flag.store(true, Ordering::Release);

// Consumer uses Acquire
if flag.load(Ordering::Acquire) {
    use_data(data); // Safe - synchronized
}
```

### **3. Spurious CAS Failures**

**Handle weak CAS correctly**:
```cpp
// WRONG: Doesn't handle spurious failures
if (atomic_val.compare_exchange_weak(expected, desired)) {
    // May not execute even when it should
}

// CORRECT: Retry on spurious failure
while (!atomic_val.compare_exchange_weak(expected, desired) && 
       atomic_val.load() == expected) {
    // Retry spurious failures
}
```

## **Comparison with Other Synchronization Primitives**

| **Mechanism** | **Performance** | **Complexity** | **Use Case** |
|--------------|----------------|----------------|--------------|
| **Atomics** | Fastest | High | Simple shared state |
| **Spinlock** | Fast (low contention) | Medium | Short critical sections |
| **Mutex** | Slower | Low | General purpose |
| **RwLock** | Medium | Low | Read-heavy workloads |
| **Channel** | Slowest | Lowest | Message passing |

## **Best Practices**

1. **Start with SeqCst**: Optimize to weaker orderings only after profiling
2. **Minimize Contention**: Use sharding or thread-local storage when possible
3. **Avoid Busy-Waiting**: Use exponential backoff or yield in spin loops
4. **Test Thoroughly**: Use thread sanitizers and stress testing
5. **Document Memory Ordering**: Explain why specific orderings are chosen

**Example of Good Practice**:
```rust
/// Counter with relaxed ordering for statistics gathering.
/// Not suitable for synchronization - use SeqCst version for that.
pub struct StatsCounter {
    count: AtomicU64,
}

impl StatsCounter {
    pub fn increment(&self) {
        // Relaxed is fine - we only need eventual consistency
        self.count.fetch_add(1, Ordering::Relaxed);
    }
    
    pub fn get(&self) -> u64 {
        // Relaxed read - may see slightly stale value
        self.count.load(Ordering::Relaxed)
    }
}
```