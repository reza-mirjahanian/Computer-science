

## Concurrency: The Fundamentals

At its heart, **concurrency** is the ability of different parts or units of a program, algorithm, or problem to be executed out-of-order or in partial order, without affecting the final outcome. This doesn't necessarily mean that they are running at the exact same instant in time, but rather that the execution of these parts can overlap. Think of it like a chef juggling multiple tasks in a kitchen: chopping vegetables, stirring a pot, and preheating an oven. They switch between tasks, making progress on each one.

The primary goal of concurrency is to improve **throughput** (the amount of work done in a given time) and **responsiveness** (the ability of the system to react to external events in a timely manner).

There are two fundamental units of execution to understand in the context of concurrency:

1.  **Processes**: A process is an instance of a computer program that is being executed. Each process has its own **independent memory space**. This means that if you have two processes, they cannot directly access each other's variables. Communication between processes (Inter-Process Communication or IPC) requires explicit mechanisms like pipes, sockets, or shared files.
2.  **Threads**: A thread is the smallest unit of execution within a process. Multiple threads can exist within a single process and they **share the same memory space**. This makes communication between threads easier (they can directly read and write the same variables) but also introduces significant challenges in managing shared data.

---
### Processes

Processes are managed by the operating system. Creating a new process is generally more **resource-intensive** (slower and consumes more memory) than creating a new thread.

**Use Cases for Processes:**

* Running independent applications.
* Tasks that require strong isolation for security or stability. If one process crashes, it typically doesn't affect other processes.
* Leveraging multiple CPU cores for truly parallel execution (though threads can also achieve this).

**Python Example: Creating Processes**

```python
import multiprocessing
import os
import time

def worker_process(process_name):
    print(f"Process {process_name} (PID: {os.getpid()}) starting.")
    time.sleep(2)
    print(f"Process {process_name} (PID: {os.getpid()}) finishing.")

if __name__ == "__main__":
    # On Windows, multiprocessing can sometimes have issues without this guard
    process1 = multiprocessing.Process(target=worker_process, args=("P1",))
    process2 = multiprocessing.Process(target=worker_process, args=("P2",))

    print(f"Main process (PID: {os.getpid()}) starting P1 and P2.")
    process1.start()
    process2.start()

    print(f"Main process waiting for P1 and P2 to complete.")
    process1.join() # Wait for process1 to finish
    process2.join() # Wait for process2 to finish

    print("All processes finished.")
```

**Edge Cases/Considerations for Processes:**

* **IPC Overhead**: Communicating data between processes can be slow due to the need for serialization/deserialization and kernel-level operations.
* **Resource Consumption**: Each process has its own memory, file descriptors, etc., which can lead to higher overall resource usage.
* **Complexity of Shared State**: If processes need to share complex state, mechanisms like shared memory (e.g., `multiprocessing.Value`, `multiprocessing.Array`, or memory-mapped files) must be carefully managed.

---
### Threads

Threads live within a process and share its resources. This makes them lighter weight than processes.

**Use Cases for Threads:**

* **I/O-bound tasks**: When a program spends a lot of time waiting for external operations (e.g., network requests, file reads/writes), threads can allow the program to perform other tasks while waiting, improving responsiveness. For example, a GUI application can use a separate thread to handle user input while another thread performs a lengthy calculation.
* **Tasks with frequent communication**: Since threads share memory, passing data between them is fast.
* **Concurrent execution within a single application**: Dividing a complex task into smaller, concurrently executable parts.

**Python Example: Creating Threads**

```python
import threading
import time
import os # For process ID, though threads share the same PID

# Note: In CPython, due to the Global Interpreter Lock (GIL),
# only one thread can hold control of the Python interpreter at any given time.
# This means that for CPU-bound tasks, threading in Python might not offer
# true parallelism on multi-core processors. However, for I/O-bound tasks,
# it's still very effective as threads can release the GIL during I/O waits.

shared_data = 0
lock = threading.Lock() # We'll discuss locks soon

def worker_thread(thread_name, iterations):
    global shared_data
    print(f"Thread {thread_name} (PID: {os.getpid()}) starting.")
    for i in range(iterations):
        # Simulate some work
        time.sleep(0.01)
        # Accessing shared data - needs synchronization!
        with lock: # Acquire the lock before modifying shared_data
            local_copy = shared_data
            local_copy += 1
            # Simulate some processing time before writing back
            time.sleep(0.01)
            shared_data = local_copy
            # print(f"Thread {thread_name}: shared_data = {shared_data}") # Can be noisy
    print(f"Thread {thread_name} (PID: {os.getpid()}) finishing.")

if __name__ == "__main__":
    thread1 = threading.Thread(target=worker_thread, args=("T1", 10))
    thread2 = threading.Thread(target=worker_thread, args=("T2", 10))

    print(f"Main thread (PID: {os.getpid()}) starting T1 and T2.")
    thread1.start()
    thread2.start()

    print(f"Main thread waiting for T1 and T2 to complete.")
    thread1.join() # Wait for thread1 to finish
    thread2.join() # Wait for thread2 to finish

    print(f"All threads finished. Final shared_data = {shared_data}")
    # Expected: 20 if synchronization is correct.
    # Without lock, it might be less due to race conditions.
```

**Edge Cases/Considerations for Threads:**

* **Race Conditions**: When multiple threads access shared data concurrently, and at least one of them modifies the data, the outcome depends on the unpredictable order in which their operations are interleaved. This can lead to corrupted data. The `shared_data` example above *without* the `lock` would likely demonstrate this.
* **Deadlocks**: A situation where two or more threads are blocked forever, each waiting for the other to release a resource.
* **Starvation**: A thread is perpetually denied necessary resources to proceed with its work.
* **Complexity of Debugging**: Concurrent programs can be harder to debug because issues might be timing-dependent and not easily reproducible.
* **Global Interpreter Lock (GIL) in CPython**: As mentioned, the GIL in CPython allows only one thread to execute Python bytecode at a time within a single process. This simplifies CPython's implementation and memory management but limits the parallelism achievable with threads for CPU-bound tasks. For I/O-bound tasks, threads still provide concurrency because the GIL is released during I/O operations. Languages like Java, C++, or Go do not have a GIL in the same way and can achieve true parallelism with threads on multi-core systems.

**Table: Processes vs. Threads**

| Feature          | Process                                   | Thread                                         |
| :--------------- | :---------------------------------------- | :--------------------------------------------- |
| **Memory Space** | Independent                               | Shared within the same process                 |
| **Creation** | Slower, more resource-intensive           | Faster, less resource-intensive                |
| **Communication**| IPC (pipes, sockets, shared memory) - slower | Shared memory (direct access) - faster          |
| **Isolation** | High (crash in one doesn't affect others) | Low (crash in one can affect the whole process)|
| **Overhead** | Higher                                    | Lower                                          |
| **Use Case** | Isolated tasks, parallelism             | I/O-bound tasks, responsive UIs, shared tasks  |
| **CPython GIL** | Not directly affected (each process has own GIL) | Limits CPU-bound parallelism in a single process |

---
## Benefits of Concurrency

1.  **Improved Responsiveness**: For applications with user interfaces, concurrency can keep the UI responsive while background tasks (like file downloads or complex calculations) are being performed. The UI thread remains unblocked.
2.  **Increased Throughput/Performance**:
    * **Task Parallelism**: If the underlying hardware has multiple CPU cores, different parts of a task (or different tasks) can truly run in parallel, leading to faster overall completion.
    * **I/O Bound Efficiency**: When a task involves waiting for I/O operations (network, disk), other tasks can run during these waiting periods, making better use of the CPU.
3.  **Resource Utilization**: Better utilization of system resources, especially CPUs. Idle CPU time while waiting for I/O can be used by other threads/processes.
4.  **Problem Decomposition**: Some problems are naturally concurrent and can be modeled more intuitively using concurrent programming constructs.

---
## Challenges in Concurrency (The "Dark Side" 😈)

Concurrency is powerful, but it comes with significant challenges. Writing correct concurrent programs is notoriously difficult.

### 1. Race Conditions

A **race condition** occurs when the behavior of a software system depends on the sequence or timing of uncontrollable events—such as the order in which threads are scheduled to run. It typically happens when multiple threads access shared data concurrently, and at least one of them modifies the data.

**Example: A Simple Counter (Illustrating Race Condition)**

```python
import threading

counter = 0
iterations = 100000

def increment_counter():
    global counter
    for _ in range(iterations):
        # This is the critical section
        # Read current value
        current_value = counter
        # Increment
        current_value += 1
        # Write back
        counter = current_value

if __name__ == "__main__":
    thread1 = threading.Thread(target=increment_counter)
    thread2 = threading.Thread(target=increment_counter)

    thread1.start()
    thread2.start()

    thread1.join()
    thread2.join()

    print(f"Expected counter value: {2 * iterations}")
    print(f"Actual counter value: {counter}")
    # Often, the actual value will be less than expected.
```

**Why it happens:**

1.  Thread 1 reads `counter` (e.g., 0).
2.  Thread 2 reads `counter` (e.g., 0).
3.  Thread 1 increments its local copy to 1.
4.  Thread 2 increments its local copy to 1.
5.  Thread 1 writes 1 back to `counter`.
6.  Thread 2 writes 1 back to `counter`.

The `counter` should be 2, but it's 1. One increment was lost. The operations `read-modify-write` are not **atomic** (indivisible).

**Solution**: Synchronization primitives like **locks (mutexes)**.

### 2. Deadlocks

A **deadlock** is a state in which each member of a group of actions is waiting for some other member to release a resource, and therefore none of them can proceed.

**Classic Deadlock Scenario: The Dining Philosophers Problem (Conceptual)**

Imagine five philosophers sitting around a circular table. Between each pair of adjacent philosophers is a single chopstick. A philosopher needs two chopsticks to eat. Each philosopher can pick up the chopstick to their left or right. If all philosophers simultaneously pick up their left chopstick, then all chopsticks are taken, and no one can pick up their right chopstick. They will all wait indefinitely.

**Python Example: Simple Deadlock**

```python
import threading
import time

# Two resources (locks)
lock_a = threading.Lock()
lock_b = threading.Lock()

def process_one():
    print("Process One: Attempting to acquire lock_a...")
    with lock_a:
        print("Process One: Acquired lock_a.")
        time.sleep(0.1) # Simulate some work
        print("Process One: Attempting to acquire lock_b...")
        with lock_b:
            print("Process One: Acquired lock_b.")
            print("Process One: Releasing both locks.")
    print("Process One: Finished.")


def process_two():
    print("Process Two: Attempting to acquire lock_b...")
    with lock_b:
        print("Process Two: Acquired lock_b.")
        time.sleep(0.1) # Simulate some work
        print("Process Two: Attempting to acquire lock_a...")
        with lock_a:
            print("Process Two: Acquired lock_a.")
            print("Process Two: Releasing both locks.")
    print("Process Two: Finished.")

if __name__ == "__main__":
    thread1 = threading.Thread(target=process_one)
    thread2 = threading.Thread(target=process_two)

    thread1.start()
    thread2.start()

    thread1.join() # These joins will likely hang indefinitely
    thread2.join()

    print("Finished (if deadlock didn't occur).")
```

**Conditions for Deadlock (Coffman Conditions):** All four must hold.

1.  **Mutual Exclusion**: At least one resource must be held in a non-sharable mode. (Only one process can use the resource at any given time).
2.  **Hold and Wait**: A process is holding at least one resource and is waiting to acquire additional resources held by other processes.
3.  **No Preemption**: A resource can be released only voluntarily by the process holding it, after that process has completed its task.
4.  **Circular Wait**: A set of waiting processes {P0, P1, ..., Pn} must exist such that P0 is waiting for a resource held by P1, P1 is waiting for a resource held by P2, ..., Pn-1 is waiting for a resource held by Pn, and Pn is waiting for a resource held by P0.

**Preventing/Avoiding Deadlocks:**

* **Lock Ordering**: Acquire locks in a consistent global order. If all threads acquire `lock_a` before `lock_b`, the deadlock in the example above wouldn't happen.
* **Timeout**: When trying to acquire a lock, use a timeout. If the lock isn't acquired within the timeout, release any locks held and retry (or abort).
* **Deadlock Detection**: Algorithms can detect deadlocks (e.g., by building a resource allocation graph and looking for cycles), and then the system can break the deadlock (e.g., by preempting a resource or killing a process). This is often complex.
* **Avoid Holding Multiple Locks**: If possible, design your system so that a thread doesn't need to hold more than one lock at a time.

### 3. Starvation (Livelock)

**Starvation** occurs when a concurrent process is perpetually denied access to a resource it needs, even though the resource may become available. The process makes no progress.

**Livelock** is a specific form of starvation where processes are not blocked but are busy continuously changing their state in response to changes in other processes, without doing any useful work. Think of two people trying to pass each other in a narrow hallway, each repeatedly stepping aside in the same direction as the other.

**Example: Starvation (Conceptual)**

Imagine a priority-based scheduling system. If high-priority threads are constantly arriving and monopolizing the CPU or a resource, a low-priority thread might never get a chance to run, or run very infrequently, even if it's ready.

**Python Example: Potential Starvation (Simplified)**

```python
import threading
import time
import random

# A shared resource
resource = threading.Lock()
stop_flag = False

def worker(name, is_greedy=False):
    print(f"Worker {name} starting.")
    while not stop_flag:
        if resource.acquire(blocking=False): # Try to acquire without blocking
            print(f"Worker {name} acquired resource.")
            # Simulate work
            time.sleep(random.uniform(0.01, 0.05) if not is_greedy else random.uniform(0.1, 0.3))
            resource.release()
            print(f"Worker {name} released resource.")
            # Non-greedy worker waits a bit, giving others a chance
            if not is_greedy:
                time.sleep(random.uniform(0.05, 0.1))
        else:
            # Could not acquire, try again soon
            # print(f"Worker {name} failed to acquire resource, retrying.")
            time.sleep(random.uniform(0.001, 0.005)) # Spin wait (not ideal for long waits)
    print(f"Worker {name} stopping.")


if __name__ == "__main__":
    # One greedy worker and several normal workers
    # The greedy worker tries to hold the resource longer and re-acquire it quickly.
    workers = []
    workers.append(threading.Thread(target=worker, args=("Greedy", True)))
    for i in range(3):
        workers.append(threading.Thread(target=worker, args=(f"Polite-{i}", False)))

    for w in workers:
        w.start()

    # Run for a few seconds
    time.sleep(5)
    stop_flag = True

    for w in workers:
        w.join()

    print("All workers finished.")
    # Observe if Polite workers get starved by the Greedy one.
    # The output will show how often each worker acquires the resource.
```

**Preventing Starvation:**

* **Fairness**: Use fair locking mechanisms or scheduling policies. For example, a lock that grants access in a First-In-First-Out (FIFO) order. Python's `threading.Lock` is not guaranteed to be fair.
* **Aging**: Increase the priority of processes that have been waiting for a long time.
* **Avoid excessive locking or spin-waiting** by less critical threads.
* **Randomization**: In some livelock scenarios (like backoff protocols), introducing randomness can help break the symmetry.

### 4. Data Corruption

This is a direct consequence of race conditions. When multiple threads modify shared data without proper synchronization, the data can end up in an inconsistent or corrupted state.

### 5. Complexity

Concurrent programs are inherently more complex to design, implement, test, and debug than sequential programs. The non-deterministic nature of thread scheduling can make bugs hard to reproduce and diagnose.

---
## Synchronization Primitives 🛡️

To manage shared resources and coordinate concurrent tasks, we use **synchronization primitives**. These are mechanisms provided by operating systems or programming language runtimes.

### 1. Mutexes (Locks)

A **Mutex** (Mutual Exclusion object), often called a **Lock**, is the most basic synchronization primitive. It ensures that only one thread can access a specific section of code (the **critical section**) or a shared resource at any given time.

* A thread **acquires** (or locks) the mutex before entering a critical section.
* If the mutex is already locked by another thread, the current thread **blocks** (waits) until the mutex is released.
* Once the thread finishes with the critical section, it **releases** (or unlocks) the mutex, allowing another waiting thread to acquire it.

**Python: `threading.Lock`**

```python
import threading
import time

shared_resource = []
lock = threading.Lock()
iterations = 5

def access_shared_resource(thread_name):
    print(f"Thread {thread_name}: attempting to acquire lock")
    # Common pattern: acquire lock, do work in try, release in finally
    # lock.acquire()
    # try:
    #     print(f"Thread {thread_name}: lock acquired")
    #     # Critical Section
    #     current_val = len(shared_resource)
    #     time.sleep(0.1) # Simulate work
    #     shared_resource.append(thread_name + str(current_val))
    #     print(f"Thread {thread_name}: modified resource to {shared_resource}")
    # finally:
    #     lock.release()
    #     print(f"Thread {thread_name}: lock released")

    # Pythonic way using 'with' statement (context manager)
    # Automatically acquires and releases the lock
    with lock:
        print(f"Thread {thread_name}: lock acquired")
        # Critical Section
        current_val = len(shared_resource)
        time.sleep(0.1) # Simulate work
        shared_resource.append(thread_name + str(current_val))
        print(f"Thread {thread_name}: modified resource to {shared_resource}")
    print(f"Thread {thread_name}: lock released (implicitly by 'with')")


if __name__ == "__main__":
    threads = []
    for i in range(iterations):
        thread = threading.Thread(target=access_shared_resource, args=(f"T{i}",))
        threads.append(thread)
        thread.start()

    for thread in threads:
        thread.join()

    print(f"Final shared resource: {shared_resource}")
    print(f"Expected length: {iterations}, Actual length: {len(shared_resource)}")
```

**Edge Cases for Locks:**

* **Forgetting to Release**: If a lock is acquired and not released (e.g., due to an exception or a bug in logic), all other threads waiting for that lock will be blocked indefinitely. The `with` statement in Python helps prevent this.
* **Deadlocks**: As discussed, improper locking order can lead to deadlocks.
* **Performance Bottleneck**: If a lock is held for too long or is too coarse-grained (protects too much code), it can serialize execution and reduce concurrency benefits. Strive for fine-grained locking where possible, but this can increase complexity.
* **Reentrant Locks (`threading.RLock` in Python)**: A standard lock cannot be acquired more than once by the same thread (it would deadlock itself). A reentrant lock (RLock) can be acquired multiple times by the same thread. It keeps a counter for acquisitions and only truly releases when the counter reaches zero (i.e., released as many times as it was acquired by that thread). This is useful in recursive functions or complex call chains where a function holding a lock might call another function that needs to acquire the same lock.

```python
import threading

r_lock = threading.RLock()

def recursive_function(depth):
    if depth <= 0:
        return

    with r_lock:
        print(f"Thread {threading.current_thread().name}: Acquired RLock at depth {depth}")
        recursive_function(depth - 1)
        print(f"Thread {threading.current_thread().name}: Releasing RLock at depth {depth}")

if __name__ == "__main__":
    # Using a normal Lock here would cause a deadlock if depth > 1
    # because the thread would try to acquire a lock it already holds.
    thread1 = threading.Thread(target=recursive_function, args=(3,))
    thread2 = threading.Thread(target=recursive_function, args=(2,))

    thread1.start()
    thread2.start()

    thread1.join()
    thread2.join()
```

### 2. Semaphores

A **semaphore** is a more general synchronization primitive than a mutex. It maintains a counter, which is initialized to some value.

* **`acquire()` (or `P` or `wait`)**: Decrements the semaphore counter. If the counter becomes negative, the thread blocks until the counter is greater than or equal to zero. (In many implementations, if the counter is zero, it blocks until it becomes positive).
* **`release()` (or `V` or `signal`)**: Increments the semaphore counter. If there are threads blocked on the semaphore, one of them is unblocked.

**Types of Semaphores:**

* **Binary Semaphore (value 0 or 1)**: Behaves like a mutex.
* **Counting Semaphore (value >= 0)**: Can be used to control access to a pool of N resources. The initial value of the semaphore is N. Each time a thread acquires a resource, it calls `acquire()` on the semaphore. When it's done, it calls `release()`. If N resources are in use, further `acquire()` calls will block.

**Python: `threading.Semaphore` and `threading.BoundedSemaphore`**

A `BoundedSemaphore` raises a `ValueError` if `release()` is called more times than `acquire()`, preventing the counter from exceeding its initial value. This can be useful for catching bugs.

```python
import threading
import time
import random

# Simulate a pool of 3 connections
connection_pool_size = 3
semaphore = threading.BoundedSemaphore(value=connection_pool_size)

def use_connection(thread_name):
    print(f"Thread {thread_name}: attempting to acquire a connection.")
    with semaphore: # Acquires the semaphore
        print(f"Thread {thread_name}: acquired a connection. (Semaphore count: {semaphore._value})") # _value is internal, for demo
        # Simulate using the connection
        time.sleep(random.uniform(0.5, 2))
        print(f"Thread {thread_name}: releasing connection.")
    # Semaphore is released automatically by 'with'
    print(f"Thread {thread_name}: connection released. (Semaphore count: {semaphore._value})")


if __name__ == "__main__":
    threads = []
    for i in range(7): # Try to run 7 tasks with only 3 available "connections"
        thread = threading.Thread(target=use_connection, args=(f"T{i}",))
        threads.append(thread)
        thread.start()
        time.sleep(0.1) # Stagger starts slightly

    for thread in threads:
        thread.join()

    print("All tasks completed.")

    # Edge Case: Releasing a BoundedSemaphore too many times
    # s = threading.BoundedSemaphore(1)
    # s.acquire()
    # s.release()
    # try:
    #     s.release() # This will raise ValueError
    # except ValueError as e:
    #     print(f"Error: {e}")
```

**Use Cases for Semaphores:**

* Controlling access to a finite number of resources (e.g., database connections, worker threads in a pool).
* Signaling between threads (though Condition Variables are often better for complex signaling).
* Implementing more complex synchronization patterns.

### 3. Condition Variables

A **Condition Variable** (or Monitor) allows threads to wait until a certain condition becomes true. It's almost always used in conjunction with a mutex.

* A thread acquires a mutex.
* It checks if a condition is met.
* If not, it calls `wait()` on the condition variable. This action **atomically releases the mutex** and puts the thread to sleep until another thread `notify()` or `notify_all()` on the same condition variable.
* When another thread changes the state (potentially making the condition true), it acquires the same mutex, makes the change, and then calls `notify()` (wakes one waiting thread) or `notify_all()` (wakes all waiting threads) on the condition variable.
* The awakened thread(s) re-acquire the mutex (the `wait()` call handles this) and should **re-check the condition** (due to "spurious wakeups" or other threads changing the state).

**Python: `threading.Condition`**

```python
import threading
import time

# Shared resource: a message queue
message_queue = []
max_queue_size = 3
queue_lock = threading.Lock() # Mutex to protect the queue
queue_condition = threading.Condition(lock=queue_lock) # Condition variable associated with queue_lock

def producer(name):
    for i in range(5):
        with queue_condition: # Acquires queue_lock
            while len(message_queue) >= max_queue_size:
                print(f"Producer {name}: Queue full. Waiting...")
                queue_condition.wait() # Releases lock, waits for notify
            # Lock re-acquired here
            message = f"Msg-{name}-{i}"
            message_queue.append(message)
            print(f"Producer {name}: Produced '{message}'. Queue size: {len(message_queue)}")
            # queue_condition.notify() # Notify one waiting consumer
            queue_condition.notify_all() # Notify all waiting consumers (if multiple)
            time.sleep(0.5) # Simulate time to produce
    print(f"Producer {name} finished.")


def consumer(name):
    for _ in range(5): # Try to consume 5 messages
        with queue_condition: # Acquires queue_lock
            while not message_queue: # or len(message_queue) == 0
                print(f"Consumer {name}: Queue empty. Waiting...")
                queue_condition.wait() # Releases lock, waits for notify
            # Lock re-acquired here
            message = message_queue.pop(0)
            print(f"Consumer {name}: Consumed '{message}'. Queue size: {len(message_queue)}")
            # queue_condition.notify() # Notify one waiting producer (if queue was full)
            queue_condition.notify_all() # Notify all (if multiple producers/consumers)
            time.sleep(random.uniform(0.6, 1.2)) # Simulate time to consume
    print(f"Consumer {name} finished.")


if __name__ == "__main__":
    producers = [threading.Thread(target=producer, args=(f"P{i}",)) for i in range(2)]
    consumers = [threading.Thread(target=consumer, args=(f"C{i}",)) for i in range(2)]

    for p in producers: p.start()
    time.sleep(0.1) # Stagger starts a bit
    for c in consumers: c.start()

    for p in producers: p.join()
    for c in consumers: c.join()

    print("All producers and consumers finished.")
    print(f"Final queue state: {message_queue}")
```

**Key points for Condition Variables:**

* **Always use with a lock**: The lock protects the shared data and the condition being checked.
* **`wait()` in a loop**: Always re-check the condition after `wait()` returns. This handles spurious wakeups and ensures the condition is truly met before proceeding.
    ```python
    with condition_variable:
        while not some_condition: # The predicate
            condition_variable.wait()
        # Condition is now true, proceed
    ```
* **`notify()` vs `notify_all()`**:
    * `notify()`: Wakes up *one* of the waiting threads. Use if any waiting thread can handle the condition. More efficient if multiple threads are waiting but only one needs to be woken.
    * `notify_all()`: Wakes up *all* waiting threads. Use if multiple threads might be able to proceed or if different threads are waiting for different aspects of the condition. Awakened threads will re-acquire the lock one by one and re-check the condition.

### 4. Events

An **Event** object is a simple synchronization primitive that manages an internal flag. It can be used to signal an event from one thread to others.

* `Event.set()`: Sets the internal flag to true. All threads waiting for it to become true are awakened.
* `Event.clear()`: Resets the internal flag to false.
* `Event.wait(timeout=None)`: Blocks until the internal flag is true. If a timeout is provided, it blocks for at most that many seconds. Returns true if the flag is set before the timeout, false otherwise.
* `Event.is_set()`: Returns true if and only if the internal flag is true.

**Python: `threading.Event`**

```python
import threading
import time

event = threading.Event()

def waiter(name, timeout=None):
    print(f"Waiter {name}: Waiting for event...")
    event_is_set = event.wait(timeout)
    if event_is_set:
        print(f"Waiter {name}: Event received! Processing...")
    else:
        print(f"Waiter {name}: Timed out waiting for event.")

def setter(name, delay):
    print(f"Setter {name}: Sleeping for {delay} seconds...")
    time.sleep(delay)
    print(f"Setter {name}: Setting the event!")
    event.set()

if __name__ == "__main__":
    # Scenario 1: Event is set
    print("--- Scenario 1: Event will be set ---")
    waiter_thread1 = threading.Thread(target=waiter, args=("W1",))
    setter_thread = threading.Thread(target=setter, args=("S1", 2))

    waiter_thread1.start()
    setter_thread.start()

    waiter_thread1.join()
    setter_thread.join()
    print("-" * 20)

    # Reset event for next scenario
    event.clear()
    print("\n--- Scenario 2: Waiter times out ---")
    waiter_thread2 = threading.Thread(target=waiter, args=("W2", 1)) # 1 second timeout
    setter_thread2 = threading.Thread(target=setter, args=("S2", 3)) # Sets event after 3s

    waiter_thread2.start()
    setter_thread2.start()

    waiter_thread2.join()
    setter_thread2.join() # Wait for setter to finish even if waiter timed out
    print("-" * 20)

    # Scenario 3: Event already set
    print("\n--- Scenario 3: Event already set ---")
    if not event.is_set(): # Ensure it's set from previous setter
        event.set()
    waiter_thread3 = threading.Thread(target=waiter, args=("W3",))
    waiter_thread3.start()
    waiter_thread3.join()
    print("-" * 20)
```

**Use Cases for Events:**

* Signaling a one-time occurrence (e.g., "initialization complete", "data ready").
* Broadcasting a state change to multiple threads.
* Implementing a simple shutdown mechanism for threads.

### 5. Barriers

A **Barrier** is a synchronization primitive that allows a specified number of threads to wait for each other at a common point (the barrier) before any of them are allowed to proceed.

* Each thread calls `Barrier.wait()`.
* It blocks until all `parties` (the number of threads specified when creating the barrier) have called `wait()`.
* Once the last thread calls `wait()`, all threads are unblocked simultaneously.
* The barrier can be reset for reuse or can be "broken" if a thread aborts or times out.

**Python: `threading.Barrier`**

```python
import threading
import time
import random

num_threads = 3
# The barrier will wait for num_threads + 1 (main thread) if main also waits,
# or just num_threads if main doesn't participate in this phase.
# Here, we make it for the worker threads.
barrier = threading.Barrier(parties=num_threads, timeout=5) # Optional timeout

def worker_phase(name, phase_time):
    print(f"Thread {name}: Starting phase 1, will take {phase_time}s.")
    time.sleep(phase_time)
    print(f"Thread {name}: Phase 1 complete. Reaching barrier.")
    try:
        # wait() returns an integer from 0 to parties-1 for one arbitrary thread,
        # and for other threads it's different.
        # This can be used for one thread to do some cleanup/setup after sync.
        serial_id = barrier.wait()
        if serial_id == 0: # Only one thread will get 0
            print(f"Thread {name} (serial_id {serial_id}): All threads reached barrier. Performing post-barrier action.")
        print(f"Thread {name}: Passed barrier. Starting phase 2.")
        time.sleep(random.uniform(0.5, 1))
        print(f"Thread {name}: Phase 2 complete.")
    except threading.BrokenBarrierError:
        print(f"Thread {name}: Barrier broken! Aborting.")
    except threading.TimeoutError:
        print(f"Thread {name}: Timed out waiting for barrier!")


if __name__ == "__main__":
    threads = []
    phase1_times = [1, 2, 3] # Different times to reach the barrier

    for i in range(num_threads):
        thread = threading.Thread(target=worker_phase, args=(f"T{i}", phase1_times[i]))
        threads.append(thread)
        thread.start()

    for thread in threads:
        thread.join()

    print(f"Barrier is_broken: {barrier.broken}")
    print("All workers finished.")

    # Example of breaking a barrier
    print("\n--- Scenario: Breaking the Barrier ---")
    barrier_to_break = threading.Barrier(parties=2, timeout=1)

    def break_worker():
        print("BreakWorker: Reaching barrier.")
        try:
            barrier_to_break.wait()
            print("BreakWorker: Passed barrier (should not happen if other aborts).")
        except threading.BrokenBarrierError:
            print("BreakWorker: Barrier was broken by another thread!")
        except threading.TimeoutError:
            print("BreakWorker: Timed out (should not happen here).")


    def abort_worker():
        print("AbortWorker: Reaching barrier then aborting.")
        # barrier_to_break.abort() # This immediately breaks the barrier for others.
        # Or, simply not calling wait() before timeout or other threads time out.
        # Let's simulate a timeout by one thread.
        # Here, we'll have one thread wait and the main thread won't join the barrier.
        # The barrier expects 2 parties. If only one calls wait, it will timeout.
        # If timeout occurs, the barrier becomes broken.
        try:
            barrier_to_break.wait() # This will timeout and then break.
        except threading.TimeoutError:
            print("AbortWorker: Timed out, barrier should now be broken.")
            # barrier_to_break.abort() # Can explicitly break it on timeout too
        except threading.BrokenBarrierError:
            print("AbortWorker: Barrier was already broken.")


    thread_b1 = threading.Thread(target=break_worker)
    thread_a1 = threading.Thread(target=abort_worker) # This one will cause a timeout

    thread_b1.start()
    thread_a1.start()

    thread_b1.join()
    thread_a1.join()
    print(f"Barrier is_broken: {barrier_to_break.broken}")

```

**Use Cases for Barriers:**

* Phased computations: When a task needs to be performed in stages, and all threads must complete one stage before any thread can move to the next.
* Ensuring all worker threads have initialized before starting the main work.
* Parallel algorithms that require synchronization points.

**Comparison of Synchronization Primitives Table**

| Primitive         | Primary Use                                     | Key Operations                     | Python Module      | Notes                                                       |
| :---------------- | :---------------------------------------------- | :--------------------------------- | :----------------- | :---------------------------------------------------------- |
| **Lock (Mutex)** | Mutual exclusion for critical sections          | `acquire()`, `release()`           | `threading.Lock`   | Protects shared data, prevents race conditions.             |
| **RLock** | Reentrant mutual exclusion                      | `acquire()`, `release()`           | `threading.RLock`  | Can be acquired multiple times by the same thread.          |
| **Semaphore** | Control access to N resources, signaling      | `acquire()`, `release()`           | `threading.Semaphore`, `threading.BoundedSemaphore` | Counter-based. BoundedSemaphore prevents counter > initial. |
| **Condition** | Wait for a condition to become true (with lock) | `wait()`, `notify()`, `notify_all()` | `threading.Condition`| Used with a lock, predicate checking in a loop is crucial.  |
| **Event** | Simple one-time signaling between threads       | `set()`, `clear()`, `wait()`, `is_set()` | `threading.Event`  | Manages an internal boolean flag.                           |
| **Barrier** | Synchronize multiple threads at a common point  | `wait()`, `reset()`, `abort()`     | `threading.Barrier`| All threads wait until `parties` count is reached.          |

---
## Higher-Level Concurrency Abstractions

While basic primitives are fundamental, modern programming languages and libraries offer higher-level abstractions to simplify concurrent programming.

### 1. Thread Pools

Creating and destroying threads frequently can be inefficient due to the overhead involved. A **thread pool** maintains a collection of pre-created worker threads that are ready to execute tasks.

* When a task arrives, it's submitted to the pool.
* An available thread from the pool picks up the task and executes it.
* If all threads are busy, the task might be queued until a thread becomes available.

**Benefits:**

* **Reduced Overhead**: Reuses existing threads.
* **Resource Management**: Limits the total number of concurrent threads, preventing resource exhaustion.
* **Simplified Task Submission**: Decouples task submission from thread management.

**Python: `concurrent.futures.ThreadPoolExecutor`**

```python
import concurrent.futures
import time
import random

def task_function(task_id, duration):
    print(f"Task {task_id}: Starting, will run for {duration:.2f} seconds.")
    time.sleep(duration)
    result = f"Task {task_id} completed successfully."
    print(f"Task {task_id}: Finished.")
    return result

if __name__ == "__main__":
    # Create a thread pool with a maximum of 3 worker threads
    with concurrent.futures.ThreadPoolExecutor(max_workers=3) as executor:
        futures = []
        task_durations = [random.uniform(0.5, 2.5) for _ in range(7)] # 7 tasks

        # Submit tasks to the pool
        for i, duration in enumerate(task_durations):
            # submit() returns a Future object
            future = executor.submit(task_function, f"ID-{i}", duration)
            futures.append(future)
            print(f"Main: Submitted task ID-{i}.")

        print("\nMain: All tasks submitted. Waiting for results...")

        # Retrieve results (optional, and can be done in order of submission or completion)
        # future.result() will block until the specific future is complete.
        for i, future in enumerate(futures):
            try:
                # You can add a timeout to result()
                # result_data = future.result(timeout=2)
                result_data = future.result()
                print(f"Main: Result for task ID-{i}: '{result_data}'")
            except concurrent.futures.TimeoutError:
                print(f"Main: Task ID-{i} timed out while waiting for result.")
            except Exception as e:
                print(f"Main: Task ID-{i} raised an exception: {e}")

    print("\nMain: ThreadPoolExecutor has shut down. All tasks processed.")

    # Edge case: Handling exceptions in tasks
    def failing_task():
        print("FailingTask: Starting, will raise an exception.")
        time.sleep(0.1)
        raise ValueError("Something went wrong in the task!")

    print("\n--- Example with a failing task ---")
    with concurrent.futures.ThreadPoolExecutor(max_workers=2) as executor:
        future1 = executor.submit(task_function, "GoodTask", 0.5)
        future_fail = executor.submit(failing_task)
        future2 = executor.submit(task_function, "AnotherGoodTask", 0.3)

        try:
            print(f"Result GoodTask: {future1.result()}")
        except Exception as e:
            print(f"Exception GoodTask: {e}")

        try:
            print(f"Result FailingTask: {future_fail.result()}") # This will re-raise the exception
        except ValueError as e:
            print(f"Caught expected exception from FailingTask: {e}")
        except Exception as e:
            print(f"Caught unexpected exception from FailingTask: {e}")

        # Check if exception occurred without calling result()
        if future_fail.exception() is not None:
            print(f"FailingTask indeed had an exception: {future_fail.exception()}")

        try:
            print(f"Result AnotherGoodTask: {future2.result()}")
        except Exception as e:
            print(f"Exception AnotherGoodTask: {e}")
```

**`concurrent.futures.ProcessPoolExecutor`**: Works similarly but uses a pool of processes instead of threads. This is suitable for CPU-bound tasks that can benefit from true parallelism across multiple CPU cores, as it bypasses the GIL. However, data passed to and from tasks must be picklable (serializable).

### 2. Task Queues (Work Queues)

A **task queue** (or work queue) is a data structure (often a queue) that stores tasks waiting to be processed. Producer threads add tasks to the queue, and consumer threads (often from a thread pool) retrieve and execute tasks from the queue.

This pattern helps decouple task production from task consumption and can handle backpressure (if the queue has a bounded size).

**Python: `queue.Queue`** (thread-safe)

```python
import queue
import threading
import time
import random

# A thread-safe queue
task_queue = queue.Queue(maxsize=5) # Bounded queue to demonstrate blocking
stop_event = threading.Event()

def producer_worker(name):
    for i in range(7): # Produce 7 tasks
        try:
            item = f"Task-{name}-{i}"
            print(f"Producer {name}: Trying to put '{item}' onto queue (Size: {task_queue.qsize()}).")
            # put() will block if the queue is full (if maxsize is reached)
            # Can use put(item, block=True, timeout=...)
            task_queue.put(item, timeout=2)
            print(f"Producer {name}: Successfully put '{item}'. (Size: {task_queue.qsize()})")
            time.sleep(random.uniform(0.1, 0.5))
        except queue.Full:
            print(f"Producer {name}: Queue is full. Failed to put '{item}'. Skipping.")
            break # Or handle differently
        except Exception as e:
            print(f"Producer {name}: Error {e}")
            break
    print(f"Producer {name} finished producing.")

def consumer_worker(name):
    while not stop_event.is_set() or not task_queue.empty():
        try:
            # get() will block if the queue is empty
            # Can use get(block=True, timeout=...)
            item = task_queue.get(timeout=1)
            print(f"Consumer {name}: Got '{item}'. Processing... (Remaining size: {task_queue.qsize()})")
            time.sleep(random.uniform(0.3, 1.0)) # Simulate work
            task_queue.task_done() # Signals that the formerly enqueued task is complete
            print(f"Consumer {name}: Finished processing '{item}'.")
        except queue.Empty:
            if stop_event.is_set():
                print(f"Consumer {name}: Queue empty and stop signaled. Exiting.")
                break
            # print(f"Consumer {name}: Queue is empty. Waiting...")
            # Continue loop to check stop_event again or try get() again
        except Exception as e:
            print(f"Consumer {name}: Error {e}")
            break # Or handle differently
    print(f"Consumer {name} finished consuming.")


if __name__ == "__main__":
    num_producers = 2
    num_consumers = 3

    producers = [threading.Thread(target=producer_worker, args=(f"P{i}",)) for i in range(num_producers)]
    consumers = [threading.Thread(target=consumer_worker, args=(f"C{i}",)) for i in range(num_consumers)]

    for p in producers: p.start()
    for c in consumers: c.start()

    # Wait for producers to finish their submissions
    for p in producers: p.join()
    print("Main: All producers have finished.")

    # Wait for all tasks in the queue to be processed
    # task_queue.join() waits until every item that was put() has had a task_done() called.
    print("Main: Waiting for queue to be empty (all tasks processed)...")
    task_queue.join()
    print("Main: Queue is empty.")

    # Signal consumers to stop
    stop_event.set()
    print("Main: Stop event set for consumers.")

    for c in consumers: c.join()
    print("Main: All consumers have finished.")
```

**`queue.PriorityQueue`**: Tasks are retrieved based on priority.
**`queue.LifoQueue`**: Last-In, First-Out queue.

### 3. Asynchronous Programming (Async/Await)

Asynchronous programming is a concurrency model that allows a single thread to manage multiple tasks by switching between them when one task performs a blocking operation (like I/O). It often uses an **event loop**.

* **`async`**: Defines a function as a coroutine. When called, it returns a coroutine object, but doesn't execute immediately.
* **`await`**: Pauses the execution of the current coroutine, allowing the event loop to run other tasks, until the awaited expression (typically another coroutine or an I/O operation) completes.

**Benefits:**

* **High Concurrency with Fewer Threads**: Can handle many concurrent I/O-bound operations efficiently within a single thread, avoiding the overhead of many threads.
* **Avoids Callback Hell**: `async/await` provides a more linear, synchronous-looking way to write asynchronous code compared to traditional callback-based approaches.
* Good for I/O-bound and high-level structured network code.

**Python: `asyncio` module**

```python
import asyncio
import time
import random

async def fetch_data(url, delay):
    print(f"Starting to fetch {url} (will take {delay}s)...")
    # In a real app, this would be an actual network request, e.g., with aiohttp
    # asyncio.sleep simulates a non-blocking I/O operation
    await asyncio.sleep(delay)
    print(f"Finished fetching {url}.")
    return f"Data from {url}"

async def process_data(data_id, raw_data):
    print(f"Starting to process data_id {data_id} ('{raw_data}')...")
    delay = random.uniform(0.2, 0.8)
    await asyncio.sleep(delay) # Simulate CPU-bound work (though in asyncio, long CPU work blocks event loop)
    processed_data = f"Processed {raw_data} (ID: {data_id})"
    print(f"Finished processing data_id {data_id}.")
    return processed_data

async def main_workflow():
    start_time = time.time()
    print("Main workflow started.")

    # Create tasks to run concurrently
    # These tasks will be scheduled on the event loop
    task1 = asyncio.create_task(fetch_data("http://example.com/api/data1", 1.5))
    task2 = asyncio.create_task(fetch_data("http://example.com/api/data2", 1.0))
    task3 = asyncio.create_task(fetch_data("http://example.com/api/data3", 2.0))

    # Wait for all fetching tasks to complete
    # await task1
    # await task2
    # await task3
    # results = [task1.result(), task2.result(), task3.result()]

    # Or, using asyncio.gather to run them concurrently and wait for all
    print("Awaiting fetch_data tasks...")
    results = await asyncio.gather(task1, task2, task3)
    # results = await asyncio.gather(
    #     fetch_data("http://example.com/api/data1", 1.5),
    #     fetch_data("http://example.com/api/data2", 1.0),
    #     fetch_data("http://example.com/api/data3", 2.0)
    # )
    print(f"All data fetched: {results}")

    processing_tasks = []
    for i, data in enumerate(results):
        ptask = asyncio.create_task(process_data(i, data))
        processing_tasks.append(ptask)

    print("Awaiting process_data tasks...")
    final_outputs = await asyncio.gather(*processing_tasks) # Unpack list into arguments
    print(f"All data processed: {final_outputs}")

    end_time = time.time()
    print(f"Main workflow completed in {end_time - start_time:.2f} seconds.")
    # Total time should be close to the longest individual path, not sum of all delays.

# To run an asyncio program
if __name__ == "__main__":
    # In Python 3.7+
    asyncio.run(main_workflow())

    # Edge case: Handling exceptions in asyncio tasks
    async def failing_coroutine():
        print("FailingCoroutine: Starting, will raise after 0.5s")
        await asyncio.sleep(0.5)
        raise ValueError("Something went wrong in the coroutine!")

    async def another_main():
        print("\n--- Asyncio Exception Handling ---")
        task_good = asyncio.create_task(fetch_data("good_url", 0.2))
        task_bad = asyncio.create_task(failing_coroutine())
        task_another_good = asyncio.create_task(fetch_data("another_good_url", 0.3))

        # Option 1: Await individually and try-except
        try:
            await task_bad
        except ValueError as e:
            print(f"Caught expected exception from task_bad: {e}")

        # Option 2: Use asyncio.gather with return_exceptions=True
        # This prevents gather from stopping on the first exception and returns
        # exception objects in place of results for failed tasks.
        all_results_including_exceptions = await asyncio.gather(
            task_good,
            failing_coroutine(), # Re-create if already awaited and raised
            task_another_good,
            return_exceptions=True
        )
        for res in all_results_including_exceptions:
            if isinstance(res, Exception):
                print(f"Gathered result was an exception: {res}")
            else:
                print(f"Gathered result: {res}")

    if __name__ == "__main__":
        asyncio.run(another_main())
```

**When to use `asyncio`:**

* Primarily for I/O-bound operations (networking, file system access where async drivers exist).
* When you need to manage a very large number of concurrent connections with minimal resource usage per connection.
* **Not suitable for CPU-bound tasks** in a single `asyncio` event loop, as these will block the loop and prevent other tasks from running. CPU-bound work should be offloaded to separate threads (using `loop.run_in_executor`) or separate processes.

---
## Concurrency vs. Parallelism

These terms are often used interchangeably, but they have distinct meanings:

* **Concurrency**: Dealing with multiple things at once. It's about the structure of the program. Concurrent tasks can *appear* to run simultaneously by interleaving their execution on a single CPU core (time-slicing) or can actually run simultaneously if multiple cores are available. The key is that the program is designed to manage multiple flows of control.
    * Example: A web server handling multiple client requests. On a single-core CPU, it rapidly switches between requests.

* **Parallelism**: Doing multiple things at once. It's about the execution of the program. Parallelism requires hardware with multiple processing units (e.g., multi-core CPU, GPU). Tasks are genuinely running at the same physical time.
    * Example: A video encoding program splitting a video into chunks and processing each chunk simultaneously on different CPU cores.

**Relationship:**

* Concurrency is a way to structure a program; parallelism is a way to execute it.
* You can have concurrency **without** parallelism (e.g., multiple threads on a single-core CPU).
* Parallelism typically implies concurrency (if multiple tasks are running at the same time, the program needs to be structured to manage them).
* The goal of concurrent programming is often to enable parallelism if hardware supports it, or to improve responsiveness and resource utilization even if it doesn't.

| Feature        | Concurrency                                      | Parallelism                                       |
| :------------- | :----------------------------------------------- | :------------------------------------------------ |
| **Definition** | Managing multiple tasks over a period of time    | Executing multiple tasks at the same instant      |
| **CPU Cores** | Can occur on a single core (interleaving) or multi-core | Requires multiple cores/processors                |
| **Goal** | Responsiveness, structure, resource utilization  | Speed up computation via simultaneous execution   |
| **Example** | A GUI app downloading files while UI is active.  | A scientific simulation running on a supercomputer. |

**Python's Global Interpreter Lock (GIL) Impact:**

* For **threads** in CPython, the GIL ensures that only one thread executes Python bytecode at any given moment within a single process.
    * This means threads in CPython achieve **concurrency** (interleaving, especially effective for I/O-bound tasks where the GIL is released during I/O waits) but **not true parallelism** for CPU-bound Python code on multi-core machines within one process.
* For **processes** (`multiprocessing` module), each process has its own Python interpreter and its own GIL.
    * This means `multiprocessing` in Python can achieve **true parallelism** for CPU-bound tasks on multi-core machines.
* **`asyncio`** achieves concurrency on a single thread by cooperative multitasking. It doesn't inherently provide parallelism for CPU-bound tasks unless combined with `multiprocessing` or offloading to threads.

---
## Concurrency Models

Different approaches to how concurrent units (threads, processes, actors, etc.) interact and share/exchange data.

### 1. Shared Memory Model

* Concurrent units (typically threads within the same process) communicate by reading and writing to shared memory locations (variables, data structures).
* Synchronization primitives (mutexes, semaphores, condition variables) are crucial to protect shared data from race conditions and ensure orderly access.
* **Pros**:
    * Fast communication (direct memory access).
    * Familiar programming model for many developers.
* **Cons**:
    * Prone to errors like race conditions, deadlocks.
    * Synchronization can be complex to get right.
    * Scalability can be limited by contention for shared resources and locks.
* **Examples**: Threads in Java, C++, Python (`threading`), POSIX threads.

```python
# (Re-showing a snippet from earlier to illustrate shared memory)
import threading

shared_counter = 0
lock = threading.Lock() # Protects shared_counter

def increment():
    global shared_counter
    for _ in range(100000):
        with lock: # Critical section
            shared_counter += 1

# ... (rest of the threading setup)
```

### 2. Message Passing Model

* Concurrent units (can be processes or specialized entities like actors) communicate by sending and receiving messages.
* Each unit has its own private state; there is no directly shared memory between them.
* Messages are typically immutable or copied when sent.
* **Pros**:
    * Avoids many shared-memory issues (race conditions on shared state are less common if state is truly isolated).
    * Often leads to more robust and easier-to-reason-about concurrent systems. "Share memory by communicating, don't communicate by sharing memory."
    * Can scale well across distributed systems (where processes are on different machines).
* **Cons**:
    * Message passing can have higher overhead than direct memory access (serialization, copying, context switching).
    * Can lead to different kinds of complexity (e.g., managing mailboxes, message ordering, potential for deadlocks if A waits for B's message and B waits for A's).
* **Examples**:
    * Processes using IPC (pipes, sockets) in Python (`multiprocessing` with `Pipe` or `Queue`).
    * Actor model (Erlang, Akka for Scala/Java, `Pykka` or `Thespian` in Python).
    * Goroutines and channels in Go.
    * MPI (Message Passing Interface) for high-performance computing.

**Python Example: Message Passing with `multiprocessing.Queue`**

```python
import multiprocessing
import time
import random

def producer(queue, num_messages):
    process_name = multiprocessing.current_process().name
    for i in range(num_messages):
        message = f"Msg from {process_name}: {i}"
        print(f"{process_name}: Sending '{message}'")
        queue.put(message) # Send message
        time.sleep(random.uniform(0.1, 0.3))
    queue.put("DONE") # Sentinel value to signal end
    print(f"{process_name}: Finished sending.")

def consumer(queue):
    process_name = multiprocessing.current_process().name
    while True:
        try:
            message = queue.get(timeout=5) # Receive message with timeout
            if message == "DONE":
                print(f"{process_name}: Received DONE. Exiting.")
                # If multiple producers, need a more robust termination signal
                # or put back DONE for other consumers if applicable.
                # For simplicity here, one consumer stops after one DONE.
                break
            print(f"{process_name}: Received '{message}'")
            # Process message
            time.sleep(random.uniform(0.2, 0.5))
        except multiprocessing.queues.Empty: # In Python 3.8+, it's queue.Empty
            print(f"{process_name}: Queue empty, timed out. Still running...")
            # Continue if expecting more messages, or break based on logic
        except Exception as e:
            print(f"{process_name}: Error - {e}")
            break
    print(f"{process_name}: Finished consuming.")


if __name__ == "__main__":
    # Queues from multiprocessing are process-safe
    message_queue = multiprocessing.Queue()

    producer_process = multiprocessing.Process(target=producer, args=(message_queue, 5), name="Producer-1")
    consumer_process = multiprocessing.Process(target=consumer, args=(message_queue,), name="Consumer-1")

    print("Main: Starting processes.")
    producer_process.start()
    consumer_process.start()

    producer_process.join()
    print("Main: Producer process joined.")
    consumer_process.join()
    print("Main: Consumer process joined.")

    print("Main: All processes finished.")
```

### 3. Other Models/Variations

* **Communicating Sequential Processes (CSP)**: A formal language for describing patterns of interaction in concurrent systems. Go's channels are inspired by CSP.
* **Software Transactional Memory (STM)**: A concurrency control mechanism analogous to database transactions for controlling access to shared memory. Operations are grouped into atomic transactions. If a conflict occurs, a transaction might abort and retry. Less common in mainstream languages but an active research area and available in some libraries/languages (e.g., Clojure).
* **Dataflow Programming**: Program is modeled as a directed graph of data flowing between operations. Operations run when their inputs are available.

**Comparison of Concurrency Models**

| Feature             | Shared Memory                                 | Message Passing                                      |
| :------------------ | :-------------------------------------------- | :--------------------------------------------------- |
| **State** | Shared among threads/units                    | Private to each unit/process/actor                   |
| **Communication** | Reading/writing shared variables              | Sending/receiving explicit messages                  |
| **Synchronization** | Explicit (locks, semaphores, etc.)           | Implicit in message send/receive, or explicit (e.g., waiting for reply) |
| **Error Proneness** | Higher risk of race conditions, deadlocks     | Lower risk for shared state issues; different deadlock types |
| **Overhead** | Lower for data access, higher for sync        | Higher for message passing, lower for sync logic     |
| **Scalability** | Can be limited by lock contention             | Can scale well, especially distributed               |
| **Suitability** | Tightly coupled tasks, performance-critical sections | Loosely coupled tasks, distributed systems, robust services |

---
## Common Concurrency Patterns

Design patterns that provide reusable solutions to common problems in concurrent programming.

### 1. Producer-Consumer

* One or more **producer** threads/processes create data items (tasks, messages, etc.) and put them into a shared buffer (often a queue).
* One or more **consumer** threads/processes take items from the buffer and process them.
* The buffer acts as a synchronization point and can help smooth out variations in production and consumption rates.
* **Synchronization**:
    * The buffer (queue) itself needs to be thread-safe.
    * Condition variables are often used to signal "buffer not empty" (for consumers) and "buffer not full" (for producers, if bounded).

*We already saw this pattern implemented with `threading.Condition` and `queue.Queue`.*

**Key challenges/considerations:**

* **Bounded vs. Unbounded Buffer**:
    * **Bounded**: Prevents producers from getting too far ahead of consumers and exhausting memory. Requires producers to wait if full.
    * **Unbounded**: Simpler for producers (they never block on put), but can lead to memory issues if consumers are slow.
* **Backpressure**: How to handle a full buffer (block producer, drop messages, etc.).
* **Graceful Shutdown**: Ensuring producers stop and consumers process all remaining items before exiting.

### 2. Reader-Writer Lock (Shared/Exclusive Lock)

* Allows multiple threads (**readers**) to read a shared resource concurrently.
* Allows only one thread (**writer**) to modify the resource exclusively. While a writer holds the lock, no other readers or writers can access it.
* Useful when reads are frequent and writes are infrequent, improving concurrency over a simple mutex that would serialize all access (reads and writes).

**Python: No direct `RWLock` in `threading` module.** It can be implemented using `threading.Condition` and a `threading.Lock`, or by using third-party libraries.

**Conceptual Implementation Sketch (Python-like pseudocode):**

```python
# Conceptual RWLock (not a complete, robust implementation)
import threading

class RWLock:
    def __init__(self):
        self._lock = threading.Lock() # Main lock for internal state
        self._read_condition = threading.Condition(self._lock)
        self._write_condition = threading.Condition(self._lock) # Could use a single condition with more logic
        self._readers = 0
        self._writer_active = False
        self._writers_waiting = 0 # For writer preference

    def acquire_read(self):
        with self._lock:
            # Wait if a writer is active or writers are waiting (writer preference)
            while self._writer_active or self._writers_waiting > 0:
                self._read_condition.wait()
            self._readers += 1

    def release_read(self):
        with self._lock:
            self._readers -= 1
            if self._readers == 0 and self._writers_waiting > 0:
                # If no readers left and writers are waiting, notify a writer
                self._write_condition.notify()

    def acquire_write(self):
        with self._lock:
            self._writers_waiting += 1
            # Wait if readers are active or another writer is active
            while self._readers > 0 or self._writer_active:
                self._write_condition.wait()
            self._writers_waiting -= 1
            self._writer_active = True

    def release_write(self):
        with self._lock:
            self._writer_active = False
            # Notify waiting writers first (preference), then readers
            if self._writers_waiting > 0:
                self._write_condition.notify()
            else:
                self._read_condition.notify_all() # Notify all waiting readers

# Example Usage (Conceptual)
# shared_data = {}
# rw_lock = RWLock()

# def reader_thread():
#     rw_lock.acquire_read()
#     try:
#         # Read from shared_data
#         pass
#     finally:
#         rw_lock.release_read()

# def writer_thread():
#     rw_lock.acquire_write()
#     try:
#         # Write to shared_data
#         pass
#     finally:
#         rw_lock.release_write()
```

**Challenges with RWLocks:**

* **Starvation**: Writers might starve if there's a continuous stream of readers, or readers might starve if writers are prioritized and frequent. Policies (writer-preference, reader-preference, fair) can mitigate this but add complexity.
* **Complexity**: More complex to implement correctly than a simple mutex.

### 3. Master-Worker (or Boss-Workers)

* A **master** (or boss) thread/process receives or generates tasks.
* It distributes these tasks to a pool of **worker** threads/processes.
* Workers process their assigned tasks independently.
* Optionally, workers might send results back to the master or a collector.
* This is a common pattern for parallelizing work. `ThreadPoolExecutor` and `ProcessPoolExecutor` are high-level implementations of this.

**Key elements:**

* Task distribution mechanism (e.g., a shared queue).
* Worker pool management.
* Result collection mechanism (optional).

```python
# (Re-showing ThreadPoolExecutor which embodies this pattern)
import concurrent.futures
import time

def worker_function(data):
    # print(f"Worker {threading.get_ident()}: processing {data}")
    time.sleep(0.5) # Simulate work
    return data * 2

if __name__ == "__main__":
    tasks_to_do = [1, 2, 3, 4, 5, 6, 7, 8]
    results = []

    # The ThreadPoolExecutor is the "master" managing worker threads
    with concurrent.futures.ThreadPoolExecutor(max_workers=4) as executor:
        # Master submits tasks
        future_to_data = {executor.submit(worker_function, task): task for task in tasks_to_do}

        # Master collects results as they complete
        for future in concurrent.futures.as_completed(future_to_data):
            data_item = future_to_data[future]
            try:
                result = future.result()
                print(f"Master: Worker processed {data_item}, got {result}")
                results.append(result)
            except Exception as exc:
                print(f"Master: Worker for {data_item} generated an exception: {exc}")

    print(f"Master: All tasks completed. Results: {results}")
```

### 4. Pipeline Pattern

* A series of processing stages, where the output of one stage becomes the input for the next.
* Each stage can be a separate thread or process.
* Data flows through the pipeline.
* Queues are often used to connect stages.

**Example: Image Processing Pipeline**
Stage 1: Load image (Thread 1) -> Queue 1
Stage 2: Resize image (Thread 2, takes from Queue 1) -> Queue 2
Stage 3: Apply filter (Thread 3, takes from Queue 2) -> Queue 3
Stage 4: Save image (Thread 4, takes from Queue 3)

**Benefits**:
* Allows for parallel processing of different items at different stages simultaneously.
* Modular design.

```python
import threading
import queue
import time
import random

# Define queues for each stage
queue1 = queue.Queue(maxsize=5) # Raw data
queue2 = queue.Queue(maxsize=5) # Stage 1 processed data
queue3 = queue.Queue(maxsize=5) # Stage 2 processed data

stop_signal_stage1 = object() # Sentinel for stage 1
stop_signal_stage2 = object() # Sentinel for stage 2
stop_signal_stage3 = object() # Sentinel for stage 3

def stage1_load_data(output_queue):
    for i in range(10): # Simulate loading 10 data items
        data = f"RawData-{i}"
        print(f"Stage 1: Loaded '{data}'")
        output_queue.put(data)
        time.sleep(random.uniform(0.1, 0.3))
    output_queue.put(stop_signal_stage1) # Signal end of data for this stage
    print("Stage 1: Finished loading all data.")

def stage2_process_data(input_queue, output_queue):
    while True:
        data = input_queue.get()
        if data is stop_signal_stage1: # Check for sentinel
            print("Stage 2: Received stop signal. Propagating.")
            output_queue.put(stop_signal_stage2) # Propagate stop signal
            input_queue.task_done()
            break
        processed_data = f"{data}-ProcessedStage2"
        print(f"Stage 2: Processed into '{processed_data}'")
        output_queue.put(processed_data)
        input_queue.task_done()
        time.sleep(random.uniform(0.2, 0.5))
    print("Stage 2: Finished processing.")

def stage3_finalize_data(input_queue):
    while True:
        data = input_queue.get()
        if data is stop_signal_stage2: # Check for sentinel
            print("Stage 3: Received stop signal.")
            input_queue.task_done()
            break
        final_data = f"{data}-FinalizedStage3"
        print(f"Stage 3: Finalized as '{final_data}'")
        # Here you might save to disk, send over network, etc.
        input_queue.task_done()
        time.sleep(random.uniform(0.1, 0.4))
    print("Stage 3: Finished finalizing.")

if __name__ == "__main__":
    thread1 = threading.Thread(target=stage1_load_data, args=(queue1,))
    thread2 = threading.Thread(target=stage2_process_data, args=(queue1, queue2))
    thread3 = threading.Thread(target=stage3_finalize_data, args=(queue2,))

    print("Starting pipeline...")
    thread1.start()
    thread2.start()
    thread3.start()

    thread1.join()
    print("Pipeline: Stage 1 thread joined.")
    # queue1.join() # Wait for stage 1 to fully process items before stage 2 finishes (if needed)
    thread2.join()
    print("Pipeline: Stage 2 thread joined.")
    # queue2.join() # Wait for stage 2 to fully process items
    thread3.join()
    print("Pipeline: Stage 3 thread joined.")

    print("Pipeline processing complete.")
```
*Careful handling of stop signals/sentinel values is crucial for pipeline shutdown.*

---
## Debugging and Testing Concurrent Applications

Debugging concurrent programs is notoriously harder than sequential ones due to:

* **Non-determinism**: Bugs may appear sporadically depending on thread scheduling and timing.
* **Heisenbugs**: Bugs that alter their behavior or disappear when you try to observe them (e.g., adding print statements can change timing).
* **Difficulty in Reproduction**: Race conditions or deadlocks might only occur under specific load or timing conditions.

### Common Pitfalls:

1.  **Forgetting to Lock**: Accessing shared mutable data without any synchronization.
2.  **Incorrect Lock Granularity**:
    * **Too coarse**: Locking too much code reduces concurrency.
    * **Too fine**: More complex, higher risk of missing locks or causing deadlocks.
3.  **Deadlocks**: Circular dependencies between locks.
4.  **Race Conditions**: Incorrect assumptions about atomicity or order of operations.
5.  **Starvation/Livelocks**.
6.  **Not Releasing Locks**: Especially in error paths if not using `try...finally` or context managers (`with` statement).
7.  **Incorrect Use of Condition Variables**:
    * Not using a `while` loop for the predicate check.
    * Not holding the associated lock when calling `wait()`, `notify()`, `notify_all()`.
8.  **Assumptions about Thread Scheduling**: Never assume threads will run in a particular order or at a particular speed.
9.  **Ignoring Exceptions in Threads**: Unhandled exceptions in a thread can terminate the thread silently (or crash the app if it's the main thread or not handled properly by a framework). `Future` objects in `concurrent.futures` help capture these.

### Techniques and Tools:

1.  **Code Reviews**: Having another pair of eyes, especially experienced in concurrency, review the code is invaluable.
2.  **Static Analysis Tools**: Some linters and static analyzers can detect potential concurrency issues (e.g., potential data races, misuse of APIs).
3.  **Logging**:
    * Use comprehensive logging with thread IDs and timestamps.
    * Be careful, as excessive logging can alter timing and hide/show bugs.
4.  **Debugging Tools**:
    * **Standard Debuggers (pdb, IDE debuggers)**: Can be used, but stepping through concurrent code can be tricky as threads switch. Setting breakpoints in multiple threads can help.
    * **Thread-Aware Debuggers**: Some debuggers have better support for inspecting thread states, stacks, and switching between threads.
    * **Specialized Concurrency Debuggers/Profilers**: (Often language/platform specific)
        * **Helgrind** (Valgrind tool): Detects synchronization errors in C/C++ like misuse of pthreads primitives, potential deadlocks, data races.
        * **ThreadSanitizer (TSan)**: A data race detector for C/C++, Go, Swift, Rust.
        * Java has tools like JConsole, VisualVM, and commercial profilers (JProfiler, YourKit) that can analyze thread dumps, detect deadlocks, and monitor thread activity.
5.  **Testing Strategies**:
    * **Unit Tests**: Test individual components, mocking dependencies. For concurrent components, test their behavior under specific interleavings if possible (hard).
    * **Integration Tests**: Test interactions between concurrent components.
    * **Stress Testing**: Run the application under high load and for extended periods to try and provoke latent concurrency bugs. Vary the number of threads, request rates, etc.
    * **Chaos Engineering**: Intentionally inject faults or delays to see how the system behaves.
    * **Deterministic Testing (where possible)**: Some frameworks or techniques allow for more controlled execution of concurrent tasks for testing (e.g., simulating an event loop, controlling a mock scheduler).
6.  **Use Proven Libraries and Abstractions**:
    * Prefer high-level abstractions like thread pools, async/await, and concurrent data structures over manual lock management where possible. These are often written and tested by experts.
    * Python's `queue.Queue`, `concurrent.futures`, `asyncio` are good examples.
7.  **Simplify Design**:
    * Minimize shared mutable state. Immutable data is your friend in concurrency.
    * Clear separation of concerns.
8.  **Idempotent Operations**: Design operations such that if they run multiple times due to retries (common in distributed/concurrent systems), they don't cause incorrect state.
9.  **Timeouts**: Use timeouts for lock acquisitions, I/O operations, and inter-thread/process communication to prevent indefinite blocking.

**Python Example: Capturing Exceptions from Threads**

```python
import threading
import time

class StoppableThread(threading.Thread):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self._exception = None
        self._stop_event = threading.Event()

    def run(self):
        try:
            # The original target function is called here if provided via constructor
            if self._target:
                self._target(*self._args, **self._kwargs)
        except Exception as e:
            self._exception = e # Catch and store the exception
        finally:
            # Clean-up actions can be performed here
            pass

    def join(self, timeout=None):
        super().join(timeout)
        if self._exception:
            # Re-raise the exception in the context of the calling thread
            raise self._exception

    def stop(self):
        self._stop_event.set()

    def stopped(self):
        return self._stop_event.is_set()

def my_buggy_function():
    print("Buggy function: Starting...")
    time.sleep(0.5)
    raise ValueError("This is a deliberate bug in the thread!")
    print("Buggy function: This will not be printed.") # Unreachable

def my_long_running_function(stop_event_ref):
    print("Long function: Starting...")
    count = 0
    while not stop_event_ref.is_set(): # Check the stop event
        print(f"Long function: Working... {count}")
        time.sleep(0.5)
        count += 1
        if count == 5:
            # Let's also simulate an error condition in a stoppable thread
            # raise RuntimeError("Simulated error in stoppable thread")
            pass # Keep it clean for this example
    print("Long function: Stop event received or loop finished. Exiting.")


if __name__ == "__main__":
    print("--- Handling exceptions in custom thread ---")
    buggy_thread = StoppableThread(target=my_buggy_function)
    buggy_thread.start()

    try:
        buggy_thread.join()
        print("Buggy thread finished without error (unexpected).")
    except ValueError as e:
        print(f"Main thread caught error from buggy_thread: {e}")
    except Exception as e:
        print(f"Main thread caught UNEXPECTED error from buggy_thread: {e}")

    print("\n--- Stoppable thread example ---")
    stoppable_t = StoppableThread(target=my_long_running_function, args=(lambda: stoppable_t._stop_event,))
    stoppable_t.start()

    time.sleep(2.2) # Let it run for a bit
    print("Main thread: Signaling stoppable_t to stop.")
    stoppable_t.stop()

    try:
        stoppable_t.join(timeout=2) # Wait for it to finish
        if stoppable_t.is_alive():
            print("Main thread: Stoppable thread did not terminate in time.")
        else:
            print("Main thread: Stoppable thread terminated gracefully.")
    except Exception as e: # If join re-raises an exception
        print(f"Main thread caught error from stoppable_t: {e}")

    # Using concurrent.futures.Future for exception handling (preferred)
    from concurrent.futures import ThreadPoolExecutor

    def another_buggy_task():
        print("Another buggy task: starting")
        time.sleep(0.1)
        raise TypeError("Bug in future task!")

    print("\n--- Handling exceptions with ThreadPoolExecutor ---")
    with ThreadPoolExecutor(max_workers=1) as executor:
        future = executor.submit(another_buggy_task)
        try:
            result = future.result() # This will re-raise the exception
            print(f"Result from future: {result}")
        except TypeError as e:
            print(f"Main thread caught error from future: {e}")
        except Exception as e:
            print(f"Main thread caught UNEXPECTED error from future: {e}")

        if future.done() and future.exception():
            print(f"Future experienced an exception: {future.exception()}")
```

---
## Advanced Concurrency Topics

### 1. Lock-Free Data Structures and Algorithms

* Data structures and algorithms designed to allow concurrent access and modification by multiple threads **without using locks**.
* They rely on low-level atomic hardware instructions like **Compare-And-Swap (CAS)**, Fetch-And-Add, etc.
* **CAS(memory_location, expected_value, new_value)**: Atomically checks if `memory_location` holds `expected_value`. If so, it updates it to `new_value` and returns true/old_value. Otherwise, it does nothing and returns false/old_value. This is a fundamental building block.
* **Benefits**:
    * Can offer better scalability and performance by avoiding lock contention, especially on multi-core systems.
    * Avoid deadlocks caused by locks.
    * More resilient to thread failures (a thread dying while holding a lock can be catastrophic).
* **Challenges**:
    * Extremely difficult to design and implement correctly. Subtle bugs are common.
    * ABA problem: A location reads value A, then another thread changes it to B and then back to A. The first thread's CAS might wrongly succeed. (Solutions involve version counters or tags).
    * Often more complex logic than lock-based counterparts.
    * Performance isn't always better, especially if contention is low or the lock-free algorithm is complex.
* **Examples**: Lock-free queues, stacks, lists, hash maps. Many concurrent data structures in Java's `java.util.concurrent` (like `ConcurrentLinkedQueue`, `AtomicInteger`) are implemented using lock-free techniques.

**Conceptual CAS loop (Python doesn't have direct CAS on arbitrary objects like this for threads, but `AtomicInteger` in Java is an example):**

```python
# Pseudocode for an atomic increment using CAS
# class AtomicInteger:
#     def __init__(self, value=0):
#         self._value = value # Assume this is an atomic type or handled by HW
#
#     def incrementAndGet(self):
#         while True:
#             current_value = self._value # Atomic read
#             next_value = current_value + 1
#             # if CAS(&self._value, current_value, next_value) succeeds:
#             #    return next_value
#             # If CAS fails, it means another thread changed self._value. Loop and retry.

# Python's `multiprocessing.Value` can be used with a lock for atomic-like ops,
# but true CAS at the Python language level for user-defined objects in threads isn't standard.
# Libraries like `atomics` exist for CPython providing access to CPU atomic operations.
```

### 2. Transactional Memory (TM)

* **Software Transactional Memory (STM)**: A concurrency control mechanism that allows programmers to define blocks of code (transactions) that should execute atomically.
* The TM system manages reads and writes within a transaction. If two transactions conflict (e.g., one writes what another read or wrote), one or both might be aborted and retried.
* **Goal**: Simplify concurrent programming by providing higher-level atomicity, similar to database transactions, without manual locking.
* **Pros**:
    * Can be easier to compose than locks.
    * Potentially avoids deadlocks associated with locks (though livelocks from repeated aborts are possible).
* **Cons**:
    * Performance overhead can be significant.
    * Not widely adopted in mainstream languages as a primary concurrency mechanism (Clojure is a notable exception with strong STM support).
    * Interactions with non-transactional operations (e.g., I/O) can be complex.
* **Hardware Transactional Memory (HTM)**: Some modern CPUs (e.g., Intel TSX) provide hardware support for transactional execution, which can reduce STM overhead.

### 3. Language-Specific Concurrency Features

* **Go: Goroutines and Channels**
    * **Goroutines**: Extremely lightweight, concurrently executing functions. Managed by the Go runtime, not directly by OS threads (M:N scheduling). Thousands or millions can be created.
    * **Channels**: Typed conduits through which you can send and receive values with the `<-` operator. Used to communicate and synchronize between goroutines (message passing).
    * **`select` statement**: Allows a goroutine to wait on multiple channel operations.
    * Promotes a CSP-like concurrency model ("Do not communicate by sharing memory; instead, share memory by communicating.").

    ```go
    // package main
    // import ("fmt"; "time")

    // func worker(id int, jobs <-chan int, results chan<- int) {
    //     for j := range jobs {
    //         fmt.Println("worker", id, "started job", j)
    //         time.Sleep(time.Second) // Simulate work
    //         fmt.Println("worker", id, "finished job", j)
    //         results <- j * 2
    //     }
    // }

    // func main() {
    //     numJobs := 5
    //     jobs := make(chan int, numJobs)
    //     results := make(chan int, numJobs)

    //     for w := 1; w <= 3; w++ { // Start 3 workers
    //         go worker(w, jobs, results)
    //     }

    //     for j := 1; j <= numJobs; j++ { // Send jobs
    //         jobs <- j
    //     }
    //     close(jobs) // Close jobs channel when all jobs sent

    //     for a := 1; a <= numJobs; a++ { // Collect results
    //         <-results
    //     }
    //     fmt.Println("All jobs done.")
    // }
    ```

* **Erlang/Elixir: Actor Model**
    * **Actors**: Isolated processes (lightweight, not OS processes) that communicate solely via asynchronous messages. Each actor has a private state and a mailbox for incoming messages.
    * "Let it crash" philosophy: Actors are often supervised. If an actor crashes, a supervisor can restart it or take other corrective actions.
    * Excellent for building highly concurrent, fault-tolerant, and distributed systems.

    ```elixir
    # defmodule MyActor do
    #   use GenServer # A common OTP behavior for actors

    #   # Client API
    #   def start_link(initial_state) do
    #     GenServer.start_link(__MODULE__, initial_state, name: __MODULE__)
    #   end

    #   def add(pid, number) do
    #     GenServer.call(pid, {:add, number}) # Synchronous call
    #   end

    #   def get_value(pid) do
    #     GenServer.call(pid, :get_value)
    #   end

    #   # Server Callbacks
    #   @impl true
    #   def init(initial_state) do
    #     {:ok, initial_state} # Actor's initial private state
    #   end

    #   @impl true
    #   def handle_call({:add, number}, _from, state) do
    #     new_state = state + number
    #     {:reply, new_state, new_state} # Reply, new state
    #   end

    #   @impl true
    #   def handle_call(:get_value, _from, state) do
    #     {:reply, state, state}
    #   end
    # end

    # # Usage
    # # {:ok, pid} = MyActor.start_link(10)
    # # MyActor.add(pid, 5) # New state is 15
    # # current = MyActor.get_value(pid) # current is 15
    ```

* **Java: `java.util.concurrent` package, Project Loom (Virtual Threads)**
    * Rich set of concurrent utilities: executors, concurrent collections (e.g., `ConcurrentHashMap`), synchronizers (Semaphore, CountDownLatch, CyclicBarrier), atomic variables.
    * **Project Loom (Virtual Threads)**: Introduced in recent Java versions. Lightweight threads managed by the JVM, not directly mapped 1:1 to OS threads. Allows for a very high number of concurrent tasks with a more traditional imperative programming style, reducing the need for reactive/async styles for I/O-bound scalability.

* **C#: `async/await`, Task Parallel Library (TPL), `System.Collections.Concurrent`**
    * Strong support for asynchronous programming (`async/await` similar to Python's).
    * TPL provides `Task` objects (similar to Python's Futures), thread pools, parallel loops (`Parallel.For`, `Parallel.ForEach`), PLINQ (Parallel LINQ).
    * Concurrent collections designed for thread-safe access.

* **Rust: Ownership and Borrowing, `async/await`, Fearless Concurrency**
    * Rust's ownership and borrowing system helps prevent data races at compile time for many scenarios.
    * `Send` and `Sync` traits mark types that can be safely transferred or shared between threads.
    * `std::thread` for OS threads, `async/await` for asynchronous programming (often with runtimes like Tokio or async-std).
    * Message passing via channels (`std::sync::mpsc`).

### 4. Structured Concurrency

* A programming paradigm for concurrent programming that aims to make concurrent code easier to reason about and more robust by ensuring that the lifetime of concurrent tasks is tied to a well-defined scope.
* If a scope is exited (e.g., a function returns or an exception is thrown), all concurrent tasks spawned within that scope are guaranteed to be completed or cancelled.
* Helps prevent "leaked" threads/tasks and simplifies error handling and resource management.
* **Python**: Libraries like `trio` and `anyio` promote structured concurrency. `asyncio.TaskGroup` (Python 3.11+) also brings structured concurrency features.

```python
# Example with asyncio.TaskGroup (Python 3.11+)
import asyncio

async def do_work(name, delay):
    print(f"Task {name}: Starting work for {delay}s")
    await asyncio.sleep(delay)
    if name == "B":
        raise ValueError(f"Task {name} encountered an error!")
    print(f"Task {name}: Finished work")
    return f"Result from {name}"

async def structured_main():
    print("Starting structured_main")
    try:
        # All tasks in the group are managed together.
        # If one task in the group raises an unhandled exception,
        # all other tasks in the group are cancelled.
        # The TaskGroup context manager then re-raises the exception (or an ExceptionGroup).
        async with asyncio.TaskGroup() as tg:
            task_a = tg.create_task(do_work("A", 1))
            task_b = tg.create_task(do_work("B", 0.5)) # This will fail
            task_c = tg.create_task(do_work("C", 1.5)) # This will be cancelled if B fails first

            print("Tasks created in TaskGroup.")
            # Results can be accessed from the task objects, but await implicitly happens
            # at the end of the 'async with' block.
    except* ValueError as eg: # Python 3.11+ syntax for ExceptionGroup
        print(f"Caught an ExceptionGroup with {len(eg.exceptions)} error(s):")
        for exc in eg.exceptions:
            print(f" - {type(exc).__name__}: {exc}")
    except Exception as e:
        print(f"Caught a single unexpected error: {e}")

    print("Finished structured_main")

if __name__ == "__main__":
    # This requires Python 3.11+ for asyncio.TaskGroup
    # For older versions, libraries like 'trio' offer similar concepts.
    try:
        asyncio.run(structured_main())
    except AttributeError:
        print("asyncio.TaskGroup not available in this Python version. Skipping example.")

```

---
## Best Practices for Concurrent Programming

1.  **Minimize Shared Mutable State**: This is the root of many concurrency evils.
    * Use immutable data structures whenever possible.
    * Encapsulate shared state and ensure access is synchronized.
    * Consider message passing or actor models to avoid direct sharing.
2.  **Use High-Level Abstractions**: Prefer thread pools, task queues, `async/await`, and concurrent collections over manual thread and lock management. They are less error-prone and often more optimized.
3.  **Keep Critical Sections Short**: Hold locks for the minimum time necessary to reduce contention and improve parallelism.
4.  **Acquire Locks in a Consistent Order**: To prevent deadlocks.
5.  **Release Resources**: Always release locks, semaphores, etc., preferably using `try...finally` or context managers (`with` statement in Python).
6.  **Handle Exceptions in Concurrent Tasks**: Ensure exceptions in threads/tasks are caught, logged, and handled appropriately. Don't let them die silently.
7.  **Make Operations Interruptible/Cancellable**: Provide a way to gracefully stop or cancel long-running tasks if needed (e.g., using `threading.Event`, `asyncio.Future.cancel()`).
8.  **Use Timeouts**: For operations that might block indefinitely (lock acquisition, I/O, queue operations).
9.  **Test Thoroughly**: Concurrency bugs are often subtle. Use stress tests, long-duration tests, and consider tools for detecting race conditions/deadlocks.
10. **Understand Your Platform's Concurrency Model**: Be aware of things like the GIL in CPython and choose the right tools (threading, multiprocessing, asyncio) for the job (I/O-bound vs. CPU-bound).
11. **Profile and Measure**: Don't assume concurrency will always make things faster. Measure performance to identify bottlenecks and ensure concurrency is actually helping. Sometimes the overhead outweighs the benefits.
12. **Read and Understand Documentation**: For any concurrency primitive or library you use, deeply understand its semantics, guarantees, and potential pitfalls.
13. **Isolate Concurrency**: If possible, confine the concurrent parts of your system and keep the core logic simpler and sequential.
14. **Start Simple**: Don't over-engineer concurrency. Add it where it provides clear benefits and manage complexity carefully.

Mastering concurrency is a continuous learning process. It requires careful thought, discipline, and a deep understanding of the underlying principles. By systematically applying these concepts and best practices, you can build robust, responsive, and performant concurrent applications.