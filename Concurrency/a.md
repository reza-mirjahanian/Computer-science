## **What is Concurrency?**

**Concurrency** is the ability of a program to manage multiple tasks at the same time. It's about dealing with lots of things at once, but not necessarily doing them simultaneously.

### **Concurrency vs Parallelism**

| **Aspect** | **Concurrency** | **Parallelism** |
|------------|-----------------|-----------------|
| **Definition** | Multiple tasks making progress | Multiple tasks executing simultaneously |
| **Hardware** | Can run on single-core CPU | Requires multi-core CPU |
| **Focus** | Structure and design | Execution |
| **Example** | Juggling multiple balls (one at a time) | Multiple jugglers each with their own ball |

## **Fundamental Concepts**

### **1. Processes**
- **Independent execution units** with their own memory space
- **Heavy-weight** - expensive to create and switch between
- **Isolated** - cannot directly access another process's memory

```python
import multiprocessing
import os

def worker_process(name):
    print(f"Process {name}: PID={os.getpid()}")
    
# Creating processes
if __name__ == "__main__":
    processes = []
    for i in range(3):
        p = multiprocessing.Process(target=worker_process, args=(i,))
        processes.append(p)
        p.start()
    
    for p in processes:
        p.join()
```

### **2. Threads**
- **Light-weight** execution units within a process
- **Shared memory** - threads share the same memory space
- **Faster context switching** than processes

```python
import threading
import time

shared_counter = 0
lock = threading.Lock()

def increment_counter(thread_name):
    global shared_counter
    for _ in range(100000):
        with lock:  # Thread-safe operation
            shared_counter += 1
    print(f"{thread_name} finished")

# Creating threads
threads = []
for i in range(5):
    t = threading.Thread(target=increment_counter, args=(f"Thread-{i}",))
    threads.append(t)
    t.start()

for t in threads:
    t.join()

print(f"Final counter value: {shared_counter}")
```

### **3. Coroutines/Async**
- **Cooperative multitasking** - tasks voluntarily yield control
- **Single-threaded** but concurrent
- **Efficient** for I/O-bound operations

```python
import asyncio

async def fetch_data(id):
    print(f"Starting to fetch data {id}")
    await asyncio.sleep(1)  # Simulating I/O operation
    return f"Data {id}"

async def main():
    # Running multiple coroutines concurrently
    tasks = [fetch_data(i) for i in range(5)]
    results = await asyncio.gather(*tasks)
    print(f"Results: {results}")

asyncio.run(main())
```

## **Synchronization Primitives**

### **1. Mutex (Mutual Exclusion)**
**Ensures only one thread can access a resource at a time**

```python
import threading
import time

class BankAccount:
    def __init__(self):
        self.balance = 1000
        self.lock = threading.Lock()
    
    def withdraw(self, amount):
        with self.lock:
            if self.balance >= amount:
                time.sleep(0.001)  # Simulate processing
                self.balance -= amount
                return True
            return False
    
    def deposit(self, amount):
        with self.lock:
            time.sleep(0.001)  # Simulate processing
            self.balance += amount

# Usage example
account = BankAccount()

def perform_transactions(name):
    for _ in range(100):
        if account.withdraw(10):
            account.deposit(10)
    print(f"{name} completed. Balance: {account.balance}")

threads = [threading.Thread(target=perform_transactions, args=(f"Thread-{i}",)) 
           for i in range(3)]

for t in threads:
    t.start()
for t in threads:
    t.join()
```

### **2. Semaphore**
**Controls access to a resource with a counter**

```python
import threading
import time

# Limiting concurrent connections
connection_pool = threading.Semaphore(3)  # Max 3 concurrent connections

def access_database(user_id):
    with connection_pool:
        print(f"User {user_id} accessing database")
        time.sleep(2)  # Simulate database operation
        print(f"User {user_id} finished")

# Create 10 users trying to access database
threads = []
for i in range(10):
    t = threading.Thread(target=access_database, args=(i,))
    threads.append(t)
    t.start()

for t in threads:
    t.join()
```

### **3. Condition Variables**
**Allows threads to wait for certain conditions**

```python
import threading
import time
import random

condition = threading.Condition()
items = []

def consumer(name):
    with condition:
        while len(items) == 0:
            print(f"{name} waiting for items...")
            condition.wait()  # Wait until notified
        item = items.pop(0)
        print(f"{name} consumed {item}")

def producer():
    for i in range(5):
        time.sleep(random.uniform(0.5, 1.5))
        with condition:
            item = f"Item-{i}"
            items.append(item)
            print(f"Produced {item}")
            condition.notify_all()  # Wake up all waiting consumers

# Start consumers and producer
consumers = [threading.Thread(target=consumer, args=(f"Consumer-{i}",)) 
             for i in range(3)]
producer_thread = threading.Thread(target=producer)

for c in consumers:
    c.start()
producer_thread.start()

producer_thread.join()
for c in consumers:
    c.join()
```

### **4. Barriers**
**Synchronization point where threads wait for each other**

```python
import threading
import time
import random

barrier = threading.Barrier(3)  # Wait for 3 threads

def worker(worker_id):
    # Phase 1
    work_time = random.uniform(1, 3)
    print(f"Worker {worker_id} starting phase 1 ({work_time:.2f}s)")
    time.sleep(work_time)
    
    print(f"Worker {worker_id} waiting at barrier")
    barrier.wait()  # Wait for all workers
    
    # Phase 2 - all workers start together
    print(f"Worker {worker_id} starting phase 2")
    time.sleep(1)
    print(f"Worker {worker_id} completed")

workers = [threading.Thread(target=worker, args=(i,)) for i in range(3)]
for w in workers:
    w.start()
for w in workers:
    w.join()
```

## **Common Concurrency Patterns**

### **1. Producer-Consumer Pattern**

```python
import threading
import queue
import time
import random

# Thread-safe queue
task_queue = queue.Queue(maxsize=10)
stop_flag = threading.Event()

def producer(name):
    for i in range(5):
        item = f"{name}-Item-{i}"
        task_queue.put(item)
        print(f"[Producer {name}] Produced: {item}")
        time.sleep(random.uniform(0.1, 0.5))
    print(f"[Producer {name}] Finished")

def consumer(name):
    while not stop_flag.is_set() or not task_queue.empty():
        try:
            item = task_queue.get(timeout=1)
            print(f"[Consumer {name}] Consumed: {item}")
            time.sleep(random.uniform(0.2, 0.7))
            task_queue.task_done()
        except queue.Empty:
            continue
    print(f"[Consumer {name}] Finished")

# Start producers and consumers
producers = [threading.Thread(target=producer, args=(f"P{i}",)) for i in range(2)]
consumers = [threading.Thread(target=consumer, args=(f"C{i}",)) for i in range(3)]

for t in producers + consumers:
    t.start()

for p in producers:
    p.join()

task_queue.join()  # Wait for all tasks to be processed
stop_flag.set()    # Signal consumers to stop

for c in consumers:
    c.join()
```

### **2. Thread Pool Pattern**

```python
import concurrent.futures
import time
import random

def process_task(task_id):
    """Simulate a time-consuming task"""
    processing_time = random.uniform(1, 3)
    print(f"Task {task_id} starting (will take {processing_time:.2f}s)")
    time.sleep(processing_time)
    return f"Task {task_id} result"

# Using ThreadPoolExecutor
with concurrent.futures.ThreadPoolExecutor(max_workers=3) as executor:
    # Submit multiple tasks
    futures = [executor.submit(process_task, i) for i in range(10)]
    
    # Process results as they complete
    for future in concurrent.futures.as_completed(futures):
        result = future.result()
        print(f"Completed: {result}")

# Using ProcessPoolExecutor for CPU-bound tasks
def cpu_bound_task(n):
    """Simulate CPU-intensive task"""
    total = sum(i * i for i in range(n))
    return f"Sum of squares up to {n}: {total}"

with concurrent.futures.ProcessPoolExecutor(max_workers=4) as executor:
    tasks = [1000000, 2000000, 3000000, 4000000]
    results = executor.map(cpu_bound_task, tasks)
    for result in results:
        print(result)
```

### **3. Reader-Writer Lock Pattern**

```python
import threading
import time
import random

class ReadWriteLock:
    def __init__(self):
        self._read_ready = threading.Condition(threading.RLock())
        self._readers = 0
        
    def acquire_read(self):
        self._read_ready.acquire()
        try:
            self._readers += 1
        finally:
            self._read_ready.release()
            
    def release_read(self):
        self._read_ready.acquire()
        try:
            self._readers -= 1
            if self._readers == 0:
                self._read_ready.notifyAll()
        finally:
            self._read_ready.release()
            
    def acquire_write(self):
        self._read_ready.acquire()
        while self._readers > 0:
            self._read_ready.wait()
            
    def release_write(self):
        self._read_ready.release()

# Shared data
shared_data = {"value": 0}
rw_lock = ReadWriteLock()

def reader(reader_id):
    for _ in range(3):
        rw_lock.acquire_read()
        try:
            print(f"Reader {reader_id} reading: {shared_data['value']}")
            time.sleep(random.uniform(0.1, 0.3))
        finally:
            rw_lock.release_read()
        time.sleep(random.uniform(0.1, 0.5))

def writer(writer_id):
    for i in range(2):
        rw_lock.acquire_write()
        try:
            shared_data['value'] += 1
            print(f"Writer {writer_id} updated value to: {shared_data['value']}")
            time.sleep(random.uniform(0.2, 0.4))
        finally:
            rw_lock.release_write()
        time.sleep(random.uniform(0.3, 0.7))

# Start readers and writers
readers = [threading.Thread(target=reader, args=(i,)) for i in range(3)]
writers = [threading.Thread(target=writer, args=(i,)) for i in range(2)]

for t in readers + writers:
    t.start()
for t in readers + writers:
    t.join()
```

## **Common Concurrency Problems**

### **1. Race Conditions**
**Multiple threads accessing shared data without proper synchronization**

```python
# WRONG - Race condition
counter = 0

def increment_unsafe():
    global counter
    for _ in range(1000000):
        counter += 1  # NOT atomic operation

# RIGHT - Using lock
counter_safe = 0
lock = threading.Lock()

def increment_safe():
    global counter_safe
    for _ in range(1000000):
        with lock:
            counter_safe += 1

# Demonstration
import threading

# Unsafe version
counter = 0
threads = [threading.Thread(target=increment_unsafe) for _ in range(5)]
for t in threads:
    t.start()
for t in threads:
    t.join()
print(f"Unsafe counter (should be 5000000): {counter}")

# Safe version
counter_safe = 0
threads = [threading.Thread(target=increment_safe) for _ in range(5)]
for t in threads:
    t.start()
for t in threads:
    t.join()
print(f"Safe counter: {counter_safe}")
```

### **2. Deadlocks**
**Threads waiting for each other indefinitely**

```python
import threading
import time

# Deadlock example
lock1 = threading.Lock()
lock2 = threading.Lock()

def thread1_func():
    with lock1:
        print("Thread 1 acquired lock1")
        time.sleep(0.1)
        print("Thread 1 waiting for lock2")
        with lock2:  # DEADLOCK - Thread 2 has lock2
            print("Thread 1 acquired lock2")

def thread2_func():
    with lock2:
        print("Thread 2 acquired lock2")
        time.sleep(0.1)
        print("Thread 2 waiting for lock1")
        with lock1:  # DEADLOCK - Thread 1 has lock1
            print("Thread 2 acquired lock1")

# Solution: Always acquire locks in the same order
def thread1_safe():
    with lock1:
        with lock2:
            print("Thread 1 working safely")

def thread2_safe():
    with lock1:  # Same order as thread1
        with lock2:
            print("Thread 2 working safely")

# Alternative: Using timeout
def thread_with_timeout():
    acquired_lock1 = lock1.acquire(timeout=1)
    if acquired_lock1:
        try:
            acquired_lock2 = lock2.acquire(timeout=1)
            if acquired_lock2:
                try:
                    print("Successfully acquired both locks")
                finally:
                    lock2.release()
            else:
                print("Failed to acquire lock2, releasing lock1")
        finally:
            lock1.release()
```

### **3. Starvation**
**Thread never gets access to resources**

```python
import threading
import time
import random

# Priority-based scheduling can cause starvation
class PriorityLock:
    def __init__(self):
        self.lock = threading.Lock()
        self.condition = threading.Condition(self.lock)
        self.high_priority_waiting = 0
        
    def acquire_high_priority(self):
        with self.lock:
            self.high_priority_waiting += 1
        self.lock.acquire()
        with self.lock:
            self.high_priority_waiting -= 1
            
    def acquire_low_priority(self):
        with self.lock:
            while self.high_priority_waiting > 0:
                self.condition.wait()
        self.lock.acquire()
        
    def release(self):
        self.lock.release()
        with self.lock:
            self.condition.notify_all()

# Fair lock implementation
class FairLock:
    def __init__(self):
        self.lock = threading.Lock()
        self.condition = threading.Condition(self.lock)
        self.waiting_queue = []
        self.owner = None
        
    def acquire(self, thread_id):