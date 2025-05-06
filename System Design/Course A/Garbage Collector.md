### **Garbage Collection (GC) Overview**  
- **Purpose**: Reclaims unused memory to prevent memory leaks, crashes, and performance degradation.  
- **Core Question**: "Which objects in memory does the program still use?"  

---

### **Core Concepts**  
1. **Reachability**:  
   - **GC Roots**: Starting points (e.g., global variables, stack references).  
   - Objects reachable from GC roots are **alive**; unreachable objects are **garbage**.  

2. **Generational Hierarchy**:  
   - Based on observation: **most objects die young**.  
   - Memory divided into generations for efficiency:  
     - **Young Generation** (e.g., Java’s Eden/Survivor spaces):  
       - New objects start here.  
       - Survivors promoted to older generations after multiple GC cycles.  
     - **Old Generation**:  
       - Long-lived objects; collected less frequently.  
     - **Metaspace** (Java-specific): Stores class metadata.  
   - **Examples**:  
     - Java: Young → Old → Metaspace.  
     - V8 (JavaScript): Two generations.  
     - .NET: Three generations (0, 1, 2).  

---

### **GC Algorithms**  
1. **Mark and Sweep**:  
   - **Phases**:  
     - **Mark**: Traverse references from GC roots to mark reachable objects.  
     - **Sweep**: Reclaim memory from unmarked objects.  
   - **Drawback**: "Stop-the-world" pauses freeze applications during collection.  

2. **Tri-Color Mark and Sweep**:  
   - **Categories**:  
     - **White**: Potential garbage.  
     - **Gray**: Reachable but unexplored.  
     - **Black**: Reachable and processed.  
   - **Advantage**: Incremental processing minimizes pauses.  

---

### **Language-Specific Implementations**  
1. **Java**:  
   - **Algorithms**: Serial, Parallel, CMS (Concurrent Mark-Sweep), G1 (Garbage-First).  
   - Focuses on balancing latency, throughput, and scalability.  

2. **Python**:  
   - **Reference Counting**: Deallocates objects when reference count drops to zero.  
   - **Cyclic Collector**: Cleans up circular references.  

3. **Go**:  
   - **Concurrent Mark-Sweep**: Runs alongside the application.  
   - Uses **tricolor marking** for incremental collection with minimal pauses.  

---

### **Drawbacks of GC**  
1. **Performance Overhead**:  
   - Unpredictable pauses (problematic for latency-sensitive systems).  
2. **Memory Fragmentation**:  
   - Gaps in memory slow allocation over time.  
3. **Loss of Control**:  
   - Developers can’t manually trigger cleanup.  
4. **Balancing Pools**:  
   - Requires efficient management of used/free memory pools.  

---

### **Key Terms**  
- **GC Roots**: Entry points for determining reachability.  
- **Stop-the-World Pause**: Application freeze during collection.  
- **Metaspace**: Java’s memory area for class metadata.  
- **Tricolor Algorithm**: Reduces GC pauses via incremental processing.