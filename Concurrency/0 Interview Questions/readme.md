# 100 Interview Questions and Answers on Concurrency in Programming

## **Fundamentals (Questions 1-20)**

### **1. What is concurrency in programming?**
**Answer:** Concurrency is the ability of a program to execute multiple tasks simultaneously or in an overlapping manner. It allows multiple computations to make progress at the same time, improving performance and responsiveness.

### **2. What's the difference between concurrency and parallelism?**
**Answer:** 
- **Concurrency**: Tasks appear to run simultaneously but may share CPU time (interleaved execution)
- **Parallelism**: Tasks actually run at the same time on multiple CPU cores

### **3. What is a thread?**
**Answer:** A **thread** is the smallest unit of execution within a process. Threads share memory space but have their own stack and registers. Multiple threads can exist within a single process.

### **4. What is a process?**
**Answer:** A **process** is an independent program in execution with its own memory space. Processes are isolated from each other and communicate through inter-process communication (IPC).

### **5. What are the main benefits of concurrent programming?**
**Answer:**
- **Better resource utilization**
- **Improved responsiveness**
- **Enhanced throughput**
- **Ability to handle multiple users/requests**
- **Better scalability**

### **6. What are the challenges of concurrent programming?**
**Answer:**
- **Race conditions**
- **Deadlocks**
- **Data synchronization**
- **Debugging complexity**
- **Resource contention**

### **7. What is a race condition?**
**Answer:** A **race condition** occurs when multiple threads access shared data simultaneously, and the final result depends on the timing of their execution. This leads to unpredictable behavior.

```cpp
// Example of race condition
int counter = 0;
void increment() {
    counter++; // Not atomic!
}
```

### **8. What is thread safety?**
**Answer:** **Thread safety** means that a piece of code can be safely executed by multiple threads simultaneously without causing race conditions or data corruption.

### **9. What is atomic operation?**
**Answer:** An **atomic operation** is an operation that completes entirely without interruption. It appears as a single, indivisible step to other threads.

```cpp
#include <atomic>
std::atomic<int> counter{0};
counter++; // Atomic operation
```

### **10. What is a critical section?**
**Answer:** A **critical section** is a portion of code that accesses shared resources and must not be executed by more than one thread at a time.

### **11. What is mutual exclusion (mutex)?**
**Answer:** **Mutual exclusion (mutex)** is a synchronization primitive that ensures only one thread can access a critical section at a time.

```cpp
#include <mutex>
std::mutex mtx;
void safe_function() {
    std::lock_guard<std::mutex> lock(mtx);
    // Critical section
}
```

### **12. What is a semaphore?**
**Answer:** A **semaphore** is a synchronization primitive that maintains a count and allows a specific number of threads to access a resource simultaneously.

### **13. What is deadlock?**
**Answer:** **Deadlock** is a situation where two or more threads are blocked forever, each waiting for the other to release a resource.

### **14. What are the four conditions for deadlock (Coffman conditions)?**
**Answer:**
1. **Mutual exclusion**: At least one resource is held in non-sharable mode
2. **Hold and wait**: A thread holds resources while waiting for others
3. **No preemption**: Resources cannot be forcibly taken from threads
4. **Circular wait**: A circular chain of threads waiting for resources

### **15. What is livelock?**
**Answer:** **Livelock** occurs when threads are not blocked but keep changing their state in response to other threads, preventing progress without being technically deadlocked.

### **16. What is starvation?**
**Answer:** **Starvation** occurs when a thread is perpetually denied access to resources it needs, usually because other threads continuously have higher priority.

### **17. What is a context switch?**
**Answer:** A **context switch** is the process of saving and restoring the state of a thread/process when the CPU switches from executing one to another.

### **18. What is thread pooling?**
**Answer:** **Thread pooling** is a pattern where a fixed number of threads are created and reused to execute multiple tasks, avoiding the overhead of creating/destroying threads.

```cpp
#include <thread>
#include <queue>
#include <functional>

class ThreadPool {
    std::vector<std::thread> workers;
    std::queue<std::function<void()>> tasks;
    // ... implementation
};
```

### **19. What is the difference between preemptive and cooperative multitasking?**
**Answer:**
- **Preemptive**: OS forcibly interrupts threads to give CPU time to others
- **Cooperative**: Threads voluntarily yield control to others

### **20. What is a scheduler?**
**Answer:** A **scheduler** is the OS component responsible for deciding which thread/process gets CPU time and for how long.

## **Synchronization Primitives (Questions 21-40)**

### **21. What is a condition variable?**
**Answer:** A **condition variable** allows threads to wait for certain conditions to become true and be notified when they change.

```cpp
#include <condition_variable>
std::condition_variable cv;
std::mutex mtx;
bool ready = false;

void wait_for_condition() {
    std::unique_lock<std::mutex> lock(mtx);
    cv.wait(lock, []{ return ready; });
}
```

### **22. What is a read-write lock?**
**Answer:** A **read-write lock** allows multiple readers or a single writer, but not both simultaneously. It optimizes for scenarios with frequent reads and infrequent writes.

```rust
use std::sync::RwLock;
let lock = RwLock::new(5);
{
    let r1 = lock.read().unwrap();
    let r2 = lock.read().unwrap(); // OK - multiple readers
}
{
    let w = lock.write().unwrap(); // OK - exclusive writer
}
```

### **23. What is a spinlock?**
**Answer:** A **spinlock** is a lock where threads continuously check (spin) if the lock is available rather than sleeping. It's efficient for short critical sections.

```cpp
#include <atomic>
class Spinlock {
    std::atomic_flag flag = ATOMIC_FLAG_INIT;
public:
    void lock() {
        while (flag.test_and_set(std::memory_order_acquire));
    }
    void unlock() {
        flag.clear(std::memory_order_release);
    }
};
```

### **24. What is a barrier?**
**Answer:** A **barrier** is a synchronization point where threads wait until all participating threads reach the barrier before continuing.

### **25. What is compare-and-swap (CAS)?**
**Answer:** **Compare-and-swap** is an atomic operation that compares a memory location's value with an expected value and, if they match, swaps it with a new value.

```cpp
bool compare_and_swap(std::atomic<int>& value, int expected, int desired) {
    return value.compare_exchange_weak(expected, desired);
}
```

### **26. What is ABA problem?**
**Answer:** The **ABA problem** occurs in lock-free algorithms when a value changes from A to B and back to A, making CAS operations think nothing changed when it actually did.

### **27. What is a futex?**
**Answer:** A **futex** (fast userspace mutex) is a Linux kernel mechanism that provides efficient blocking synchronization by combining userspace atomic operations with kernel-space blocking.

### **28. What is priority inversion?**
**Answer:** **Priority inversion** occurs when a high-priority thread is blocked by a low-priority thread holding a required resource, while a medium-priority thread runs instead.

### **29. What is lock-free programming?**
**Answer:** **Lock-free programming** uses atomic operations instead of locks to coordinate access to shared data, avoiding blocking and potential deadlocks.

### **30. What is wait-free programming?**
**Answer:** **Wait-free programming** guarantees that every thread will complete its operations in a bounded number of steps, regardless of other threads' behavior.

### **31. What is a memory barrier/fence?**
**Answer:** A **memory barrier** is an instruction that prevents reordering of memory operations by the CPU or compiler across the barrier point.

```cpp
#include <atomic>
std::atomic_thread_fence(std::memory_order_acquire);
```

### **32. What are the different memory ordering models?**
**Answer:**
- **Relaxed**: No synchronization constraints
- **Acquire**: Prevents reordering of subsequent reads/writes
- **Release**: Prevents reordering of preceding reads/writes  
- **Sequential consistency**: Strongest ordering, appears as if all operations execute in some sequential order

### **33. What is a counting semaphore vs binary semaphore?**
**Answer:**
- **Counting semaphore**: Can have values > 1, allowing multiple threads
- **Binary semaphore**: Can only be 0 or 1, similar to mutex

### **34. What is a reentrant lock?**
**Answer:** A **reentrant lock** (recursive lock) can be acquired multiple times by the same thread without causing deadlock.

```cpp
#include <mutex>
std::recursive_mutex rmtx;
void recursive_function(int n) {
    std::lock_guard<std::recursive_mutex> lock(rmtx);
    if (n > 0) recursive_function(n - 1);
}
```

### **35. What is lock ordering and why is it important?**
**Answer:** **Lock ordering** means acquiring locks in a consistent order across all threads to prevent deadlock. If all threads acquire locks in the same order, circular wait cannot occur.

### **36. What is a try-lock?**
**Answer:** A **try-lock** attempts to acquire a lock without blocking. It returns immediately with success or failure status.

```cpp
std::mutex mtx;
if (mtx.try_lock()) {
    // Got the lock
    mtx.unlock();
} else {
    // Lock not available
}
```

### **37. What is double-checked locking?**
**Answer:** **Double-checked locking** is a pattern that reduces locking overhead in singleton implementation by checking the condition before acquiring the lock, then checking again after acquiring it.

```cpp
std::atomic<Singleton*> instance{nullptr};
std::mutex mtx;

Singleton* getInstance() {
    if (instance.load() == nullptr) {
        std::lock_guard<std::mutex> lock(mtx);
        if (instance.load() == nullptr) {
            instance.store(new Singleton());
        }
    }
    return instance.load();
}
```

### **38. What is a monitor?**
**Answer:** A **monitor** is a synchronization construct that combines mutual exclusion with the ability to wait for conditions to become true, encapsulating both data and synchronization.

### **39. What are lock-free data structures?**
**Answer:** **Lock-free data structures** use atomic operations and careful memory ordering to provide thread safety without using locks, examples include lock-free queues and stacks.

### **40. What is RCU (Read-Copy-Update)?**
**Answer:** **RCU** is a synchronization mechanism that allows readers to access data structures concurrently with writers, where updates create new versions of the data.

## **Thread Communication (Questions 41-60)**

### **41. What is a message queue?**
**Answer:** A **message queue** is a communication mechanism where threads/processes send messages to a queue, and other threads read them asynchronously.

```go
ch := make(chan string, 10) // Buffered channel
go func() {
    ch <- "Hello" // Send message
}()
message := <-ch // Receive message
```

### **42. What is the Producer-Consumer problem?**
**Answer:** The **Producer-Consumer problem** involves coordinating producers that generate data and consumers that process it, typically using a bounded buffer.

### **43. What is a channel in Go?**
**Answer:** A **channel** in Go is a typed communication mechanism that allows goroutines to send and receive values, following the principle "Don't communicate by sharing memory; share memory by communicating."

### **44. What's the difference between buffered and unbuffered channels?**
**Answer:**
- **Unbuffered channel**: Synchronous communication, sender blocks until receiver is ready
- **Buffered channel**: Asynchronous communication up to buffer capacity

### **45. What is select statement in Go?**
**Answer:** The **select statement** allows a goroutine to wait on multiple channel operations simultaneously.

```go
select {
case msg1 := <-ch1:
    // Handle msg1
case msg2 := <-ch2:
    // Handle msg2
default:
    // Non-blocking default case
}
```

### **46. What is actor model?**
**Answer:** The **actor model** is a concurrency paradigm where actors are independent entities that communicate only through message passing, with no shared state.

### **47. What is CSP (Communicating Sequential Processes)?**
**Answer:** **CSP** is a formal language for describing concurrent systems where processes communicate through channels rather than shared memory.

### **48. What is pipe in operating systems?**
**Answer:** A **pipe** is a unidirectional communication channel between processes, allowing data flow from one process to another.

### **49. What is shared memory?**
**Answer:** **Shared memory** is a memory region accessible by multiple processes/threads, requiring synchronization mechanisms to prevent race conditions.

### **50. What is memory-mapped I/O for IPC?**
**Answer:** **Memory-mapped I/O** allows processes to share data by mapping the same file or memory region into their address spaces.

### **51. What is a socket for IPC?**
**Answer:** A **socket** provides communication between processes, either locally (Unix domain sockets) or over networks (TCP/UDP sockets).

### **52. What is signal in operating systems?**
**Answer:** A **signal** is an asynchronous notification sent to a process to notify it of an event (like SIGTERM, SIGKILL).

### **53. What is mailbox-based communication?**
**Answer:** **Mailbox-based communication** is a pattern where threads/processes send messages to named mailboxes, allowing many-to-many communication.

### **54. What is publish-subscribe pattern?**
**Answer:** **Publish-subscribe** is a messaging pattern where publishers send messages to topics, and subscribers receive messages from topics they're interested in.

### **55. What is request-response pattern?**
**Answer:** **Request-response** is a communication pattern where a client sends a request and waits for a response from a server.

### **56. What is event-driven programming?**
**Answer:** **Event-driven programming** is a paradigm where program flow is determined by events (user input, sensor output, messages) rather than sequential execution.

### **57. What is asynchronous message passing?**
**Answer:** **Asynchronous message passing** allows the sender to continue execution immediately after sending a message without waiting for the receiver.

### **58. What is synchronous message passing?**
**Answer:** **Synchronous message passing** requires the sender to wait until the receiver has received and possibly processed the message.

### **59. What is a callback function in concurrent programming?**
**Answer:** A **callback function** is a function passed as an argument to be called when an asynchronous operation completes.

### **60. What is future/promise pattern?**
**Answer:** **Future/promise** is a pattern where a function returns a placeholder (future) for a result that will be available later, allowing asynchronous computation.

```rust
use std::future::Future;
async fn fetch_data() -> Result<String, Error> {
    // Asynchronous operation
    Ok("data".to_string())
}
```

## **Design Patterns (Questions 61-80)**

### **61. What is the Singleton pattern in concurrent environment?**
**Answer:** **Thread-safe Singleton** ensures only one instance exists even with multiple threads, typically using double-checked locking or std::once_flag.

```cpp
class Singleton {
    static std::once_flag flag;
    static std::unique_ptr<Singleton> instance;
public:
    static Singleton* getInstance() {
        std::call_once(flag, []() {
            instance = std::make_unique<Singleton>();
        });
        return instance.get();
    }
};
```

### **62. What is Thread-Local Storage (TLS)?**
**Answer:** **TLS** provides each thread with its own copy of a variable, avoiding synchronization needs for thread-specific data.

```cpp
thread_local int counter = 0; // Each thread has its own counter
```

### **63. What is the Worker Thread pattern?**
**Answer:** **Worker Thread pattern** uses a pool of threads to process tasks from a queue, decoupling task creation from execution.

### **64. What is the Fork-Join pattern?**
**Answer:** **Fork-Join** pattern divides a task into subtasks (fork), executes them in parallel, then combines results (join).

```cpp
#include <future>
auto task1 = std::async(std::launch::async, work_function1);
auto task2 = std::async(std::launch::async, work_function2);
int result = task1.get() + task2.get(); // Join
```

### **65. What is the Pipeline pattern?**
**Answer:** **Pipeline pattern** chains processing stages where each stage processes data and passes it to the next stage, enabling parallel processing.

### **66. What is the Map-Reduce pattern?**
**Answer:** **Map-Reduce** applies a function to all elements (map), then combines results (reduce), often used for parallel data processing.

### **67. What is the Leader-Follower pattern?**
**Answer:** **Leader-Follower** pattern has one thread (leader) listening for events while others (followers) wait. When an event occurs, leader becomes follower and a follower becomes the new leader.

### **68. What is the Half-Sync/Half-Async pattern?**
**Answer:** **Half-Sync/Half-Async** separates synchronous and asynchronous processing layers, often using a queue for communication between them.

### **69. What is the Active Object pattern?**
**Answer:** **Active Object** decouples method invocation from execution by queuing method requests and executing them in a separate thread.

### **70. What is the Reactor pattern?**
**Answer:** **Reactor** pattern handles multiple I/O events by demultiplexing and dispatching them to appropriate handlers, commonly used in event-driven servers.

### **71. What is the Proactor pattern?**
**Answer:** **Proactor** pattern initiates asynchronous operations and handles their completion events, focusing on completion handling rather than event detection.

### **72. What is immutable objects pattern?**
**Answer:** **Immutable objects** cannot be modified after creation, making them inherently thread-safe and eliminating the need for synchronization.

### **73. What is the Copy-on-Write (COW) pattern?**
**Answer:** **COW** delays copying shared data until a write operation occurs, optimizing for scenarios with many readers and few writers.

### **74. What is the Balking pattern?**
**Answer:** **Balking pattern** immediately returns from a method if the object is in an inappropriate state for the operation.

### **75. What is the Guarded Suspension pattern?**
**Answer:** **Guarded Suspension** waits until a guard condition becomes true before proceeding with the operation.

### **76. What is the Scheduler pattern?**
**Answer:** **Scheduler pattern** manages when and how threads are executed based on priorities, policies, and resource availability.

### **77. What is the Resource Pool pattern?**
**Answer:** **Resource Pool** manages reusable resources (like database connections) to avoid the cost of creating/destroying them repeatedly.

### **78. What is the Master-Worker pattern?**
**Answer:** **Master-Worker** pattern has a master thread that distributes work to worker threads and collects results.

### **79. What is the Two-Phase Termination pattern?**
**Answer:** **Two-Phase Termination** safely shuts down threads by first setting a termination flag, then waiting for threads to finish their current work.

### **80. What is the Thread-Specific Storage pattern?**
**Answer:** **Thread-Specific Storage** associates data with individual threads, avoiding synchronization overhead for thread-local data.

## **Performance and Debugging (Questions 81-100)**

### **81. How do you detect race conditions?**
**Answer:** **Race condition detection** methods:
- **Static analysis tools** (e.g., clang static analyzer)
- **Dynamic analysis tools** (e.g., ThreadSanitizer)
- **Code review** and testing
- **Stress testing** with multiple threads

### **82. What is ThreadSanitizer?**
**Answer:** **ThreadSanitizer** is a runtime tool that detects data races in C/C++ programs by monitoring memory accesses and detecting unsynchronized access to shared memory.

### **83. How do you profile concurrent programs?**
**Answer:** **Profiling concurrent programs**:
- **perf**: Linux profiler for CPU usage
- **Intel VTune**: Advanced profiling tool
- **Visual Studio Diagnostic Tools**
- **Custom timing measurements**
- **Lock contention analyzers**

### **84. What is lock contention?**
**Answer:** **Lock contention** occurs when multiple threads compete for the same lock, leading to threads waiting and reduced performance.

### **85. How do you reduce lock contention?**
**Answer:** **Reducing lock contention**:
- **Reduce critical section size**
- **Use lock-free data structures**
- **Lock splitting/stripping**
- **Use read-write locks for read-heavy scenarios**
- **Avoid unnecessary locking**

### **86. What is false sharing?**
**Answer:** **False sharing** occurs when threads modify different variables that reside on the same cache line, causing unnecessary cache invalidations and performance degradation.

```cpp
struct BadAlignment {
    int thread1_var;  // These might be on same cache line
    int thread2_var;  // causing false sharing
};

struct GoodAlignment {
    alignas(64) int thread1_var;  // 64-byte aligned
    alignas(64) int thread2_var;  // separate cache lines
};
```

### **87. What is cache coherency?**
**Answer:** **Cache coherency** ensures that all processors see a consistent view of memory when multiple caches contain copies of the same memory location.

### **88. What is memory consistency model?**
**Answer:** **Memory consistency model** defines the order in which memory operations appear to execute, affecting how concurrent programs behave across different architectures.

### **89. How do you debug deadlocks?**
**Answer:** **Deadlock debugging**:
- **Stack traces** of blocked threads
- **Deadlock detection tools** (e.g., jstack for Java)
- **Lock ordering analysis**
- **Resource allocation graphs**
- **Timeout-based detection**

### **90. What is load balancing in concurrent systems?**
**Answer:** **Load balancing** distributes work evenly across multiple threads/processors to maximize resource utilization and minimize completion time.

### **91. What is work stealing?**
**Answer:** **Work stealing** is a load balancing technique where idle threads steal work from busy threads' task queues.

### **92. What is NUMA (Non-Uniform Memory Access)?**
**Answer:** **NUMA** is a computer memory design where memory access time depends on processor location. Some memory regions are closer (faster) than others.

### **93. How do you measure concurrent program performance?**
**Answer:** **Performance metrics**:
- **Throughput**: Tasks completed per unit time
- **Latency**: Time to complete individual tasks
- **CPU utilization**: Percentage of CPU time used
- **Scalability**: Performance improvement with more cores
- **Lock contention**: Time spent waiting for locks

### **94. What is Amdahl's Law?**
**Answer:** **Amdahl's Law** calculates the theoretical speedup of a program when parallelized: $Speedup = \frac{1}{(1-P) + \frac{P}{N}}$ where P is the parallelizable portion and N is the number of processors.

### **95. What is the difference between latency and throughput?**
**Answer:**
- **Latency**: Time taken to complete a single operation
- **Throughput**: Number of operations completed per unit time

These can be inversely related in concurrent systems.

### **96. What are common performance anti-patterns in concurrent programming?**
**Answer:** **Performance anti-patterns**:
- **Over-synchronization** (too many locks)
- **Under-parallelization** (insufficient concurrency)
- **Resource contention** (competing for shared resources)
- **Context switching overhead** (too many threads)
- **False sharing** (cache line bouncing)

### **97. What is priority inheritance?**
**Answer:** **Priority inheritance** is a solution to priority inversion where a low-priority thread temporarily inherits the priority of a high-priority thread it's blocking.

### **98. What is lock-free vs wait-free performance trade-offs?**
**Answer:**
- **Lock-free**: Better average performance, potential for unbounded waiting under contention
- **Wait-free**: Guaranteed bounded completion time, often higher overhead
- **Locks**: Simplicity vs potential for blocking and priority inversion

### **99. How do you optimize memory access patterns in concurrent programs?**
**Answer:** **Memory optimization**:
- **Cache-friendly data structures** (arrays vs linked lists)
- **Memory pooling** to reduce allocation overhead
- **NUMA-aware allocation** on multi-socket systems
- **Avoiding false sharing** with proper alignment
- **Prefetching** for predictable access patterns

### **100. What are the key principles for writing efficient concurrent code?**
**Answer:** **Key principles**:
- **Minimize shared mutable state**
- **Use appropriate synchronization primitives**
- **Avoid premature optimization**
- **Design for scalability**
- **Measure and profile performance**
- **Consider lock-free alternatives when appropriate**
- **Keep critical sections small**
- **Use immutable data structures when possible**
- **Understand your hardware architecture**
- **Test thoroughly under load**

---

