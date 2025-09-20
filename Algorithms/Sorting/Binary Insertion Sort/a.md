### 1. One-line summary
Binary insertion sort is a sorting algorithm that builds a sorted list by inserting each new element into its correct position using binary search for efficiency, useful for small to medium datasets where insertion sort's simplicity is desired but with faster position finding.

### 2. Intuition and real-world analogy
Imagine organizing a deck of cards in your hand: for each new card, you quickly scan (like binary search) to find the right spot between the already sorted cards and slide it in, shifting others over. This is like filing books on a shelf where you halve the search area each time to find the insertion point instead of checking one by one.

### 3. Formal definition and key properties
Binary insertion sort is a stable, in-place sorting algorithm that extends insertion sort by using binary search to locate the insertion position for each element. Key properties include: it maintains a sorted subarray at the beginning, it's adaptive (faster on nearly sorted data), online (can sort as elements arrive), and has quadratic time complexity but with logarithmic search improvements.

### 4. Required operations and what they do
- **Sort**: Processes the array from left to right, treating the left part as sorted and inserting each new element into it.
- **Binary search (for insertion point)**: Finds the correct position in the sorted subarray by repeatedly dividing the search interval in half; returns the index where the element should go to keep the subarray sorted.
- **Insert (shift and place)**: Shifts elements in the sorted subarray to the right to make space, then places the new element at the found position. No delete operation is needed as it's a sorting algorithm.

### 5. Step-by-step algorithm walkthrough
1. **Start with the first element**: Consider the array's first element as the initial sorted subarray (of size 1).  
   Example: Array = [5, 3, 8, 4]. After step: Sorted part = [5], unsorted = [3, 8, 4].

2. **Pick the next unsorted element**: Select the element at the current index (starting from 1). This is the key to insert.  
   Example: Next key = 3. Current array: [5 | 3, 8, 4] (sorted | unsorted).

3. **Use binary search to find insertion position**: Search the sorted subarray (indices 0 to current-1) for where the key fits, without inserting yet. Binary search compares the middle, then halves left or right.  
   Example: Sorted subarray [5], binary search for 3: Mid = 5 > 3, so position = 0.  
   Intermediate: Low=0, High=0, Mid=0 (5>3, set high=-1, position=0).

4. **Shift elements to make space**: Move elements from the found position to the end of the sorted subarray one spot right.  
   Example: Shift [5] right from position 0: Array becomes [_, 5, 8, 4]. (But since position 0, shift 5 to index 1).

5. **Place the key**: Insert the key at the found position. Now the sorted subarray grows by one.  
   Example: Place 3 at 0: Array = [3, 5, 8, 4]. Sorted part = [3,5], unsorted = [8,4].  
   Repeat for next keys.

6. **Repeat until end**: Continue for all remaining elements until the whole array is sorted.

### 6. Pseudocode
```
function binary_insertion_sort(array):
    for i from 1 to length(array)-1:
        key = array[i]
        left = 0
        right = i - 1
        while left <= right:
            mid = (left + right) // 2
            if array[mid] < key:
                left = mid + 1
            else:
                right = mid - 1
        pos = left  # insertion point
        for j from i-1 down to pos:
            array[j+1] = array[j]  # shift right
        array[pos] = key
```

### 7. Two runnable implementations
#### Short, idiomatic version in Python
```python
def binary_insertion_sort(arr):
    for i in range(1, len(arr)):
        key = arr[i]
        left, right = 0, i - 1
        while left <= right:
            mid = (left + right) // 2
            if arr[mid] < key:
                left = mid + 1
            else:
                right = mid - 1
        pos = left
        arr[pos + 1:i + 1] = arr[pos:i]  # Shift using slicing
        arr[pos] = key
    return arr
```

#### A commented version suitable for learners
```python
def binary_insertion_sort(arr):
    # Loop through each element starting from the second
    for i in range(1, len(arr)):
        key = arr[i]  # The element to insert
        left = 0  # Start of sorted part
        right = i - 1  # End of sorted part
        
        # Binary search to find the insertion position
        while left <= right:
            mid = (left + right) // 2  # Middle index
            if arr[mid] < key:  # If mid value is smaller, search right half
                left = mid + 1
            else:  # Otherwise, search left half
                right = mid - 1
        
        pos = left  # This is where key should go
        
        # Shift elements to the right to make space
        # We do this by moving from the end backwards
        for j in range(i - 1, pos - 1, -1):
            arr[j + 1] = arr[j]  # Shift each one right
        
        arr[pos] = key  # Place the key in position
    return arr
```

### 8. One or two complete worked examples
#### Worked Example 1: Sorting [4, 2, 5, 1]
Textual table of states:

| Step | i | Key | Binary Search (left, right, mid) | Pos | Shift/Insert | Array State |
|------|---|-----|----------------------------------|-----|-------------|-------------|
| Initial | - | - | - | - | - | [4, 2, 5, 1] |
| 1 | 1 | 2 | (0,0,0): 4>2 → right=-1 | 0 | Shift [4] to right, insert 2 at 0 | [2, 4, 5, 1] |
| 2 | 2 | 5 | (0,1,0): 2<5 → left=1; (1,1,1):4<5 → left=2 | 2 | No shift (pos=2==i), insert 5 at 2 | [2, 4, 5, 1] |
| 3 | 3 | 1 | (0,2,1):4>1 → right=0; (0,0,0):2>1 → right=-1 | 0 | Shift [2,4,5] right, insert 1 at 0 | [1, 2, 4, 5] |

```mermaid
graph TD
    A[Start: [4,2,5,1]] --> B[After i=1 insert 2: [2,4,5,1]]
    B --> C[After i=2 insert 5: [2,4,5,1]]
    C --> D[After i=3 insert 1: [1,2,4,5]]
```
ASCII fallback:
```
Start: [4 | 2,5,1]
After insert 2 at 0: [2,4 | 5,1]
After insert 5 at 2: [2,4,5 | 1]
After insert 1 at 0: [1,2,4,5]
```

#### Worked Example 2: Sorting [3, 1]
Textual table of states:

| Step | i | Key | Binary Search (left, right, mid) | Pos | Shift/Insert | Array State |
|------|---|-----|----------------------------------|-----|-------------|-------------|
| Initial | - | - | - | - | - | [3, 1] |
| 1 | 1 | 1 | (0,0,0): 3>1 → right=-1 | 0 | Shift [3] to right, insert 1 at 0 | [1, 3] |

ASCII fallback:
```
Start: [3 | 1]
After insert 1 at 0: [1,3]
```

### 9. Complexity
- **Time**: Best O(n log n) when already sorted (binary search per insertion, minimal shifts); average/worst O(n²) because while search is O(log n), shifting can take O(n) per insertion, totaling O(n²) for n insertions.  
- **Space**: O(1) auxiliary space since it's in-place, only using a few variables.  
Explanation: Binary search speeds up finding the spot, but the bottleneck is shifting elements in the array, which is linear in the subarray size on average.

### 10. Correctness sketch or invariant
The algorithm maintains the invariant that after each outer loop iteration for index i, the subarray from 0 to i is sorted. This holds because we start with a sorted subarray of one element, and for each new key, binary search correctly identifies the position where inserting it preserves the sorted order (since it finds the first spot where all left are <= key and right > key, adjusted for stability). The shifts ensure space is made without disrupting the order, and placing the key completes the invariant for the larger subarray. By induction, at the end, the whole array is sorted.

### 11. Common pitfalls, edge cases, and variations/optimizations
- **Pitfalls**: Off-by-one errors in binary search bounds (e.g., forgetting left=mid+1); not handling duplicates properly, but since it's stable, it preserves order.  
- **Edge cases**: Empty array (already sorted); single element (no action); all elements equal (minimal shifts); reverse sorted (max shifts).  
- **Variations/optimizations**: Use it in hybrid sorts like Timsort for small subarrays; optimize shifts with array slicing in languages like Python; for large arrays, prefer O(n log n) sorts like merge sort.

### 12. 2 practice problems
- **Easy**: Sort [7, 3, 9] using binary insertion sort. Show final array.  
  **Solution**: After inserting 3 at pos=0: [3,7,9]; then 9 at pos=2: [3,7,9]. Final: [3,7,9].  

- **Medium**: Implement a function that uses binary insertion sort but only on arrays longer than 5; otherwise use simple insertion sort. Why might this be useful?  
  **Solution**: 
  ```python
  def hybrid_sort(arr):
      if len(arr) <= 5:
          for i in range(1, len(arr)):  # Simple insertion
              key = arr[i]
              j = i - 1
              while j >= 0 and arr[j] > key:
                  arr[j + 1] = arr[j]
                  j -= 1
              arr[j + 1] = key
      else:
          # Use binary_insertion_sort from section 7
          binary_insertion_sort(arr)
      return arr
  ```
  Useful because binary search overhead is negligible for small arrays, where linear scan is faster due to less computation.

### 13. Short cheat-sheet and recommended next topics
- Builds sorted prefix by inserting via binary search (O(log n) find) + linear shift (O(n)).
- Stable and in-place; time O(n²) worst, better on sorted data.
- Use for small n or nearly sorted lists.
- Pseudocode: Outer loop i=1 to n-1; binary search pos; shift from i-1 to pos; insert key.
- Next topics: Insertion sort (base version), Merge sort (efficient alternative), Binary search (core subroutine).