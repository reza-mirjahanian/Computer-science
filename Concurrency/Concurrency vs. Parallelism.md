

---

## 🔰 FOUNDATION: Understanding the Concepts

| Concept        | Concurrency                                   | Parallelism                             |
| -------------- | --------------------------------------------- | --------------------------------------- |
| **Definition** | Handling multiple tasks at once (interleaved) | Executing multiple tasks simultaneously |
| **Goal**       | Better responsiveness, structure              | Better throughput, speed                |
| **Requires**   | Task management                               | Multiple cores/processors               |
| **Looks like** | Juggling many balls                           | Many jugglers juggling simultaneously   |

* **Concurrency** ≠ **Parallelism**
* Concurrency *can* happen on a single core (via time slicing).
* Parallelism *requires* multiple cores.

---

## 🧠 MENTAL MODEL

* **Concurrency** is about *structure*, *task composition*, and *non-blocking behavior*.
* **Parallelism** is about *speed* and *doing many things at the same time*.

---

## 🛠 EXAMPLES: Basic Programs

---

### ⚙️ Go: Concurrency (Goroutines)

```go
package main

import (
	"fmt"
	"time"
)

func say(msg string) {
	for i := 0; i < 3; i++ {
		fmt.Println(msg)
		time.Sleep(time.Millisecond * 100)
	}
}

func main() {
	go say("world") // concurrent
	say("hello")    // main thread
}
```

* `go say(...)` launches a new **goroutine**: concurrent, not guaranteed to be parallel.

---

### ⚙️ Go: Parallelism via GOMAXPROCS

```go
package main

import (
	"fmt"
	"runtime"
	"time"
)

func task(id int) {
	start := time.Now()
	for time.Since(start) < time.Second {
	}
	fmt.Printf("Task %d done\n", id)
}

func main() {
	runtime.GOMAXPROCS(4) // allow 4 threads in parallel
	for i := 0; i < 4; i++ {
		go task(i)
	}
	time.Sleep(2 * time.Second)
}
```

---

### 🦀 Rust: Concurrency via `std::thread`

```rust
use std::thread;
use std::time::Duration;

fn main() {
    thread::spawn(|| {
        for i in 1..5 {
            println!("Hello from spawned thread: {}", i);
            thread::sleep(Duration::from_millis(100));
        }
    });

    for i in 1..5 {
        println!("Hello from main thread: {}", i);
        thread::sleep(Duration::from_millis(100));
    }
}
```

---

### 🦀 Rust: Parallelism with `rayon`

```rust
use rayon::prelude::*;

fn main() {
    let v: Vec<i32> = (1..1000000).collect();
    let sum: i32 = v.par_iter().sum(); // parallel iteration
    println!("Sum: {}", sum);
}
```

* Rayon abstracts away thread-pooling for parallel computation.

---

### ⚙️ C++: Concurrency via `std::thread`

```cpp
#include <iostream>
#include <thread>
#include <chrono>

void task(const std::string& name) {
    for (int i = 0; i < 5; ++i) {
        std::cout << name << " " << i << std::endl;
        std::this_thread::sleep_for(std::chrono::milliseconds(100));
    }
}

int main() {
    std::thread t1(task, "Thread 1");
    task("Main thread");
    t1.join();
}
```

---

### ⚙️ C++: Parallelism via OpenMP

```cpp
#include <iostream>
#include <omp.h>

int main() {
    #pragma omp parallel for
    for (int i = 0; i < 8; i++) {
        std::cout << "Thread " << omp_get_thread_num() << " processing " << i << std::endl;
    }
}
```

---

## 🔍 Differences in Behavior

| Feature            | Concurrency                         | Parallelism                       |
| ------------------ | ----------------------------------- | --------------------------------- |
| Core Usage         | One or more                         | Requires multiple                 |
| Threading          | Often with fewer threads than tasks | Often maps 1:1 threads to tasks   |
| Speedup?           | Not necessarily                     | Yes                               |
| Real-time handling | Great (UI, servers)                 | Not optimal                       |
| Use cases          | Web servers, async IO, schedulers   | Video encoding, matrix operations |

---

## 🧪 Edge Cases & Gotchas

1. **Rust**: Thread panics don’t crash the main thread.

   ```rust
   let handle = std::thread::spawn(|| panic!("Oops"));
   handle.join().unwrap_or_else(|_| println!("Recovered"));
   ```

2. **Go**: Goroutines leak if not carefully handled.

   ```go
   // Common leak pattern:
   func leaky() chan int {
       ch := make(chan int)
       go func() {
           // Blocks forever
           ch <- 42
       }()
       return ch
   }
   ```

3. **C++**: `std::async` may not run in parallel depending on policy:

   ```cpp
   auto future = std::async(std::launch::deferred, [] { return 42; });
   // Runs only when .get() is called
   ```

---

## 🧰 Best Use by Scenario

| Use Case                         | Use Concurrency | Use Parallelism |
| -------------------------------- | --------------- | --------------- |
| Web server                       | ✅               | ❌               |
| CPU-intensive computation        | ❌               | ✅               |
| IO-bound with many connections   | ✅               | ❌               |
| Large data processing (e.g. map) | ❌               | ✅               |

---

## 🧵 Advanced: Hybrid Use

You can mix both:

* **Web server (concurrent)** receives requests.
* **Worker pool (parallel)** handles CPU-heavy processing.

**Rust example:**

```rust
use rayon::prelude::*;
use std::thread;

fn main() {
    let data = (1..100).collect::<Vec<_>>();

    thread::spawn(move || {
        let processed: Vec<_> = data.par_iter().map(|x| x * 2).collect();
        println!("Processed: {:?}", &processed[..5]);
    }).join().unwrap();
}
```

---

## 🏁 Summary

| Aspect            | Concurrency                          | Parallelism                             |
| ----------------- | ------------------------------------ | --------------------------------------- |
| Focus             | *Decomposition of tasks*             | *Execution of tasks*                    |
| Core need         | 1+ cores, not essential              | Requires multi-core                     |
| Outcome           | Responsiveness, good structure       | Speedup, efficiency                     |
| Tools             | Threads, goroutines, `async`/`await` | `rayon`, OpenMP, parallel iterators     |
| Typical Languages | Go, Rust, Erlang                     | C++, Rust, Python (with native modules) |


