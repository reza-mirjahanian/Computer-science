import unittest
import random


def binary_search(arr, val, start, end):
    """
    Binary search to find the correct position to insert val in sorted arr[start:end].
    Returns the index where val should be inserted.
    """
    if start == end:
        return start if arr[start] > val else start + 1
    
    if start > end:
        return start
    
    mid = (start + end) // 2
    
    if arr[mid] < val:
        return binary_search(arr, val, mid + 1, end)
    elif arr[mid] > val:
        return binary_search(arr, val, start, mid - 1)
    else:
        return mid


def binary_insertion_sort(arr):
    """
    Sorts an array using binary insertion sort algorithm.
    
    Time Complexity: O(n²) - binary search reduces comparisons to O(log n) but 
                     shifting elements still takes O(n) time
    Space Complexity: O(1) - sorts in place
    
    Args:
        arr: List of comparable elements to sort
    
    Returns:
        The sorted array (modifies original array)
    """
    if not arr or len(arr) <= 1:
        return arr
    
    for i in range(1, len(arr)):
        val = arr[i]
        # Find the position to insert arr[i] in sorted portion arr[0:i]
        j = binary_search(arr, val, 0, i - 1)
        
        # Shift elements to the right to make space for insertion
        arr[j + 1:i + 1] = arr[j:i]
        arr[j] = val
    
    return arr


def binary_insertion_sort_iterative(arr):
    """
    Alternative implementation using iterative binary search.
    """
    if not arr or len(arr) <= 1:
        return arr
    
    for i in range(1, len(arr)):
        val = arr[i]
        left, right = 0, i - 1
        
        # Iterative binary search
        while left <= right:
            mid = (left + right) // 2
            if arr[mid] > val:
                right = mid - 1
            else:
                left = mid + 1
        
        # Shift elements and insert
        for k in range(i - 1, left - 1, -1):
            arr[k + 1] = arr[k]
        arr[left] = val
    
    return arr


class TestBinaryInsertionSort(unittest.TestCase):
    
    def test_empty_array(self):
        """Test with empty array"""
        self.assertEqual(binary_insertion_sort([]), [])
        self.assertEqual(binary_insertion_sort_iterative([]), [])
    
    def test_single_element(self):
        """Test with single element"""
        self.assertEqual(binary_insertion_sort([5]), [5])
        self.assertEqual(binary_insertion_sort_iterative([5]), [5])
    
    def test_already_sorted(self):
        """Test with already sorted array"""
        arr1 = [1, 2, 3, 4, 5]
        arr2 = [1, 2, 3, 4, 5]
        self.assertEqual(binary_insertion_sort(arr1), [1, 2, 3, 4, 5])
        self.assertEqual(binary_insertion_sort_iterative(arr2), [1, 2, 3, 4, 5])
    
    def test_reverse_sorted(self):
        """Test with reverse sorted array"""
        arr1 = [5, 4, 3, 2, 1]
        arr2 = [5, 4, 3, 2, 1]
        self.assertEqual(binary_insertion_sort(arr1), [1, 2, 3, 4, 5])
        self.assertEqual(binary_insertion_sort_iterative(arr2), [1, 2, 3, 4, 5])
    
    def test_duplicates(self):
        """Test with duplicate elements"""
        arr1 = [3, 1, 4, 1, 5, 9, 2, 6, 5, 3]
        arr2 = [3, 1, 4, 1, 5, 9, 2, 6, 5, 3]
        expected = [1, 1, 2, 3, 3, 4, 5, 5, 6, 9]
        self.assertEqual(binary_insertion_sort(arr1), expected)
        self.assertEqual(binary_insertion_sort_iterative(arr2), expected)
    
    def test_negative_numbers(self):
        """Test with negative numbers"""
        arr1 = [-3, -1, -4, -1, -5, 2, 6]
        arr2 = [-3, -1, -4, -1, -5, 2, 6]
        expected = [-5, -4, -3, -1, -1, 2, 6]
        self.assertEqual(binary_insertion_sort(arr1), expected)
        self.assertEqual(binary_insertion_sort_iterative(arr2), expected)
    
    def test_mixed_numbers(self):
        """Test with mixed positive and negative numbers"""
        arr1 = [0, -2, 5, -1, 3, -4, 1]
        arr2 = [0, -2, 5, -1, 3, -4, 1]
        expected = [-4, -2, -1, 0, 1, 3, 5]
        self.assertEqual(binary_insertion_sort(arr1), expected)
        self.assertEqual(binary_insertion_sort_iterative(arr2), expected)
    
    def test_all_same_elements(self):
        """Test with all identical elements"""
        arr1 = [7, 7, 7, 7, 7]
        arr2 = [7, 7, 7, 7, 7]
        expected = [7, 7, 7, 7, 7]
        self.assertEqual(binary_insertion_sort(arr1), expected)
        self.assertEqual(binary_insertion_sort_iterative(arr2), expected)
    
    def test_random_array(self):
        """Test with random array and compare both implementations"""
        for _ in range(10):  # Test multiple random arrays
            original = [random.randint(-100, 100) for _ in range(20)]
            arr1 = original.copy()
            arr2 = original.copy()
            expected = sorted(original)
            
            result1 = binary_insertion_sort(arr1)
            result2 = binary_insertion_sort_iterative(arr2)
            
            self.assertEqual(result1, expected)
            self.assertEqual(result2, expected)
            self.assertEqual(result1, result2)  # Both implementations should give same result
    
    def test_strings(self):
        """Test with string elements"""
        arr1 = ['banana', 'apple', 'cherry', 'date']
        arr2 = ['banana', 'apple', 'cherry', 'date']
        expected = ['apple', 'banana', 'cherry', 'date']
        self.assertEqual(binary_insertion_sort(arr1), expected)
        self.assertEqual(binary_insertion_sort_iterative(arr2), expected)
    
    def test_large_array_performance(self):
        """Test with larger array to verify it works"""
        arr = list(range(100, 0, -1))  # [100, 99, 98, ..., 1]
        expected = list(range(1, 101))  # [1, 2, 3, ..., 100]
        self.assertEqual(binary_insertion_sort(arr), expected)


def demo():
    """Demonstration of binary insertion sort"""
    print("Binary Insertion Sort Demo")
    print("=" * 30)
    
    # Test cases
    test_arrays = [
        [64, 34, 25, 12, 22, 11, 90],
        [5, 2, 4, 6, 1, 3],
        [1],
        [],
        [3, 3, 3, 3],
        [-5, -2, 0, 3, 1, -1]
    ]
    
    for i, arr in enumerate(test_arrays, 1):
        original = arr.copy()
        print(f"Test {i}:")
        print(f"  Original: {original}")
        print(f"  Sorted:   {binary_insertion_sort(arr)}")
        print()


if __name__ == "__main__":
    # Run demonstration
    demo()
    
    # Run unit tests
    print("Running unit tests...")
    unittest.main(verbosity=2)