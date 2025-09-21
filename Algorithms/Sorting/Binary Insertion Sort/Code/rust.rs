/// Binary Insertion Sort
///
/// This is an optimization of the standard insertion sort that uses binary search
/// to find the proper location to insert the selected element, reducing the number
/// of comparisons from O(n) to O(log n) per insertion.
///
/// Time Complexity: O(n²) - due to shifting elements
/// Space Complexity: O(1) - in-place sorting
pub fn binary_insertion_sort<T: Ord + Copy>(arr: &mut [T]) {
    if arr.len() <= 1 {
        return;
    }

    for i in 1..arr.len() {
        let key = arr[i];
        let pos = binary_search(arr, key, 0, i);

        // Shift elements to make room for the key
        for j in (pos..i).rev() {
            arr[j + 1] = arr[j];
        }

        arr[pos] = key;
    }
}

/// Binary search to find the position where the key should be inserted
/// Returns the leftmost position where the key can be inserted
fn binary_search<T: Ord>(arr: &[T], key: T, low: usize, high: usize) -> usize {
    let mut left = low;
    let mut right = high;

    while left < right {
        let mid = left + (right - left) / 2;
        if arr[mid] < key {
            left = mid + 1;
        } else {
            right = mid;
        }
    }

    left
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_array() {
        let mut arr: Vec<i32> = vec![];
        binary_insertion_sort(&mut arr);
        assert_eq!(arr, vec![]);
    }

    #[test]
    fn test_single_element() {
        let mut arr = vec![42];
        binary_insertion_sort(&mut arr);
        assert_eq!(arr, vec![42]);
    }

    #[test]
    fn test_already_sorted() {
        let mut arr = vec![1, 2, 3, 4, 5];
        let expected = vec![1, 2, 3, 4, 5];
        binary_insertion_sort(&mut arr);
        assert_eq!(arr, expected);
    }

    #[test]
    fn test_reverse_sorted() {
        let mut arr = vec![5, 4, 3, 2, 1];
        let expected = vec![1, 2, 3, 4, 5];
        binary_insertion_sort(&mut arr);
        assert_eq!(arr, expected);
    }

    #[test]
    fn test_random_order() {
        let mut arr = vec![64, 34, 25, 12, 22, 11, 90];
        let mut expected = arr.clone();
        expected.sort();
        binary_insertion_sort(&mut arr);
        assert_eq!(arr, expected);
    }

    #[test]
    fn test_with_duplicates() {
        let mut arr = vec![5, 2, 8, 2, 9, 1, 5, 5];
        let mut expected = arr.clone();
        expected.sort();
        binary_insertion_sort(&mut arr);
        assert_eq!(arr, expected);
    }

    #[test]
    fn test_negative_numbers() {
        let mut arr = vec![-3, -1, -7, -2, -5, 0, 4];
        let mut expected = arr.clone();
        expected.sort();
        binary_insertion_sort(&mut arr);
        assert_eq!(arr, expected);
    }

    #[test]
    fn test_strings() {
        let mut arr = vec!["banana", "apple", "cherry", "date"];
        let mut expected = arr.clone();
        expected.sort();
        binary_insertion_sort(&mut arr);
        assert_eq!(arr, expected);
    }

    #[test]
    fn test_binary_search_edge_cases() {
        // Test binary search function directly
        let arr = [1, 3, 5, 7, 9];

        // Search for value smaller than all elements
        assert_eq!(binary_search(&arr, 0, 0, 5), 0);

        // Search for value larger than all elements
        assert_eq!(binary_search(&arr, 10, 0, 5), 5);

        // Search for value that exists
        assert_eq!(binary_search(&arr, 5, 0, 5), 2);

        // Search for value between elements
        assert_eq!(binary_search(&arr, 4, 0, 5), 2);
    }
}

#[cfg(test)]
mod benchmark {
    use super::*;
    use rand::Rng;
    use std::time::Instant;

    #[test]
    fn benchmark_binary_insertion_sort() {
        // Generate a random array for benchmarking
        let mut rng = rand::thread_rng();
        let mut arr: Vec<i32> = (0..1000).map(|_| rng.gen_range(0..1000)).collect();

        let start = Instant::now();
        binary_insertion_sort(&mut arr);
        let duration = start.elapsed();

        // Verify it's sorted
        assert!(arr.windows(2).all(|w| w[0] <= w[1]));

        println!("Binary Insertion Sort for 1000 elements: {:?}", duration);

        // Compare with standard sort for correctness
        let mut expected: Vec<i32> = (0..1000).map(|_| rng.gen_range(0..1000)).collect();
        let mut test_arr = expected.clone();
        expected.sort();
        binary_insertion_sort(&mut test_arr);
        assert_eq!(test_arr, expected);
    }
}

// Optional: Add this if you want to run benchmarks with cargo bench
#[cfg(feature = "bench")]
mod benches {
    use super::*;
    use rand::Rng;
    use test::Bencher;

    #[bench]
    fn bench_binary_insertion_sort_100(b: &mut Bencher) {
        b.iter(|| {
            let mut rng = rand::thread_rng();
            let mut arr: Vec<i32> = (0..100).map(|_| rng.gen_range(0..1000)).collect();
            binary_insertion_sort(&mut arr);
        });
    }

    #[bench]
    fn bench_binary_insertion_sort_1000(b: &mut Bencher) {
        b.iter(|| {
            let mut rng = rand::thread_rng();
            let mut arr: Vec<i32> = (0..1000).map(|_| rng.gen_range(0..1000)).collect();
            binary_insertion_sort(&mut arr);
        });
    }
}