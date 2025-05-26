

---

## 🔹 Part 1: Foundations of Lock-Free Programming

### What is a Lock-Free Data Structure?

A **lock-free** data structure ensures that *some* thread makes progress in a finite number of steps — even if others are delayed or paused.

#### 🔄 Comparison Table: Blocking vs Non-Blocking

| Aspect                 | Lock-Based            | Lock-Free                       |
| ---------------------- | --------------------- | ------------------------------- |
| Thread Progress        | Can block all threads | Guarantees system-wide progress |
| Deadlock               | Possible              | Impossible                      |
| Starvation             | Possible              | Possible                        |
| Performance under Load | Degrades              | Scales better                   |
| Implementation         | Easier                | Harder                          |

---

## 🔹 Part 2: Core Building Blocks

### 1. **Atomic Operations**

Use CPU-level primitives like `compare_and_swap`, `fetch_add`, etc.

#### In C++

```cpp
#include <atomic>

std::atomic<int> counter{0};

void increment() {
    counter.fetch_add(1, std::memory_order_relaxed);
}
```

#### In Rust

```rust
use std::sync::atomic::{AtomicUsize, Ordering};

static COUNTER: AtomicUsize = AtomicUsize::new(0);

fn increment() {
    COUNTER.fetch_add(1, Ordering::Relaxed);
}
```

#### In Go

```go
import "sync/atomic"

var counter int32

func increment() {
    atomic.AddInt32(&counter, 1)
}
```

### 2. **CAS (Compare-And-Swap)**

#### Rust

```rust
let mut old = 5;
let atomic = AtomicUsize::new(old);

match atomic.compare_exchange(old, 6, Ordering::SeqCst, Ordering::Relaxed) {
    Ok(_) => println!("Success!"),
    Err(current) => println!("Failed, current value is {}", current),
}
```

---

## 🔹 Part 3: Single-Producer Single-Consumer (SPSC) Queue

### C++ Lock-Free SPSC Queue

```cpp
template<typename T, size_t Size>
class SPSCQueue {
    std::atomic<size_t> head{0};
    std::atomic<size_t> tail{0};
    T buffer[Size];

public:
    bool enqueue(T value) {
        size_t t = tail.load(std::memory_order_relaxed);
        size_t next = (t + 1) % Size;
        if (next == head.load(std::memory_order_acquire))
            return false; // Full
        buffer[t] = value;
        tail.store(next, std::memory_order_release);
        return true;
    }

    bool dequeue(T& value) {
        size_t h = head.load(std::memory_order_relaxed);
        if (h == tail.load(std::memory_order_acquire))
            return false; // Empty
        value = buffer[h];
        head.store((h + 1) % Size, std::memory_order_release);
        return true;
    }
};
```

---

## 🔹 Part 4: Treiber's Stack (LIFO Lock-Free Stack)

A classic lock-free stack using CAS on head pointer.

### Rust Example

```rust
use std::sync::atomic::{AtomicPtr, Ordering};
use std::ptr;

struct Node<T> {
    value: T,
    next: *mut Node<T>,
}

pub struct TreiberStack<T> {
    head: AtomicPtr<Node<T>>,
}

impl<T> TreiberStack<T> {
    pub fn new() -> Self {
        TreiberStack { head: AtomicPtr::new(ptr::null_mut()) }
    }

    pub fn push(&self, value: T) {
        let node = Box::into_raw(Box::new(Node {
            value,
            next: self.head.load(Ordering::Relaxed),
        }));

        while self.head
            .compare_exchange_weak(unsafe { (*node).next }, node, Ordering::SeqCst, Ordering::Relaxed)
            .is_err()
        {}
    }

    pub fn pop(&self) -> Option<T> {
        loop {
            let head = self.head.load(Ordering::Acquire);
            if head.is_null() {
                return None;
            }
            let next = unsafe { (*head).next };
            if self.head.compare_exchange(head, next, Ordering::SeqCst, Ordering::Relaxed).is_ok() {
                let value = unsafe { Box::from_raw(head).value };
                return Some(value);
            }
        }
    }
}
```

---

## 🔹 Part 5: Michael & Scott's Queue (Multi-producer Multi-consumer)

Uses atomic pointers and CAS on both head and tail.

### Core Ideas:

* Dummy node to start.
* Use `compare_and_swap` on head and tail.
* Must use `memory_order_acquire/release`.

I'll walk through a Rust implementation in the next part.

---

## 🔹 Part 6: ABA Problem & Memory Reclamation

### ABA Problem

Occurs when value A is replaced with B and back to A, making CAS think nothing changed.

#### Solutions:

* Tag pointers (atomic stamped reference)
* Hazard pointers
* Epoch-based reclamation (used in Crossbeam’s Rust lib)

---

## 🔹 Part 7: Comparison of Lock-Free vs Wait-Free

| Property     | Lock-Free                   | Wait-Free                              |
| ------------ | --------------------------- | -------------------------------------- |
| Guarantee    | Some thread always proceeds | Every thread completes in finite steps |
| Complexity   | High                        | Extremely High                         |
| Common Usage | Stack, Queue                | Real-time systems, kernel work         |
| Performance  | High                        | Lower due to overhead                  |

---

## 🔹 Part 8: Libraries & Tools

* **Rust**

  * `crossbeam`: epoch-based GC, lock-free stack/queue
  * `arc-swap`: atomic Arc swapping
* **C++**

  * `libcds`: concurrent data structures
  * `folly`: Facebook’s concurrent lib
* **Go**

  * `atomic.Value` and sync/atomic pkg
  * Use channels when abstraction over lock-free logic is needed

---
