#include <iostream>
#include <vector>
#include <algorithm>
#include <cassert>
#include <chrono>
#include <random>

// Binary search to find the position where element should be inserted
template<typename T>
int binarySearch(std::vector<T>& arr, int left, int right, T key) {
    while (left <= right) {
        int mid = left + (right - left) / 2;

        if (arr[mid] == key) {
            return mid + 1;  // Insert after equal elements to maintain stability
        } else if (arr[mid] < key) {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }
    return left;
}

// Binary Insertion Sort implementation
template<typename T>
void binaryInsertionSort(std::vector<T>& arr) {
    int n = arr.size();

    for (int i = 1; i < n; i++) {
        T key = arr[i];
        int j = i - 1;

        // Find location to insert using binary search
        int loc = binarySearch(arr, 0, j, key);

        // Shift elements and insert
        while (j >= loc) {
            arr[j + 1] = arr[j];
            j--;
        }
        arr[j + 1] = key;
    }
}

// Helper function to print array
template<typename T>
void printArray(const std::vector<T>& arr) {
    for (const auto& elem : arr) {
        std::cout << elem << " ";
    }
    std::cout << std::endl;
}

// Unit Test Class
class BinaryInsertionSortTest {
private:
    static void testEmptyArray() {
        std::vector<int> arr;
        binaryInsertionSort(arr);
        assert(arr.empty());
        std::cout << "✓ Empty array test passed" << std::endl;
    }

    static void testSingleElement() {
        std::vector<int> arr = {42};
        binaryInsertionSort(arr);
        assert(arr.size() == 1 && arr[0] == 42);
        std::cout << "✓ Single element test passed" << std::endl;
    }

    static void testAlreadySorted() {
        std::vector<int> arr = {1, 2, 3, 4, 5};
        std::vector<int> expected = {1, 2, 3, 4, 5};
        binaryInsertionSort(arr);
        assert(arr == expected);
        std::cout << "✓ Already sorted array test passed" << std::endl;
    }

    static void testReverseSorted() {
        std::vector<int> arr = {5, 4, 3, 2, 1};
        std::vector<int> expected = {1, 2, 3, 4, 5};
        binaryInsertionSort(arr);
        assert(arr == expected);
        std::cout << "✓ Reverse sorted array test passed" << std::endl;
    }

    static void testRandomArray() {
        std::vector<int> arr = {3, 7, 1, 9, 2, 5, 8, 4, 6};
        std::vector<int> expected = {1, 2, 3, 4, 5, 6, 7, 8, 9};
        binaryInsertionSort(arr);
        assert(arr == expected);
        std::cout << "✓ Random array test passed" << std::endl;
    }

    static void testDuplicateElements() {
        std::vector<int> arr = {5, 2, 8, 2, 9, 1, 5, 5};
        std::vector<int> expected = {1, 2, 2, 5, 5, 5, 8, 9};
        binaryInsertionSort(arr);
        assert(arr == expected);
        std::cout << "✓ Duplicate elements test passed" << std::endl;
    }

    static void testNegativeNumbers() {
        std::vector<int> arr = {-3, 5, -1, 0, -7, 2, -5};
        std::vector<int> expected = {-7, -5, -3, -1, 0, 2, 5};
        binaryInsertionSort(arr);
        assert(arr == expected);
        std::cout << "✓ Negative numbers test passed" << std::endl;
    }

    static void testFloatingPoint() {
        std::vector<double> arr = {3.14, 1.41, 2.71, 0.5, -1.5};
        std::vector<double> expected = {-1.5, 0.5, 1.41, 2.71, 3.14};
        binaryInsertionSort(arr);

        // Compare floating points with tolerance
        assert(arr.size() == expected.size());
        for (size_t i = 0; i < arr.size(); i++) {
            assert(std::abs(arr[i] - expected[i]) < 1e-9);
        }
        std::cout << "✓ Floating point numbers test passed" << std::endl;
    }

    static void testStrings() {
        std::vector<std::string> arr = {"banana", "apple", "cherry", "date", "elderberry"};
        std::vector<std::string> expected = {"apple", "banana", "cherry", "date", "elderberry"};
        binaryInsertionSort(arr);
        assert(arr == expected);
        std::cout << "✓ String sorting test passed" << std::endl;
    }

    static void testLargeArray() {
        const int size = 1000;
        std::vector<int> arr(size);
        std::vector<int> expected(size);

        // Fill with random numbers
        std::random_device rd;
        std::mt19937 gen(rd());
        std::uniform_int_distribution<> dis(1, 1000);

        for (int i = 0; i < size; i++) {
            arr[i] = dis(gen);
            expected[i] = arr[i];
        }

        // Sort expected array using STL for comparison
        std::sort(expected.begin(), expected.end());

        // Sort using binary insertion sort
        binaryInsertionSort(arr);

        assert(arr == expected);
        std::cout << "✓ Large array (1000 elements) test passed" << std::endl;
    }

    static void testStability() {
        // Test if the sort is stable (maintains relative order of equal elements)
        struct Person {
            std::string name;
            int age;
            bool operator<(const Person& other) const {
                return age < other.age;
            }
            bool operator==(const Person& other) const {
                return name == other.name && age == other.age;
            }
        };

        std::vector<Person> arr = {
            {"Alice", 25}, {"Bob", 30}, {"Charlie", 25}, {"David", 20}, {"Eve", 30}
        };

        std::vector<Person> expected = {
            {"David", 20}, {"Alice", 25}, {"Charlie", 25}, {"Bob", 30}, {"Eve", 30}
        };

        binaryInsertionSort(arr);

        // Check if order is maintained for equal elements
        assert(arr == expected);
        std::cout << "✓ Stability test passed" << std::endl;
    }

public:
    static void runAllTests() {
        std::cout << "\n=== Running Binary Insertion Sort Unit Tests ===" << std::endl;

        testEmptyArray();
        testSingleElement();
        testAlreadySorted();
        testReverseSorted();
        testRandomArray();
        testDuplicateElements();
        testNegativeNumbers();
        testFloatingPoint();
        testStrings();
        testLargeArray();
        testStability();

        std::cout << "\n✓ All tests passed successfully!" << std::endl;
    }
};

// Performance comparison function
void performanceComparison() {
    std::cout << "\n=== Performance Comparison ===" << std::endl;

    std::vector<int> sizes = {10, 100, 1000, 5000};
    std::random_device rd;
    std::mt19937 gen(rd());
    std::uniform_int_distribution<> dis(1, 10000);

    for (int size : sizes) {
        std::vector<int> arr(size);
        for (int i = 0; i < size; i++) {
            arr[i] = dis(gen);
        }

        // Time binary insertion sort
        std::vector<int> arr1 = arr;
        auto start = std::chrono::high_resolution_clock::now();
        binaryInsertionSort(arr1);
        auto end = std::chrono::high_resolution_clock::now();
        auto duration1 = std::chrono::duration_cast<std::chrono::microseconds>(end - start);

        // Time std::sort for comparison
        std::vector<int> arr2 = arr;
        start = std::chrono::high_resolution_clock::now();
        std::sort(arr2.begin(), arr2.end());
        end = std::chrono::high_resolution_clock::now();
        auto duration2 = std::chrono::duration_cast<std::chrono::microseconds>(end - start);

        std::cout << "Array size: " << size << std::endl;
        std::cout << "  Binary Insertion Sort: " << duration1.count() << " μs" << std::endl;
        std::cout << "  std::sort: " << duration2.count() << " μs" << std::endl;
        std::cout << "  Ratio: " << (double)duration1.count() / duration2.count() << "x slower" << std::endl;
        std::cout << std::endl;
    }
}

// Demo function
void demonstrateSort() {
    std::cout << "\n=== Binary Insertion Sort Demo ===" << std::endl;

    std::vector<int> arr = {64, 34, 25, 12, 22, 11, 90};

    std::cout << "Original array: ";
    printArray(arr);

    binaryInsertionSort(arr);

    std::cout << "Sorted array: ";
    printArray(arr);
}

int main() {
    // Run demonstration
    demonstrateSort();

    // Run all unit tests
    BinaryInsertionSortTest::runAllTests();

    // Run performance comparison
    performanceComparison();

    return 0;
}
