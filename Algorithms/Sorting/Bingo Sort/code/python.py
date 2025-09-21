def bingo_sort(arr):
    """
    Bingo Sort implementation.
    Sorts the list in ascending order.
    """
    if len(arr) <= 1:
        return arr


    # Place all elements in sorted position starting from the end

    i = len(arr) - 1

    while i >= 0:
        # Find the next maximum value
        next_val = max(arr[:i+1])

        j = i
        while j >= 0:
            if arr[j] == next_val:
                # Swap to the correct position
                arr[j], arr[i] = arr[i], arr[j]
                i -= 1
            j -= 1

    return arr
import unittest

class TestBingoSort(unittest.TestCase):

    def test_empty(self):
        self.assertEqual(bingo_sort([]), [])

    def test_single_element(self):
        self.assertEqual(bingo_sort([7]), [7])

    def test_sorted(self):
        self.assertEqual(bingo_sort([1, 2, 3, 4, 5]), [1, 2, 3, 4, 5])

    def test_reverse_sorted(self):
        self.assertEqual(bingo_sort([5, 4, 3, 2, 1]), [1, 2, 3, 4, 5])

    def test_duplicates(self):
        self.assertEqual(bingo_sort([4, 2, 4, 3, 1, 2]), [1, 2, 2, 3, 4, 4])

    def test_random(self):
        self.assertEqual(bingo_sort([10, 7, 2, 15, 7, 3]), [2, 3, 7, 7, 10, 15])


if __name__ == "__main__":
    unittest.main()
