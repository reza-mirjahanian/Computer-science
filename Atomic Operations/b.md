## 🧠 Atomic Operations in Concurrency — From Foundational to Expert

---

### **1. What Are Atomic Operations?**

**Atomic operations** are low-level operations that are **indivisible**, meaning:

* They either **complete fully** or **don’t happen at all**
* **No other thread can observe a partial result**

This is **crucial in concurrent programs**, where multiple threads or processes access and modify shared data.

---

### **2. Why Are Atomic Operations Needed?**

Without atomicity:

* **Race conditions** occur
* **Inconsistent state** can be observed
* **Undefined behavior** can manifest

#### ✅ Use-Cases:

* Counters shared across threads
* Flags for coordination
* Lock-free data structures

---

### **3. Common Atomic Operations**

| Operation          | Meaning                                                   |
| ------------------ | --------------------------------------------------------- |
| `load`             | Atomically read a value                                   |
| `store`            | Atomically write a value                                  |
| `swap`             | Atomically swap a value                                   |
| `compare_exchange` | Compare and conditionally update (CAS - Compare-And-Swap) |
| `fetch_add`        | Atomically add a value and return previous                |
| `fetch_sub`        | Atomically subtract and return previous                   |

---

## **4. Language-Specific Foundations**

---

### **🔶 Rust: `std::sync::atomic`**

#### Example: Atomic Counter

```rust
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;

static GLOBAL_COUNTER: AtomicUsize = AtomicUsize::new(0);

fn main() {
    let mut handles = vec![];

    for _ in 0..10 {
        handles.push(thread::spawn(|| {
            for _ in 0..1000 {
                GLOBAL_COUNTER.fetch_add(1, Ordering::SeqCst);
            }
        }));
    }

    for h in handles {
        h.join().unwrap();
    }

    println!("Final count: {}", GLOBAL_COUNTER.load(Ordering::SeqCst));
}
```

#### Orderings:

| Ordering  | Guarantees                 |
| --------- | -------------------------- |
| `Relaxed` | No sync                    |
| `Acquire` | Reads before future ops    |
| `Release` | Writes before previous ops |
| `AcqRel`  | Acquire + Release          |
| `SeqCst`  | Total order (most strict)  |

---

### **🔷 Go: `sync/atomic`**

#### Example: Atomic Counter

```go
package main

import (
	"fmt"
	"sync"
	"sync/atomic"
)

var counter int64

func main() {
	var wg sync.WaitGroup

	for i := 0; i < 10; i++ {
		wg.Add(1)

		go func() {
			defer wg.Done()
			for j := 0; j < 1000; j++ {
				atomic.AddInt64(&counter, 1)
			}
		}()
	}

	wg.Wait()
	fmt.Println("Final count:", atomic.LoadInt64(&counter))
}
```

#### Go atomic ops:

* `LoadInt64`, `StoreInt64`
* `AddInt64`, `CompareAndSwapInt64`
* Only supports **limited types** (`int32`, `int64`, `uint32`, etc.)

---

### **🔷 C++: `<atomic>`**

#### Example: Atomic Counter

```cpp
#include <iostream>
#include <atomic>
#include <thread>
#include <vector>

std::atomic<int> counter(0);

void increment() {
    for (int i = 0; i < 1000; ++i) {
        counter.fetch_add(1, std::memory_order_seq_cst);
    }
}

int main() {
    std::vector<std::thread> threads;

    for (int i = 0; i < 10; ++i)
        threads.emplace_back(increment);

    for (auto& t : threads)
        t.join();

    std::cout << "Final count: " << counter.load() << "\n";
}
```

#### Memory Orders:

| C++ Memory Order       | Similar To Rust |
| ---------------------- | --------------- |
| `memory_order_relaxed` | `Relaxed`       |
| `memory_order_acquire` | `Acquire`       |
| `memory_order_release` | `Release`       |
| `memory_order_acq_rel` | `AcqRel`        |
| `memory_order_seq_cst` | `SeqCst`        |

---

## **5. Compare-And-Swap (CAS)**

### ➕ Used for building **lock-free algorithms**

#### Rust Example:

```rust
let mut old = atomic_var.load(Ordering::SeqCst);
loop {
    match atomic_var.compare_exchange(
        old, 
        old + 1, 
        Ordering::SeqCst, 
        Ordering::SeqCst
    ) {
        Ok(_) => break,
        Err(x) => old = x,
    }
}
```

#### Go Example:

```go
for {
    old := atomic.LoadInt64(&val)
    if atomic.CompareAndSwapInt64(&val, old, old+1) {
        break
    }
}
```

#### C++ Example:

```cpp
int old = value.load();
while (!value.compare_exchange_strong(old, old + 1)) {
    // retry with updated `old`
}
```

---

## **6. Edge Cases and Gotchas**

* **Spin loops** (busy-waiting) using atomics can waste CPU:

  * Use `std::thread::yield_now()` in Rust or `std::this_thread::yield()` in C++
* Incorrect ordering can cause **subtle visibility bugs**
* Atomics can be **faster than locks**, but only for **simple operations**

---

## **7. Atomics vs Mutex**

| Feature       | Atomic                   | Mutex                   |
| ------------- | ------------------------ | ----------------------- |
| Overhead      | Low                      | Higher                  |
| Use Case      | Simple values (counters) | Complex data structures |
| Deadlock Risk | ❌ No                     | ✅ Yes                   |
| Blocking      | ❌ Non-blocking           | ✅ Blocking              |
| Composition   | ❌ Difficult              | ✅ Easy                  |

---

## **8. Advanced: Building a Spinlock with Atomics (Rust)**

```rust
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;

struct SpinLock {
    lock: AtomicBool,
}

impl SpinLock {
    fn new() -> Self {
        SpinLock {
            lock: AtomicBool::new(false),
        }
    }

    fn lock(&self) {
        while self.lock.compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed).is_err() {
            thread::yield_now(); // prevent busy loop
        }
    }

    fn unlock(&self) {
        self.lock.store(false, Ordering::Release);
    }
}
```

---

## **9. Tools to Visualize and Verify Atomics**

* **Rust**: `loom` crate for testing concurrency edge cases
* **Go**: `-race` flag detects data races
* **C++**: Use ThreadSanitizer (`-fsanitize=thread` with Clang)

---

## **10. Summary Table of Atomic Ops**

| Operation       | Rust                     | Go                    | C++                           |
| --------------- | ------------------------ | --------------------- | ----------------------------- |
| Atomic Counter  | `AtomicUsize::fetch_add` | `atomic.AddInt64`     | `std::atomic<int>::fetch_add` |
| CAS             | `compare_exchange`       | `CompareAndSwapInt64` | `compare_exchange_strong`     |
| Memory Ordering | `Ordering::*`            | **Not user-exposed**  | `memory_order_*`              |

---


