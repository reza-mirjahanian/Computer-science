
## Concurrency Interview Questions and Answers

---

### Fundamentals of Concurrency

1.  **What is concurrency?**
    **Answer:** Concurrency is the ability of different parts or units of a program, algorithm, or problem to be executed out-of-order or in partial order, without affecting the final outcome. It's about dealing with multiple things at once.

2.  **What is parallelism?**
    **Answer:** Parallelism is the ability of a system to execute multiple computations simultaneously, typically by using multiple processor cores. It's about doing multiple things at once.

3.  **What is the difference between concurrency and parallelism?**
    **Answer:** **Concurrency** is about managing multiple tasks at the same time (can be on a single core via context switching). **Parallelism** is about executing multiple tasks at the same time (requires multiple cores). Concurrency can exist without parallelism.

4.  **What is a process?**
    **Answer:** A **process** is an instance of a computer program that is being executed. It has its own independent memory space.

5.  **What is a thread?**
    **Answer:** A **thread** is the smallest unit of execution within a process. Multiple threads can exist within a single process and share its memory space.

6.  **Why use concurrency?**
    **Answer:** To improve **responsiveness** (e.g., UI remains active during background tasks), **throughput** (handling more tasks in a given time), and **resource utilization** (e.g., using multiple CPU cores effectively).

7.  **What are the main challenges of concurrent programming?**
    **Answer:** **Race conditions**, **deadlocks**, **livelocks**, **starvation**, **complexity** of managing shared resources, and **debugging difficulties**.

8.  **What is context switching?**
    **Answer:** **Context switching** is the process of storing the state of a process or thread so that it can be restored and resume execution at a later point. This allows multiple processes or threads to share a single CPU.

9.  **What is multitasking?**
    **Answer:** **Multitasking** is the ability of an operating system to execute more than one task (program) seemingly simultaneously. It's typically achieved through context switching.

10. **What is preemptive multitasking?**
    **Answer:** In **preemptive multitasking**, the operating system's scheduler can interrupt a running task to give CPU time to another task, based on priority or time slicing.

11. **What is cooperative multitasking?**
    **Answer:** In **cooperative multitasking**, tasks voluntarily cede control of the CPU to other tasks. If a task doesn't yield, it can monopolize the CPU.

---

### Synchronization Primitives

12. **What are synchronization primitives?**
    **Answer:** **Synchronization primitives** are low-level mechanisms provided by an operating system or programming language to coordinate the execution of concurrent threads and manage access to shared resources.

13. **What is a mutex (Mutual Exclusion)?**
    **Answer:** A **mutex** is a synchronization primitive that grants exclusive access to a shared resource to only one thread at a time. A thread "locks" the mutex to access the resource and "unlocks" it when done.

14. **Explain how a mutex works.**
    **Answer:** A thread wanting to access a resource first tries to lock the associated mutex. If the mutex is already locked by another thread, the current thread blocks (waits) until the mutex is unlocked.

15. **What is a semaphore?**
    **Answer:** A **semaphore** is a synchronization primitive that controls access to a shared resource by maintaining a counter. It allows a certain number of threads to access the resource concurrently.

16. **What are the two main operations of a semaphore?**
    **Answer:** **`wait()`** (or `P()`, `acquire()`): Decrements the semaphore count. If the count becomes negative, the thread blocks.
    **`signal()`** (or `V()`, `release()`): Increments the semaphore count. If threads are blocked, one is unblocked.

17. **What is the difference between a binary semaphore and a mutex?**
    **Answer:** A **binary semaphore** (value 0 or 1) can be used like a mutex. However, a crucial difference is that a mutex can typically only be unlocked by the thread that locked it, while a semaphore's `signal()` operation can be called by any thread.

18. **What is a condition variable?**
    **Answer:** A **condition variable** is a synchronization primitive that allows threads to wait (sleep) until a certain condition becomes true. It's always used in conjunction with a mutex.

19. **How do condition variables work with mutexes?**
    **Answer:** A thread locks a mutex, checks a condition. If false, it calls `wait()` on the condition variable, which atomically releases the mutex and puts the thread to sleep. Another thread, after changing the condition and holding the same mutex, calls `signal()` or `broadcast()` to wake one or all waiting threads.

20. **What is the difference between `signal()` and `broadcast()` on a condition variable?**
    **Answer:** **`signal()`** (or `notify_one()`) wakes up *at least one* thread waiting on the condition variable. **`broadcast()`** (or `notify_all()`) wakes up *all* threads waiting on the condition variable.

21. **What is a Reader-Writer Lock?**
    **Answer:** A **Reader-Writer Lock** allows multiple threads to read a shared resource concurrently but requires exclusive access for writing. This improves performance when reads are much more frequent than writes.

22. **What is a spinlock?**
    **Answer:** A **spinlock** is a type of lock where a thread trying to acquire it repeatedly checks if the lock is available (i.e., "spins") rather than blocking. Useful for short critical sections on multi-core systems.

23. **When would you use a spinlock over a mutex?**
    **Answer:** When the expected wait time for the lock is very short, less than the time it would take to perform two context switches (one to block, one to unblock). Primarily used in low-level kernel programming.

24. **What is a barrier?**
    **Answer:** A **barrier** is a synchronization point where multiple threads must wait until all participating threads have reached it before any can proceed.

---

### Concurrency Models

25. **What are some common concurrency models?**
    **Answer:** **Shared Memory Model** (threads communicate via shared data structures) and **Message Passing Model** (threads communicate by sending messages, e.g., Actor model, CSP).

26. **Explain the Shared Memory Model.**
    **Answer:** In the **Shared Memory Model**, concurrent modules (threads or processes) interact by reading and writing shared objects in memory. Requires explicit synchronization (mutexes, semaphores) to prevent data corruption.

27. **Explain the Message Passing Model.**
    **Answer:** In the **Message Passing Model**, concurrent modules interact by sending messages to each other over communication channels. State is typically not shared directly. Examples: Go channels, Erlang actors.

28. **What is the Actor Model?**
    **Answer:** The **Actor Model** is a conceptual model of concurrent computation where "actors" are the universal primitives. An actor can:
    * Send messages to other actors.
    * Create new actors.
    * Designate behavior for the next message it receives.
    Actors have private state and communicate solely via asynchronous messages.

29. **What is Communicating Sequential Processes (CSP)?**
    **Answer:** **CSP** is a formal language for describing patterns of interaction in concurrent systems. It emphasizes channels as a means of communication between processes. Go's concurrency model is heavily influenced by CSP.

30. **What are threads pools?**
    **Answer:** A **thread pool** is a collection of pre-created worker threads that stand ready to execute tasks. This avoids the overhead of creating a new thread for each task.

31. **What is fork-join parallelism?**
    **Answer:** The **fork-join model** is a way of setting up and executing parallel programs where a main process (or thread) "forks" off a number of parallel tasks and then "joins" them, waiting for them to complete before continuing.

---

### Race Conditions and Data Races

32. **What is a race condition?**
    **Answer:** A **race condition** is a situation where the behavior of a system depends on the unpredictable timing or interleaving of operations by multiple threads or processes. The outcome is non-deterministic and often incorrect.

33. **What is a data race?**
    **Answer:** A **data race** occurs when:
    * Two or more threads concurrently access the same memory location.
    * At least one of these accesses is a write.
    * The accesses are not synchronized.
    Data races can lead to undefined behavior.

34. **Give an example of a race condition.**
    **Answer:** Two threads incrementing a shared counter without synchronization:
    Thread A reads counter (e.g., 5).
    Thread B reads counter (e.g., 5).
    Thread A increments its local value to 6 and writes it back.
    Thread B increments its local value to 6 and writes it back.
    Expected result: 7. Actual result: 6.

    ```cpp
    // C++ Example (Race Condition)
    #include <iostream>
    #include <thread>
    #include <vector>

    int shared_counter = 0;

    void increment_counter() {
        for (int i = 0; i < 100000; ++i) {
            shared_counter++; // Potential race condition
        }
    }

    int main() {
        std::thread t1(increment_counter);
        std::thread t2(increment_counter);
        t1.join();
        t2.join();
        std::cout << "Counter: " << shared_counter << std::endl; // Value is often not 200000
        return 0;
    }
    ```

35. **How can you prevent race conditions?**
    **Answer:** By using **synchronization primitives** like mutexes, semaphores, or atomic operations to control access to shared resources, or by designing code to avoid shared mutable state (e.g., message passing).

36. **What is a critical section?**
    **Answer:** A **critical section** is a piece of code that accesses a shared resource and must not be concurrently executed by more than one thread. Mutexes are used to protect critical sections.

37. **What does "thread-safe" mean?**
    **Answer:** **Thread-safe** code can be called from multiple threads concurrently without causing race conditions or incorrect behavior. This usually implies proper synchronization or immutability.

---

### Deadlocks and Livelocks

38. **What is a deadlock?**
    **Answer:** A **deadlock** is a state in which two or more threads are blocked indefinitely, each waiting for a resource held by another thread in the same set.

39. **What are the four necessary conditions for deadlock (Coffman conditions)?**
    **Answer:**
    1.  **Mutual Exclusion:** Resources are non-shareable (only one process can use at a time).
    2.  **Hold and Wait:** A process holds at least one resource and requests additional resources held by other processes.
    3.  **No Preemption:** Resources cannot be forcibly taken from a process; they must be released voluntarily.
    4.  **Circular Wait:** A set of processes {P0, P1, ..., Pn} exists such that P0 is waiting for a resource held by P1, P1 for P2, ..., Pn for P0.

40. **How can you prevent deadlocks?**
    **Answer:** By breaking one of the Coffman conditions:
    * Avoid circular wait (e.g., establish a lock ordering).
    * Release held resources before requesting new ones (break hold and wait).
    * Use lock timeouts.
    * Use algorithms that detect and recover from deadlocks.

41. **What is lock ordering?**
    **Answer:** **Lock ordering** is a strategy to prevent deadlocks by ensuring that all threads acquire locks in a predefined, consistent order. This prevents circular wait conditions.

42. **What is a livelock?**
    **Answer:** A **livelock** occurs when threads are actively executing but are not making progress because they are continuously changing their state in response to each other's actions, without doing any useful work. They are not blocked, but still stuck.

43. **Give an example of a livelock.**
    **Answer:** Two people trying to pass each other in a narrow hallway. Each politely steps aside, but they both step in the same direction, and repeat the process, never actually passing.

44. **What is starvation?**
    **Answer:** **Starvation** (or indefinite postponement) occurs when a thread is perpetually denied access to a resource it needs to make progress, often because other threads (e.g., higher priority ones) are constantly being favored.

---

### Memory Models and Atomicity

45. **What is a memory model in concurrency?**
    **Answer:** A **memory model** defines how threads interact through memory. It specifies the guarantees the system provides about the visibility and ordering of memory operations performed by different threads.

46. **What is sequential consistency?**
    **Answer:** **Sequential consistency** is a strong memory model where the result of any execution is the same as if all operations by all processors were executed in some sequential order, and the operations of each individual processor appear in this sequence in the order specified by its program.

47. **What is relaxed consistency?**
    **Answer:** **Relaxed consistency** models offer weaker guarantees than sequential consistency, allowing for more compiler and hardware optimizations, but requiring programmers to use explicit synchronization (e.g., memory fences/barriers) to ensure correct ordering and visibility when needed.

48. **What is an atomic operation?**
    **Answer:** An **atomic operation** is an operation that appears to occur instantaneously from the perspective of other threads. It is indivisible; it either completes fully or not at all, without any intermediate states being visible to other threads.

49. **Why are atomic operations important in concurrency?**
    **Answer:** They allow for lock-free manipulation of shared data in certain cases, often improving performance by avoiding the overhead of mutexes. For example, incrementing a shared counter atomically.

    ```rust
    // Rust Example (Atomic Operation)
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;
    use std::thread;

    fn main() {
        let counter = Arc::new(AtomicUsize::new(0));
        let mut handles = vec![];

        for _ in 0..10 {
            let counter_clone = Arc::clone(&counter);
            handles.push(thread::spawn(move || {
                for _ in 0..10000 {
                    counter_clone.fetch_add(1, Ordering::Relaxed);
                }
            }));
        }

        for handle in handles {
            handle.join().unwrap();
        }
        println!("Counter: {}", counter.load(Ordering::Relaxed)); // Should be 100000
    }
    ```

50. **What is a memory barrier (or fence)?**
    **Answer:** A **memory barrier** is an instruction that enforces an ordering constraint on memory operations. Operations issued before the barrier are guaranteed to be performed before operations issued after the barrier. This helps ensure visibility of changes across threads.

51. **What is "false sharing"?**
    **Answer:** **False sharing** occurs when threads on different processors modify variables that reside on the same cache line. Even if the threads are accessing different variables, the cache coherency protocol may cause the cache line to be invalidated and reloaded frequently, degrading performance.

52. **How can false sharing be mitigated?**
    **Answer:** By padding data structures to ensure that variables frequently accessed by different threads are on different cache lines.

---

### Thread Management

53. **What are user-level threads and kernel-level threads?**
    **Answer:** **User-level threads** are managed by a user-level library without kernel support; context switching is fast. **Kernel-level threads** are managed by the OS; context switching is slower but they can run in true parallel on multi-core systems and don't block the entire process if one thread makes a blocking system call.

54. **What is thread affinity?**
    **Answer:** **Thread affinity** (or CPU pinning) is the practice of binding a thread to a specific CPU core or set of cores. This can improve performance by reducing cache misses and context switching overhead.

55. **How do you create a thread in C++?**
    **Answer:** Using `std::thread`.
    ```cpp
    #include <iostream>
    #include <thread>

    void task() { std::cout << "Hello from thread!" << std::endl; }

    int main() {
        std::thread my_thread(task);
        my_thread.join(); // Wait for the thread to finish
        return 0;
    }
    ```

56. **What does `thread::join()` do?**
    **Answer:** `thread::join()` blocks the calling thread until the thread on which `join()` is called completes its execution.

57. **What does `thread::detach()` do?**
    **Answer:** `thread::detach()` separates the thread object from the actual thread of execution, allowing the thread to run independently in the background. The detached thread is no longer joinable, and the original thread object no longer owns it. Resources are reclaimed when the detached thread finishes.

58. **What is a "daemon" thread?**
    **Answer:** A **daemon thread** (or background thread) is a thread that does not prevent the program from exiting when all non-daemon threads have finished.

---

### Asynchronous Programming

59. **What is asynchronous programming?**
    **Answer:** **Asynchronous programming** is a means of parallel programming in which a unit of work runs separately from the main application thread and notifies the calling thread of its completion, failure, or progress. It avoids blocking the main thread.

60. **What are futures and promises?**
    **Answer:** A **Promise** represents an object that will eventually produce a value. A **Future** is a read-only placeholder for a value that will be available at some point in the future, typically the result of an asynchronous operation associated with a Promise.

61. **How does `async/await` work?**
    **Answer:** **`async`** functions allow the use of **`await`**. When an `async` function encounters an `await` expression, it suspends its execution (without blocking the thread) until the awaited task completes. The underlying system (e.g., an event loop or thread pool) can run other tasks in the meantime.

    ```rust
    // Rust async/await example (simplified)
    async fn fetch_data() -> String {
        // Simulate an async operation
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        "Data fetched".to_string()
    }

    #[tokio::main]
    async fn main() {
        println!("Fetching...");
        let data = fetch_data().await; // Suspends main until fetch_data completes
        println!("{}", data);
    }
    ```

62. **What is an event loop?**
    **Answer:** An **event loop** is a programming construct that waits for and dispatches events or messages in a program. It's central to non-blocking I/O and asynchronous programming, allowing a single thread to handle many concurrent operations.

63. **What are callbacks?**
    **Answer:** A **callback** is a function passed as an argument to another function, which is then invoked ("called back") at a later point, typically when an asynchronous operation completes or an event occurs.

64. **What is "callback hell"?**
    **Answer:** **"Callback hell"** (or "pyramid of doom") refers to deeply nested callbacks, which can make code hard to read, reason about, and maintain. `async/await` and promises help mitigate this.

---

### Language-Specific Concurrency Features

#### Go

65. **What are goroutines?**
    **Answer:** **Goroutines** are lightweight, concurrently executing functions in Go. They are managed by the Go runtime, not directly by the OS threads, making them very cheap to create and manage.

66. **How do you create a goroutine?**
    **Answer:** By using the `go` keyword before a function call.
    ```go
    // Go example
    import (
        "fmt"
        "time"
    )

    func say(s string) {
        for i := 0; i < 3; i++ {
            fmt.Println(s)
            time.Sleep(100 * time.Millisecond)
        }
    }

    func main() {
        go say("world") // Start a new goroutine
        say("hello")
        // Note: main goroutine might exit before "world" goroutine finishes without proper synchronization
    }
    ```

67. **What are Go channels?**
    **Answer:** **Channels** in Go are typed conduits through which you can send and receive values with the `<-` operator. They are a primary way goroutines communicate and synchronize.

68. **What is a buffered channel in Go?**
    **Answer:** A **buffered channel** has a capacity greater than zero. Sends to a buffered channel block only when the buffer is full. Receives block when the buffer is empty. Unbuffered channels (capacity 0) block until both sender and receiver are ready.

69. **What is the `select` statement in Go used for?**
    **Answer:** The `select` statement lets a goroutine wait on multiple communication operations (channel sends or receives). It blocks until one of its cases can run, then it executes that case. If multiple are ready, it chooses one at random.

70. **Explain "Don't communicate by sharing memory; share memory by communicating."**
    **Answer:** This is a Go proverb advocating for the use of channels (communication) to manage shared state, rather than traditional locking mechanisms (shared memory). It promotes clearer and less error-prone concurrent code.

#### Rust

71. **How does Rust ensure thread safety?**
    **Answer:** Rust's ownership and borrowing system, along with the `Send` and `Sync` traits, statically ensures memory safety and helps prevent data races at compile time.
    * **`Send`**: A type `T` is `Send` if it's safe to transfer ownership of `T` to another thread.
    * **`Sync`**: A type `T` is `Sync` if it's safe to share `&T` (an immutable reference) across threads.

72. **What is `Arc<T>` in Rust?**
    **Answer:** **`Arc<T>`** (Atomically Referenced Counter) is a thread-safe reference-counting pointer. It's used to share ownership of a value across multiple threads.

73. **What is `Mutex<T>` in Rust?**
    **Answer:** **`std::sync::Mutex<T>`** provides mutual exclusion for shared data `T`. When the mutex is locked, the calling thread gets a `MutexGuard`, which provides access to the data and automatically unlocks the mutex when it goes out of scope (RAII).

74. **How does Rust prevent data races with its `Mutex`?**
    **Answer:** The `MutexGuard` returned by `lock()` borrows the `Mutex`. The data can only be accessed through this guard. The borrow checker ensures the guard (and thus access to the data) isn't shared in a way that could cause a data race. The `Mutex` itself enforces that only one `MutexGuard` exists at a time.

75. **What are `mpsc` channels in Rust?**
    **Answer:** `std::sync::mpsc` provides **multiple-producer, single-consumer** channels for message passing between threads in Rust.

#### C++

76. **What concurrency features were introduced in C++11?**
    **Answer:** `std::thread`, `std::mutex`, `std::condition_variable`, `std::future`, `std::promise`, `std::async`, `std::atomic`.

77. **What is `std::atomic` in C++?**
    **Answer:** `std::atomic<T>` provides atomic operations for type `T` (e.g., integers, pointers). This allows for lock-free manipulation of shared variables, preventing data races on those specific variables.

78. **What is `std::async` in C++?**
    **Answer:** `std::async` runs a function asynchronously (potentially in a new thread) and returns a `std::future` that will eventually hold the result of the function call. It can choose a launch policy: `std::launch::async` (new thread) or `std::launch::deferred` (lazy evaluation on `get()`).

79. **What is RAII and how does it relate to mutexes in C++?**
    **Answer:** **RAII (Resource Acquisition Is Initialization)** is a C++ programming technique where resource management (acquisition and release) is tied to object lifetime. `std::lock_guard` and `std::unique_lock` are RAII wrappers for mutexes; they lock the mutex in their constructor and unlock it in their destructor, ensuring the mutex is always released, even if exceptions occur.

    ```cpp
    // C++ RAII Mutex Example
    #include <mutex>
    #include <iostream>

    std::mutex mtx;
    int shared_data = 0;

    void safe_increment() {
        std::lock_guard<std::mutex> lock(mtx); // Mutex locked in ctor
        shared_data++;
    } // Mutex unlocked in dtor when 'lock' goes out of scope

    int main() {
        safe_increment();
        std::cout << "Shared data: " << shared_data << std::endl;
        return 0;
    }
    ```

80. **What is `std::packaged_task`?**
    **Answer:** `std::packaged_task` wraps a callable target (function, lambda, bind expression, function object) so that it can be invoked asynchronously. Its result is stored in a `std::future` associated with the `packaged_task`.

---

### Testing and Debugging Concurrent Code

81. **Why is testing concurrent code difficult?**
    **Answer:** Due to **non-determinism**. Bugs like race conditions or deadlocks might only appear under specific, hard-to-reproduce timing conditions.

82. **What are some techniques for testing concurrent applications?**
    **Answer:** **Stress testing** (running with many threads/high load), **injecting delays** to try and expose timing issues, using **thread sanitizers** or **race detectors**, formal verification (less common), and designing for testability (e.g., deterministic components).

83. **What is a thread sanitizer?**
    **Answer:** A **thread sanitizer** (e.g., TSan in Clang/GCC) is a dynamic analysis tool that detects data races and other thread-related errors during program execution.

84. **How can you debug a deadlock?**
    **Answer:** By examining the state of blocked threads (e.g., using a debugger to inspect stacks and held locks), analyzing lock acquisition orders, and logging lock events.

85. **What is "Heisenbug"?**
    **Answer:** A **Heisenbug** is a bug that seems to disappear or alter its behavior when one attempts to study it (e.g., by adding print statements or using a debugger), due to the act of observation changing the timing of events. Common in concurrent systems.

---

### Advanced Concurrency Concepts

86. **What is lock-free programming?**
    **Answer:** **Lock-free programming** allows multiple threads to access shared data without using locks. It relies on atomic operations (like CAS - Compare-And-Swap). Guarantees system-wide progress: at least one thread always makes progress.

87. **What is wait-free programming?**
    **Answer:** **Wait-free programming** is a stronger guarantee than lock-free. Every thread is guaranteed to complete its operation in a finite number of steps, regardless of the actions of other threads. This prevents starvation.

88. **What is a Compare-And-Swap (CAS) operation?**
    **Answer:** **CAS** is an atomic instruction that compares the contents of a memory location to a given value and, only if they are the same, modifies the contents of that memory location to a new given value. It returns the original value or a status indicating success/failure.

89. **What is Software Transactional Memory (STM)?**
    **Answer:** **STM** is a concurrency control mechanism analogous to database transactions for controlling access to shared memory. Operations are grouped into transactions which are either committed (all changes applied atomically) or rolled back. Aims to simplify concurrent programming.

90. **What is work-stealing?**
    **Answer:** **Work-stealing** is a scheduling strategy used in multi-threaded environments. When a thread in a thread pool runs out of tasks, it "steals" tasks from the deque of another busy thread. Common in fork-join frameworks.

---

### Performance Considerations

91. **How can excessive locking impact performance?**
    **Answer:** Excessive locking can lead to **contention** (threads waiting for locks), **reduced parallelism** (parts of the code become serialized), and **overhead** from lock acquisition/release.

92. **What is Amdahl's Law?**
    **Answer:** **Amdahl's Law** describes the theoretical speedup in latency of the execution of a task when resources are improved. It states that the speedup is limited by the fraction of the task that is sequential (cannot be parallelized).
    $S = \frac{1}{(1-P) + \frac{P}{N}}$, where $P$ is the proportion of the program that can be parallelized, and $N$ is the number of processors.

93. **What is Gustafson's Law?**
    **Answer:** **Gustafson's Law** addresses some limitations of Amdahl's Law for problems that scale with available resources. It suggests that as you get more computing power, you tend to tackle larger or more complex problems. The speedup is calculated for a scaled problem size.

94. **What is scalability in the context of concurrency?**
    **Answer:** **Scalability** refers to how well a concurrent system can handle an increasing workload or an increasing number of processors. Ideally, performance increases linearly with added resources.

95. **How can you measure the performance of concurrent code?**
    **Answer:** Using **profilers** to identify bottlenecks, **benchmarking tools** to measure throughput and latency under various loads, and observing CPU utilization and context switching rates.

---

### Common Concurrency Problems

96. **What is "busy-waiting"? Is it always bad?**
    **Answer:** **Busy-waiting** (or spinning) is when a thread repeatedly checks a condition instead of blocking. It consumes CPU cycles. It's generally bad for long waits but can be good for very short waits on multi-core systems (spinlocks) to avoid context switch overhead.

97. **What are priority inversion issues?**
    **Answer:** **Priority inversion** occurs when a lower-priority task holds a resource needed by a higher-priority task, effectively "inverting" their priorities. The high-priority task waits while the low-priority task runs. This can be solved by priority inheritance.

98. **What is "thundering herd" problem?**
    **Answer:** The **thundering herd problem** occurs when many threads are waiting for an event, and when the event occurs, all threads are awakened, but only one (or a few) can proceed. This causes a spike in contention and wasted CPU cycles. Often seen with `notify_all()` or when multiple processes `accept()` on the same listening socket.

---

### Design Patterns for Concurrency

99. **Describe the Producer-Consumer pattern.**
    **Answer:** The **Producer-Consumer** pattern involves one or more **producers** generating data and putting it into a shared buffer (queue), and one or more **consumers** taking data from the buffer and processing it. This decouples producers from consumers and helps smooth out workloads. Requires synchronization for the shared buffer.

100. **Describe the Pipeline pattern.**
     **Answer:** The **Pipeline** pattern consists of a chain of processing elements (stages), arranged so that the output of each element is the input of the next. Each stage can run concurrently. Data flows through the stages like an assembly line. This is useful for stream processing or tasks that can be broken into sequential steps.

---
Here are 100 interview questions and answers on concurrency in programming:

## **Fundamentals of Concurrency**

---

1.  **What is concurrency?**
    * **Answer:** Concurrency is the ability of a system to execute multiple tasks or parts of programs in overlapping time periods. It's about **dealing with** lots of things at once.

2.  **What is a process?**
    * **Answer:** A process is an instance of a computer program that is being executed. It has its **own memory space** and resources.

3.  **What is a thread?**
    * **Answer:** A thread is the smallest unit of execution within a process. Multiple threads can exist within a single process and **share its resources** like memory, but each thread has its own program counter, stack, and local variables.

4.  **What's the difference between a process and a thread?**
    * **Answer:** Processes are **independent** and have separate memory spaces. Threads exist within a process, share memory, and are **lighter weight** to create and manage than processes.

5.  **What are the benefits of using concurrency?**
    * **Answer:** Improved **responsiveness** (UI remains active during background tasks), better **resource utilization** (keeping CPU busy), and increased **throughput** (completing more tasks in a given time).

6.  **What are common challenges in concurrent programming?**
    * **Answer:** **Race conditions**, **deadlocks**, **livelocks**, **starvation**, and ensuring **data consistency**.

7.  **What is context switching?**
    * **Answer:** Context switching is the process of **storing the state** of a process or thread so that it can be restored and resume execution at a later point. This allows multiple processes or threads to share a single CPU.

8.  **Is concurrency only possible on multi-core processors?**
    * **Answer:** No. Concurrency can be achieved on a single-core processor through **time-slicing** (rapidly switching between tasks). Multi-core processors enable **parallelism**, where tasks truly run at the same instant.

## **Parallelism vs. Concurrency**

---

9.  **What is parallelism?**
    * **Answer:** Parallelism is the ability of a system to execute multiple tasks or parts of a program **simultaneously**, typically on a multi-core processor. It's about **doing** lots of things at once.

10. **Explain the difference between concurrency and parallelism with an analogy.**
    * **Answer:** Imagine one person juggling (concurrency – dealing with multiple balls by switching attention) versus two people each juggling their own set of balls (parallelism – doing multiple things at the exact same time).

11. **Can a program be concurrent but not parallel?**
    * **Answer:** Yes. A program on a single-core CPU can be concurrent by switching between tasks, but it cannot be parallel as only one task executes at any given instant.

12. **When is parallelism more beneficial than "just" concurrency?**
    * **Answer:** Parallelism is more beneficial for **CPU-bound tasks** where the goal is to speed up computation by distributing work across multiple cores.

## **Synchronization Primitives**

---

13. **What are synchronization primitives?**
    * **Answer:** Tools provided by an operating system or programming language to **coordinate the execution** of concurrent threads or processes, preventing race conditions and ensuring orderly access to shared resources.

14. **What is a mutex (Mutual Exclusion)?**
    * **Answer:** A mutex is a lock that allows **only one thread** to access a shared resource or critical section of code at a time.
    * **C++ Example:**
        ```cpp
        std::mutex mtx;
        void critical_section() {
            std::lock_guard<std::mutex> lock(mtx); // Lock acquired
            // Access shared data
        } // Lock released automatically
        ```

15. **What is a semaphore?**
    * **Answer:** A semaphore is a signaling mechanism that controls access to a shared resource by maintaining a **counter**. It allows a certain number of threads to access the resource simultaneously.

16. **What's the difference between a binary semaphore and a mutex?**
    * **Answer:** A binary semaphore (value 0 or 1) can function like a mutex. However, a key difference is that a mutex should be **unlocked by the same thread** that locked it, while a semaphore's signal (V operation) can be performed by any thread.

17. **What is a condition variable?**
    * **Answer:** A condition variable allows threads to **wait for a certain condition** to become true. It's typically used in conjunction with a mutex to avoid busy-waiting.
    * **C++ Example:**
        ```cpp
        std::mutex mtx;
        std::condition_variable cv;
        bool ready = false;

        void worker_thread() {
            std::unique_lock<std::mutex> lock(mtx);
            cv.wait(lock, []{ return ready; }); // Wait until ready is true
            // Process data
        }
        ```

18. **What is a Read-Write Lock?**
    * **Answer:** A read-write lock allows **multiple threads to read** a shared resource concurrently but requires **exclusive access for writing**. This can improve performance if reads are much more frequent than writes.

19. **What is a barrier?**
    * **Answer:** A barrier is a synchronization point where multiple threads must **wait until all participating threads** have reached that point before any can proceed.

20. **What is an atomic operation?**
    * **Answer:** An operation that executes as a **single, indivisible unit**, without interference from other threads. It's crucial for lock-free programming.
    * **C++ Example:** `std::atomic<int> counter; counter++;`

21. **When would you use `std::atomic_flag`?**
    * **Answer:** `std::atomic_flag` is the simplest atomic type, often used as a building block for other synchronization primitives or for simple spinlocks because it guarantees being **lock-free**. It has only two states: set and clear.

## **Race Conditions and Data Races**

---

22. **What is a race condition?**
    * **Answer:** A situation where the outcome of a program depends on the **unpredictable sequence or timing** of operations from multiple threads. The behavior becomes non-deterministic.

23. **What is a data race?**
    * **Answer:** A specific type of race condition where:
        1.  Two or more threads concurrently access the **same memory location**.
        2.  At least one of these accesses is a **write**.
        3.  The accesses are **not synchronized** by atomic operations or locks.
    * Data races lead to **undefined behavior** in languages like C++ and Rust.

24. **How can you detect race conditions?**
    * **Answer:** Using **static analysis tools**, **dynamic analysis tools** (like thread sanitizers), code reviews, and careful testing. However, they can be hard to reproduce.

25. **How can you prevent race conditions?**
    * **Answer:** Using **synchronization primitives** (mutexes, semaphores), **atomic operations**, immutable data structures, and careful design of shared resource access.

26. **Give an example of a race condition.**
    * **Answer:** Two threads incrementing a shared counter without synchronization:
        ```cpp
        // Shared counter
        int counter = 0;

        void increment() {
            int temp = counter; // Thread 1 reads counter (e.g., 0)
                               // Context switch to Thread 2
                               // Thread 2 reads counter (e.g., 0)
                               // Thread 2 increments temp (0 -> 1)
                               // Thread 2 writes temp to counter (counter = 1)
                               // Context switch back to Thread 1
            temp++;             // Thread 1 increments its old temp (0 -> 1)
            counter = temp;     // Thread 1 writes temp to counter (counter = 1)
                                // Expected counter = 2, but it's 1
        }
        ```

## **Deadlocks and Livelocks**

---

27. **What is a deadlock?**
    * **Answer:** A situation where two or more threads are **blocked forever**, each waiting for a resource held by another thread in the same set.

28. **What are the four Coffman conditions for deadlock?**
    * **Answer:**
        1.  **Mutual Exclusion:** Resources cannot be shared.
        2.  **Hold and Wait:** A thread holds at least one resource and is waiting to acquire additional resources held by other threads.
        3.  **No Preemption:** Resources cannot be forcibly taken from threads holding them.
        4.  **Circular Wait:** A set of waiting threads {T0, T1, ..., Tn} exists such that T0 is waiting for a resource held by T1, T1 for T2, ..., Tn for T0.

29. **How can you prevent deadlocks?**
    * **Answer:**
        * **Break Mutual Exclusion:** Use shareable resources or optimistic locking (not always feasible).
        * **Break Hold and Wait:** Acquire all necessary resources at once, or release held resources before requesting new ones.
        * **Allow Preemption:** If a thread holding resources requests another that cannot be immediately allocated, it releases all its currently held resources.
        * **Break Circular Wait:** Impose a **total ordering** of all resource types and require threads to request resources in increasing order.

30. **What is deadlock detection?**
    * **Answer:** Allowing deadlocks to occur, then detecting them using algorithms (e.g., resource allocation graphs) and then recovering (e.g., by aborting a process).

31. **What is deadlock avoidance?**
    * **Answer:** Using information about future resource requests to dynamically decide if an allocation is safe, ensuring the system never enters a deadlock state (e.g., Banker's algorithm).

32. **What is a livelock?**
    * **Answer:** A situation where threads are **not blocked** but are too busy responding to each other's state changes to make any real progress. They are active but not doing useful work. Example: Two people trying to pass in a hallway, each repeatedly stepping aside in the same direction.

33. **What is starvation?**
    * **Answer:** A situation where a thread is **indefinitely denied access** to a resource it needs to make progress, often because other threads (e.g., higher priority) are constantly being favored.

34. **How can livelocks be prevented or resolved?**
    * **Answer:** Introduce **randomness** in retry attempts or use a more coordinated approach to resource access, like backoff algorithms.

## **Memory Models and Atomicity**

---

35. **What is a memory model in the context of concurrency?**
    * **Answer:** A set of rules that specifies how threads interact through memory. It defines the **visibility and ordering** of memory operations performed by one thread to other threads.

36. **What does "sequentially consistent" mean?**
    * **Answer:** A memory model where the result of any execution is the same as if the operations of all processors were executed in **some sequential order**, and the operations of each individual processor appear in this sequence in the order specified by its program. This is the most intuitive model but can be costly to implement.

37. **What is "relaxed memory ordering"?**
    * **Answer:** A memory model that allows for **more reordering** of memory operations by the compiler and hardware than sequential consistency, potentially improving performance but requiring programmers to use explicit synchronization for correctness.

38. **What does `std::memory_order_acquire` mean in C++?**
    * **Answer:** An operation with acquire semantics ensures that **no memory operations in the current thread that follow this operation** can be reordered before it. It also ensures that all writes in other threads that have release semantics and happened-before this acquire become visible.

39. **What does `std::memory_order_release` mean in C++?**
    * **Answer:** An operation with release semantics ensures that **no memory operations in the current thread that precede this operation** can be reordered after it. It also makes all prior writes in the current thread visible to other threads that perform an acquire operation on the same atomic variable.

40. **What is a "happens-before" relationship?**
    * **Answer:** A fundamental concept in memory models that defines a **partial order** of operations. If operation A happens-before operation B, then the effects of A are guaranteed to be visible to B.

41. **Why are atomic operations important for lock-free programming?**
    * **Answer:** They provide guarantees about indivisibility and memory ordering without relying on traditional locks (mutexes), allowing multiple threads to make progress even if one thread is slow, thus avoiding issues like priority inversion.

42. **What is false sharing?**
    * **Answer:** A performance-degrading situation where multiple threads access different data items located on the **same cache line**. When one thread modifies its data, the cache line is invalidated for other threads, even if they weren't interested in the modified data, causing unnecessary cache coherency traffic.

43. **How can false sharing be mitigated?**
    * **Answer:** By **padding data structures** so that data items frequently accessed by different threads are on different cache lines.

## **Thread Management**

---

44. **How do you create a thread in C++?**
    * **Answer:** Using `std::thread`.
        ```cpp
        #include <iostream>
        #include <thread>
        void task() { std::cout << "Hello from thread!" << std::endl; }
        int main() {
            std::thread t1(task);
            t1.join(); // Wait for t1 to finish
            return 0;
        }
        ```

45. **How do you create a goroutine in Go?**
    * **Answer:** Using the `go` keyword.
        ```go
        import (
            "fmt"
            "time"
        )
        func task() {
            fmt.Println("Hello from goroutine!")
        }
        func main() {
            go task()
            time.Sleep(100 * time.Millisecond) // Wait for goroutine (crude way)
        }
        ```

46. **How do you create a thread in Rust?**
    * **Answer:** Using `std::thread::spawn`.
        ```rust
        use std::thread;
        use std::time::Duration;
        fn task() {
            println!("Hello from thread!");
        }
        fn main() {
            let handle = thread::spawn(task);
            handle.join().unwrap(); // Wait for the thread to finish
        }
        ```

47. **What does `thread::join()` do?**
    * **Answer:** It makes the calling thread **wait for the specified thread to complete** its execution before the calling thread continues.

48. **What is a detached thread (or daemon thread)?**
    * **Answer:** A thread whose execution is **not tied to the lifetime of the thread that created it**. The program can exit even if detached threads are still running. Resources of detached threads are typically reclaimed by the system when they exit.

49. **What is a thread pool?**
    * **Answer:** A collection of **pre-created worker threads** that are ready to execute tasks. This avoids the overhead of creating and destroying threads for each task.

50. **Why use a thread pool?**
    * **Answer:** **Reduces thread creation/destruction overhead**, limits the number of concurrent threads (preventing resource exhaustion), and can simplify task management.

51. **What is `ThreadLocal` storage?**
    * **Answer:** A mechanism that allows each thread to have its **own independent copy** of a variable. This avoids the need for synchronization when accessing that variable, as each thread interacts with its private copy.

## **Asynchronous Programming**

---

52. **What is asynchronous programming?**
    * **Answer:** A programming paradigm that allows work to be done **without blocking the main thread of execution**. When a long-running operation is started (e.g., I/O), the program can continue with other tasks and be notified when the operation completes.

53. **What are the benefits of asynchronous programming?**
    * **Answer:** Improved **responsiveness** (especially in UI applications), better **scalability** (handling more concurrent operations with fewer threads, particularly for I/O-bound tasks).

54. **What is a callback function in asynchronous programming?**
    * **Answer:** A function passed as an argument to another function, which is then **invoked (called back) when an asynchronous operation completes**.

55. **What is "callback hell"?**
    * **Answer:** A situation in asynchronous programming with many nested callbacks, leading to code that is **hard to read, debug, and maintain**.

56. **What are Promises (or Futures)?**
    * **Answer:** Objects that represent the **eventual result (or failure) of an asynchronous operation**. They provide a cleaner way to handle asynchronous operations than raw callbacks, allowing for chaining and better error handling.
    * **C++ Example:** `std::future` and `std::promise`.

57. **What is `async/await`?**
    * **Answer:** Syntactic sugar built on top of Promises/Futures that allows asynchronous code to be written in a style that **looks synchronous**, making it easier to understand and manage.
    * **Rust Example:**
        ```rust
        async fn fetch_data() -> String {
            // Simulate an async operation
            tokio::time::sleep(Duration::from_secs(1)).await;
            "Data fetched".to_string()
        }
        #[tokio::main]
        async fn main() {
            let data = fetch_data().await;
            println!("{}", data);
        }
        ```

58. **What is an event loop?**
    * **Answer:** A central mechanism in many asynchronous frameworks that continuously **waits for and dispatches events or messages** in a program. It allows single-threaded concurrency by processing one event at a time.

59. **When is asynchronous programming preferred over multi-threading?**
    * **Answer:** For **I/O-bound tasks** where threads would spend most of their time waiting. Asynchronous programming can handle many concurrent I/O operations efficiently with fewer threads. For CPU-bound tasks, multi-threading/parallelism is usually better.

## **Language-Specific Concurrency Features**

---

### **Go**

60. **What are goroutines?**
    * **Answer:** **Lightweight, concurrent execution units** managed by the Go runtime. They are cheaper than OS threads.

61. **What are channels in Go?**
    * **Answer:** Typed conduits through which you can **send and receive values** with the channel operator `<-`, enabling communication and synchronization between goroutines.
    * **Go Example:**
        ```go
        messages := make(chan string)
        go func() { messages <- "ping" }()
        msg := <-messages
        fmt.Println(msg) // prints "ping"
        ```

62. **What is a buffered channel in Go?**
    * **Answer:** A channel with a **capacity greater than zero**. Sends to a buffered channel block only when the buffer is full. Receives block when the buffer is empty.

63. **What is an unbuffered channel in Go?**
    * **Answer:** A channel with **zero capacity**. Sender and receiver must be ready simultaneously for communication to occur (it's a rendezvous).

64. **What is the `select` statement in Go used for?**
    * **Answer:** It lets a goroutine **wait on multiple communication operations** (sends or receives on channels). A `select` blocks until one of its cases can run, then it executes that case. If multiple are ready, it chooses one at random.
    * **Go Example:**
        ```go
        select {
        case msg1 := <-ch1:
            fmt.Println("received", msg1)
        case ch2 <- "hello":
            fmt.Println("sent hello")
        default: // Optional: non-blocking operation
            fmt.Println("no communication")
        }
        ```

65. **Explain `sync.Mutex` in Go.**
    * **Answer:** `sync.Mutex` provides mutual exclusion, similar to mutexes in other languages, to protect shared data from concurrent access. It has `Lock()` and `Unlock()` methods.

66. **Explain `sync.WaitGroup` in Go.**
    * **Answer:** `sync.WaitGroup` is used to **wait for a collection of goroutines to finish**. The main goroutine calls `Add` to set the number of goroutines to wait for, each goroutine calls `Done` when it finishes, and `Wait` blocks until all goroutines have finished.

### **Rust**

67. **How does Rust ensure thread safety?**
    * **Answer:** Through its **ownership and borrowing system** combined with the `Send` and `Sync` traits.
        * `Send`: A type `T` is `Send` if it's safe to transfer ownership of `T` to another thread.
        * `Sync`: A type `T` is `Sync` if it's safe to have references (`&T`) to `T` shared across threads.

68. **What is `std::sync::Mutex<T>` in Rust?**
    * **Answer:** A mutual exclusion primitive which ensures that only one thread can access the data `T` it contains at a time. `lock()` returns a `MutexGuard` which automatically unlocks when it goes out of scope (RAII).
    * **Rust Example:**
        ```rust
        use std::sync::{Mutex, Arc};
        use std::thread;

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
        ```

69. **What is `Arc<T>` (Atomically Referenced Counter) in Rust?**
    * **Answer:** A thread-safe reference-counting pointer. `Arc<T>` provides **shared ownership** of a value of type `T`, allocated on the heap. It's used when you need to share data between multiple threads.

70. **What are channels (`std::sync::mpsc`) in Rust?**
    * **Answer:** `mpsc` stands for "multiple producer, single consumer." Channels allow **one-way communication** between threads. You can clone the `Sender` to have multiple producers.

71. **Explain Rust's `async/await` syntax.**
    * **Answer:** `async fn` defines an asynchronous function that returns a `Future`. The `await` keyword is used inside an `async fn` to pause execution until the `Future` it's called on is ready, without blocking the entire thread. This requires an async runtime like Tokio or async-std.

### **C++**

72. **What is `std::thread` in C++?**
    * **Answer:** A class in the C++ Standard Library (`<thread>`) used to **create and manage threads**.

73. **What is `std::mutex` in C++?**
    * **Answer:** A synchronization primitive that provides **exclusive access** to a shared resource. Key methods are `lock()` and `unlock()`.

74. **What is `std::lock_guard` in C++?**
    * **Answer:** A RAII (Resource Acquisition Is Initialization) wrapper for `std::mutex`. It **locks the mutex in its constructor and unlocks it in its destructor**, simplifying lock management and preventing forgetting to unlock.

75. **What is `std::unique_lock` in C++?**
    * **Answer:** Similar to `std::lock_guard` but more flexible. It allows for **deferred locking, timed locking, and explicit unlock/lock calls**. It's also required for use with `std::condition_variable`.

76. **What is `std::condition_variable` in C++?**
    * **Answer:** A synchronization primitive used to **block one or more threads until notified by another thread** that a certain condition is met. It's typically used with `std::unique_lock` and a `std::mutex`.

77. **What are `std::future` and `std::promise` in C++?**
    * **Answer:**
        * `std::promise`: Used to **provide a value or exception** that will be available at some point in the future.
        * `std::future`: Used to **retrieve the value or exception** set by a corresponding `std::promise` or returned by an asynchronous task (e.g., from `std::async`).

78. **What is `std::async` in C++?**
    * **Answer:** A function template that can run a function **asynchronously** (potentially in a new thread) and returns a `std::future` that will eventually hold the result of that function call.

79. **What is `std::atomic<T>` in C++?**
    * **Answer:** A template class that provides **atomic operations** for type `T` (e.g., integers, pointers). These operations are indivisible and can be used to avoid data races without explicit locks.

## **Testing and Debugging Concurrent Code**

---

80. **Why is testing concurrent code difficult?**
    * **Answer:** Due to **non-determinism**. Bugs like race conditions or deadlocks might only appear under specific, hard-to-reproduce timing conditions.

81. **What are thread sanitizers?**
    * **Answer:** Tools (e.g., TSan in GCC/Clang, Go's race detector) that dynamically analyze code at runtime to **detect data races and other threading errors**.

82. **What is stress testing in the context of concurrency?**
    * **Answer:** Running the application under **high load and with many concurrent threads/operations** for extended periods to try and expose concurrency bugs.

83. **How can you make concurrent bugs more reproducible?**
    * **Answer:** Introducing **controlled delays** (`sleeps`) at specific points can sometimes help, though it's not a foolproof method. Using **logging** extensively can help trace execution. Some specialized testing tools allow for deterministic replay of concurrent executions.

84. **What is model checking for concurrent systems?**
    * **Answer:** An automated technique that **explores all possible states** of a (typically simplified) model of a concurrent system to verify properties like deadlock-freedom or absence of race conditions.

## **Advanced Concurrency Concepts**

---

85. **What is lock-free programming?**
    * **Answer:** A style of concurrent programming where shared data is accessed **without using locks** (like mutexes). It typically relies on atomic operations. Guarantees system-wide progress.

86. **What is wait-free programming?**
    * **Answer:** A stronger guarantee than lock-free. Every thread making a call to a wait-free operation is **guaranteed to complete its operation in a finite number of its own steps**, regardless of the execution speed or state of other threads.

87. **What is the ABA problem?**
    * **Answer:** A problem in lock-free algorithms where a location is read twice, has the same value both times, and it's assumed nothing changed. However, between the reads, another thread could have changed it to B, then back to A. This can lead to incorrect behavior in algorithms like lock-free stacks or queues.
    * **Solution:** Often solved using **tagged pointers** or version counters.

88. **What is Software Transactional Memory (STM)?**
    * **Answer:** A concurrency control mechanism analogous to database transactions. Programmers define blocks of code (transactions) that should execute atomically. The STM system manages conflicts and rollbacks.

89. **What are coroutines (distinct from goroutines)?**
    * **Answer:** Functions that can **suspend their execution** and resume later from the point of suspension. They are a form of cooperative multitasking, often used for iterators, generators, and asynchronous programming without explicit threads.

90. **What is message passing concurrency?**
    * **Answer:** A concurrency model where independent actors or processes communicate by **sending and receiving messages**, rather than sharing memory directly (e.g., Actor model, Go channels). "Share memory by communicating, don't communicate by sharing memory."

## **Performance Considerations**

---

91. **What is Amdahl's Law?**
    * **Answer:** A formula that gives the theoretical **maximum speedup** of a task when using multiple processors. It states that speedup is limited by the **serial portion** of the task.
    * $S_{latency}(s) = \frac{1}{(1-P) + \frac{P}{s}}$, where P is the proportion of parallelizable code and s is the speedup of that part.

92. **What is Gustafson's Law?**
    * **Answer:** Another perspective on parallel computing performance, which argues that as processing power increases, the **problem size also tends to increase**. It suggests that for larger problems, the speedup can be more significant than Amdahl's Law might predict for a fixed problem size.

93. **What is the overhead of using locks?**
    * **Answer:** **Acquiring and releasing locks takes time**. Contention (multiple threads trying to acquire the same lock) can cause threads to block, leading to context switches and reduced parallelism.

94. **How can contention be reduced?**
    * **Answer:**
        * **Reduce critical section size:** Hold locks for the shortest possible time.
        * **Use finer-grained locking:** Lock smaller, independent parts of data.
        * **Use read-write locks:** If reads are more frequent.
        * **Use lock-free data structures.**
        * **Partition data.**

95. **What is scalability in a concurrent application?**
    * **Answer:** The ability of the application to **handle an increasing workload or use more processors effectively** to improve performance.

## **Common Concurrency Problems & Design Patterns**

---

96. **What is the Producer-Consumer problem?**
    * **Answer:** A classic synchronization problem where one or more **producer threads create data** items and put them into a shared buffer, and one or more **consumer threads retrieve items** from the buffer. Requires synchronization to prevent buffer overflow/underflow and race conditions.
    * **Common Solution:** Use a bounded buffer with mutexes and condition variables (or semaphores).

97. **What is the Readers-Writers problem?**
    * **Answer:** A problem where multiple threads need to access a shared data structure. Some threads only read (readers), and some modify (writers). The goal is to allow multiple readers concurrently, but only one writer exclusively.
    * **Common Solution:** Use read-write locks.

98. **What is the Dining Philosophers problem?**
    * **Answer:** A classic problem illustrating deadlock and starvation. Five philosophers sit at a round table with five forks between them. Each needs two forks to eat. If each picks up their left fork simultaneously, they all wait for their right fork, leading to deadlock.
    * **Solutions:** Resource hierarchy (ordering forks), allowing at most N-1 philosophers to pick up forks, using a central arbiter.

99. **Explain the "Work Stealing" pattern.**
    * **Answer:** A technique used in some thread pool implementations (e.g., fork-join pools). Each worker thread has its own deque (double-ended queue) of tasks. If a thread runs out of tasks, it can "steal" tasks from the deque of another busy thread. This helps in load balancing.

100. **What is the "Actor Model"?**
     * **Answer:** A concurrency model where "actors" are the universal primitives of concurrent computation. Each actor has a mailbox for incoming messages and processes them sequentially. Actors can create other actors, send messages to other actors, and designate behavior for the next message. This model avoids shared state and relies on message passing.