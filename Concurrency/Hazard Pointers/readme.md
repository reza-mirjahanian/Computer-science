

# 🧠 Foundational Concepts

## What Problem Do Hazard Pointers Solve?

In **lock-free** or **wait-free** data structures (e.g., stacks, queues), threads may **concurrently** remove nodes from shared memory. Simply using `free()` or `delete` is unsafe, because:

* Another thread might still be **reading or accessing** that node.
* **Use-after-free** and **dangling pointer** bugs may occur.
* Memory reclamation must be **deferred** until it's safe.

### Alternatives to Hazard Pointers

| Technique               | Description                                          | Drawback                      |
| ----------------------- | ---------------------------------------------------- | ----------------------------- |
| Garbage Collection      | Automatic reclamation                                | Overhead, not deterministic   |
| Reference Counting      | Increment/decrement ref counts                       | Expensive in multithreading   |
| Epoch-Based Reclamation | Global "quiescent" points determine safe reclamation | Can't reclaim during long ops |
| **Hazard Pointers**     | Threads announce nodes they may access               | Slight overhead, highly safe  |

---

# 🧱 Core Idea of Hazard Pointers

A **hazard pointer** is a global/shared pointer that each thread uses to **publish** what memory location it is accessing. Before accessing a shared object:

1. Thread sets a hazard pointer to point to the object.
2. Other threads will **not reclaim** memory that is still pointed to by any hazard pointer.
3. After a thread is done accessing, it clears its hazard pointer.

---

## 🧩 Terminology

* **Hazard Pointer Table**: Global/shared array of hazard pointers.
* **Retire List**: A per-thread list of nodes that were logically removed but not yet safely freed.
* **Scan Phase**: Reclaims memory that is no longer hazardous.

---

# ✅ Basic Implementation Walkthrough

We’ll build this in **C++**, which is commonly used for low-level lock-free programming.

---

## Step 1: Hazard Pointer Table

```cpp
constexpr int MAX_THREADS = 128;
constexpr int HAZARD_POINTERS_PER_THREAD = 1;

struct HazardPointer {
    std::atomic<void*> pointer;
};

HazardPointer hazard_pointers[MAX_THREADS * HAZARD_POINTERS_PER_THREAD];
```

---

## Step 2: Accessing Hazard Pointer

Each thread "claims" a slot in the table:

```cpp
HazardPointer* get_hazard_pointer_for_thread(int thread_id) {
    return &hazard_pointers[thread_id * HAZARD_POINTERS_PER_THREAD];
}
```

---

## Step 3: Using Hazard Pointers During Read

Suppose we have a `Node*` in a lock-free stack:

```cpp
Node* load_protected(std::atomic<Node*>& shared_ptr, int thread_id) {
    Node* ptr;
    HazardPointer* hp = get_hazard_pointer_for_thread(thread_id);

    do {
        ptr = shared_ptr.load();
        hp->pointer.store(ptr);  // Publish hazard pointer
    } while (ptr != shared_ptr.load());  // Retry if changed

    return ptr;
}
```

---

## Step 4: Retiring a Node

Once a node is removed logically, we do *not* free it immediately.

```cpp
std::vector<Node*> retired_nodes[MAX_THREADS];

void retire_node(Node* node, int thread_id) {
    retired_nodes[thread_id].push_back(node);
    if (retired_nodes[thread_id].size() >= 10) {
        scan(thread_id);
    }
}
```

---

## Step 5: Scan for Safe Reclamation

```cpp
void scan(int thread_id) {
    std::unordered_set<void*> hazard_set;

    // Collect all hazard pointers
    for (int i = 0; i < MAX_THREADS * HAZARD_POINTERS_PER_THREAD; ++i) {
        hazard_set.insert(hazard_pointers[i].pointer.load());
    }

    // Reclaim retired nodes not in hazard pointers
    auto& retired = retired_nodes[thread_id];
    auto it = retired.begin();
    while (it != retired.end()) {
        if (hazard_set.count(*it) == 0) {
            delete *it;
            it = retired.erase(it);
        } else {
            ++it;
        }
    }
}
```

---

# 🧪 Example: Lock-Free Stack with Hazard Pointers

```cpp
struct Node {
    int value;
    std::atomic<Node*> next;
};

std::atomic<Node*> head;

void push(int value) {
    Node* new_node = new Node{value};
    do {
        new_node->next = head.load();
    } while (!head.compare_exchange_weak(new_node->next, new_node));
}

int pop(int thread_id) {
    Node* node;
    do {
        node = load_protected(head, thread_id);
        if (!node) return -1;
    } while (!head.compare_exchange_weak(node, node->next.load()));

    int value = node->value;
    retire_node(node, thread_id);
    return value;
}
```

---

# ⚠️ Edge Cases

1. **ABA Problem**: Hazard pointers prevent use-after-free, but *not* ABA. Use version tags or `tagged_ptr` for ABA protection.
2. **Thread IDs**: Need a safe, unique mapping from threads to hazard pointer slots.
3. **Thread Exit**: Threads must clean up or mark hazard pointers on exit.
4. **Multiple Pointers per Thread**: Some algorithms require 2–3 hazard pointers per thread (e.g., in linked list traversal).

---

# 🔁 Comparison: Hazard Pointers vs Epoch-Based Reclamation

| Feature                   | Hazard Pointers     | Epoch-Based Reclamation      |
| ------------------------- | ------------------- | ---------------------------- |
| Per-object tracking       | ✅ Yes               | ❌ No                         |
| Long-lived operations     | ✅ Safe              | ❌ Blocks reclamation         |
| Reclamation latency       | Low                 | High if threads are inactive |
| Implementation complexity | Medium              | Simple                       |
| Memory overhead           | Per-thread + global | Minimal                      |

---

# 🔧 Advanced Topics

## 1. Multiple Hazard Pointers per Thread

Useful in traversals or helping delete nodes in complex structures.

```cpp
HazardPointer* get_hazard_pointer(int thread_id, int slot) {
    return &hazard_pointers[thread_id * HAZARD_POINTERS_PER_THREAD + slot];
}
```

---

## 2. Using Hazard Pointers in C++ Smart Pointers

```cpp
template<typename T>
class HazardSmartPtr {
public:
    T* acquire(std::atomic<T*>& src, int thread_id) {
        T* ptr;
        do {
            ptr = src.load();
            get_hazard_pointer_for_thread(thread_id)->pointer.store(ptr);
        } while (ptr != src.load());
        return ptr;
    }

    void clear(int thread_id) {
        get_hazard_pointer_for_thread(thread_id)->pointer.store(nullptr);
    }
};
```

---

## 3. Integration with Custom Allocators

When using pool-based memory management, retired nodes can be returned to a thread-local pool instead of `delete`.

---

## 4. Lock-Free Linked List with Hazard Pointers

```cpp
bool search(int key, int thread_id) {
    Node* curr = load_protected(head, thread_id);
    while (curr && curr->value < key) {
        curr = load_protected(curr->next, thread_id);
    }
    return curr && curr->value == key;
}
```

---

## 5. Formal Properties

* **Safe Reclamation**: Never deallocates memory still in use.
* **Non-blocking**: No locks; threads don’t block each other.
* **Deterministic**: Reclamation occurs at known points.

---

# 🔒 Memory Order and Fences

Use `std::memory_order_acquire` and `std::memory_order_release` when loading/storing pointers if strict memory safety is needed:

```cpp
ptr = shared_ptr.load(std::memory_order_acquire);
hp->pointer.store(ptr, std::memory_order_release);
```

---

# 📚 Summary Table

| Concept             | Purpose                           |
| ------------------- | --------------------------------- |
| Hazard Pointer      | Prevents use-after-free           |
| Retire List         | Delays reclamation                |
| Scan Phase          | Reclaims safe memory              |
| load\_protected     | Safe load with hazard publication |
| retire\_node + scan | Safe deletion mechanism           |

---

