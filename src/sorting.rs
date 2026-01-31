//! Standard Sorting Algorithms
//!
//! Implementation of classic sorting algorithms for i32 arrays.
//! Includes:
//! - Insertion sort
//! - Selection sort
//! - Quick sort
//! - Merge sort
//! - Heap sort (generic implementation)
//!
//! For generic implementations, see algos_tri_variantes.rs.
//!
//! Author: Vincent Espitalier
//! Date: June 2024

#![allow(dead_code)]

/// Sorts an array using the insertion sort algorithm.
///
/// # Arguments
/// * `array` - Mutable slice of i32 to be sorted
///
/// # Complexity
/// O(n²) time complexity
///
/// # Example
/// ```
/// let mut arr = [5, 2, 4, 6, 1, 3];
/// tri_par_insertion(&mut arr);
/// assert_eq!(arr, [1, 2, 3, 4, 5, 6]);
/// ```
///
/// # Reference
/// [Insertion sort - Wikipedia](https://en.wikipedia.org/wiki/Insertion_sort)
pub fn insertion_sort(array: &mut [i32]) {
    println!("insertion_sort > called");

    let n = array.len();

    // Sort elements of the array successively
    for i in 0..n {
        let current = array[i];

        // Shift elements smaller than current to make space for insertion
        let mut insert_index = 0;
        for j in (0..i).rev() {
            // Stop when elements are smaller than current
            // In case of equality, break to maintain stability
            if array[j] <= current {
                insert_index = j + 1;
                break;
            }
            array[j + 1] = array[j];
        }

        // Insert the current element at the correct position
        array[insert_index] = current;

        // Loop invariant:
        // After each iteration, the first (i+1) elements of the array are sorted
    }
}

/// Sorts an array using the selection sort algorithm.
///
/// # Arguments
/// * `array` - Mutable slice of i32 to be sorted
///
/// # Complexity
/// O(n²) time complexity
/// Exactly (n(n-1)/2) comparisons and at most (n-1) swaps
///
/// # Example
/// ```
/// let mut arr = [5, 2, 4, 6, 1, 3];
/// tri_par_selection(&mut arr);
/// assert_eq!(arr, [1, 2, 3, 4, 5, 6]);
/// ```
///
/// # Reference
/// [Selection sort - Wikipedia](https://en.wikipedia.org/wiki/Selection_sort)
pub fn selection_sort(array: &mut [i32]) {
    println!("selection_sort > called");

    let n = array.len();

    // Special case: array with only one element
    if n <= 1 {
        return;
    }

    // Sort elements of the array successively
    for i in 0..n {
        let mut min_value = array[i];
        let mut min_index = i;

        // Find the smallest element among the unsorted elements
        for (j, &elem) in array.iter().enumerate().skip(i + 1) {
            if elem < min_value {
                min_value = elem;
                min_index = j;
            }
        }

        // The i-th smallest element is at position min_index with value min_value
        // Swap elements at indices i and min_index
        if i != min_index {
            array.swap(i, min_index);
        }

        // Loop invariant:
        // After each iteration, the first (i+1) elements of the array are sorted
        // and are the smallest elements of the entire array
    }
}

/// Sorts an array using the quick sort algorithm.
///
/// # Arguments
/// * `array` - Mutable slice of i32 to be sorted
///
/// # Complexity
/// O(n log n) average time complexity
/// O(n²) worst-case time complexity
///
/// # Note
/// This is a non-optimized implementation that creates new allocations
/// for sub-arrays at each recursive call.
///
/// # Example
/// ```
/// let mut arr = [5, 2, 4, 6, 1, 3];
/// tri_rapide(&mut arr);
/// assert_eq!(arr, [1, 2, 3, 4, 5, 6]);
/// ```
///
/// # Reference
/// [Quick sort - Wikipedia](https://en.wikipedia.org/wiki/Quicksort)
pub fn quick_sort(array: &mut [i32]) {
    let n = array.len();

    // Base case: array with 0 or 1 element (nothing to sort)
    if n <= 1 {
        return;
    }

    // General case: split, sort sub-arrays, and combine
    let pivot = array[0]; // Pivot is the first element

    let mut left_vec: Vec<i32> = Vec::new();
    let mut right_vec: Vec<i32> = Vec::new();

    // Partition elements into left and right sub-arrays
    for &elem in array.iter().skip(1) {
        if elem <= pivot {
            left_vec.push(elem);
        } else {
            right_vec.push(elem);
        }
    }

    // Recursively sort sub-arrays
    let left_array: &mut [i32] = left_vec.as_mut_slice();
    let right_array: &mut [i32] = right_vec.as_mut_slice();
    quick_sort(left_array);
    quick_sort(right_array);

    // Combine sorted sub-arrays
    let mut index = 0;
    for elem in left_array.iter() {
        array[index] = *elem;
        index += 1;
    }

    array[index] = pivot;
    index += 1;

    for elem in right_array {
        array[index] = *elem;
        index += 1;
    }
}

/// Sorts an array using the merge sort algorithm.
///
/// # Arguments
/// * `array` - Mutable slice of i32 to be sorted
///
/// # Complexity
/// O(n log n) time complexity in both average and worst cases
///
/// # Note
/// This is a non-optimized implementation that creates new allocations
/// for merging sub-arrays at each recursive call.
///
/// # Example
/// ```
/// let mut arr = [5, 2, 4, 6, 1, 3];
/// tri_fusion(&mut arr);
/// assert_eq!(arr, [1, 2, 3, 4, 5, 6]);
/// ```
///
/// # Reference
/// [Merge sort - Wikipedia](https://en.wikipedia.org/wiki/Merge_sort)
pub fn merge_sort(array: &mut [i32]) {
    let n = array.len();

    // Base case: array with 0 or 1 element (nothing to sort)
    if n <= 1 {
        return;
    }

    // Special case: array with 2 elements
    if n == 2 {
        if array[0] > array[1] {
            array.swap(0, 1);
        }
        return;
    }

    // General case: split, sort sub-arrays, and merge
    let mid = n / 2;

    // Split the array into left and right halves
    let (left_array, right_array) = array.split_at_mut(mid);

    // Recursively sort sub-arrays
    merge_sort(left_array);
    merge_sort(right_array);

    // Merge the sorted sub-arrays
    let mut merged_array: Vec<i32> = Vec::new();
    let mut left_index = 0;
    let mut right_index = 0;

    // Merge the two arrays by taking the smaller element at each step
    for _ in 0..n {
        if left_index < left_array.len() && right_index < right_array.len() {
            // Both sub-arrays still have elements to process
            if left_array[left_index] <= right_array[right_index] {
                merged_array.push(left_array[left_index]);
                left_index += 1;
            } else {
                merged_array.push(right_array[right_index]);
                right_index += 1;
            }
        } else if left_index < left_array.len() {
            // Right array has been fully processed
            merged_array.push(left_array[left_index]);
            left_index += 1;
        } else if right_index < right_array.len() {
            // Left array has been fully processed
            merged_array.push(right_array[right_index]);
            right_index += 1;
        } else {
            // This case should never happen
            panic!("Internal error: Main array not filled, but both sub-arrays have been fully processed.");
        }
    }

    // Copy the merged array back to the original array
    array.clone_from_slice(&merged_array);
}

/// Sorts an array using the heap sort algorithm (generic implementation).
///
/// # Type Parameters
/// * `T` - Type of elements to sort (must be Ord + Clone + Debug)
///
/// # Arguments
/// * `array` - Mutable slice of elements to be sorted
///
/// # Complexity
/// O(n log n) time complexity in both average and worst cases
///
/// # Note
/// Uses Rust's BinaryHeap data structure.
/// Elements are inserted into the heap and then extracted in descending order.
///
/// # Example
/// ```
/// let mut arr = [5, 2, 4, 6, 1, 3];
/// tri_par_tas_generique(&mut arr);
/// assert_eq!(arr, [1, 2, 3, 4, 5, 6]);
/// ```
///
/// # Reference
/// [Heap sort - Wikipedia](https://en.wikipedia.org/wiki/Heapsort)
pub fn heap_sort<T>(array: &mut [T])
where
    T: Ord + Clone + core::fmt::Debug,
{
    println!("heap_sort > called");
    let n = array.len();

    // Base case: array with 0 or 1 element (nothing to sort)
    if n <= 1 {
        return;
    }

    // General case: use a binary heap
    use std::collections::BinaryHeap;
    let mut heap = BinaryHeap::new();

    // Insert all elements into the heap
    for elem in array.iter() {
        heap.push(elem.clone());
    }

    // Extract elements from the heap in descending order
    for i in (0..n).rev() {
        let element = heap.pop();
        assert_ne!(
            element, None,
            "heap_sort: Internal error (1): There should be elements left in the heap."
        );

        array[i] = element.unwrap().clone();

        // Loop invariant:
        // After each iteration:
        // - The last (n - i) elements of the array are sorted and are the largest elements
        //   from the original array (i.e., they are in their final position)
        // - The heap contains the remaining elements, with the root pointing to the largest element
    }

    assert_eq!(
        heap.pop(),
        None,
        "heap_sort: Internal error (2): There should be no elements left in the heap."
    );
}

/// Verifies if an array is sorted in ascending order.
///
/// # Type Parameters
/// * `T` - Type of elements in the array (must be Ord)
///
/// # Arguments
/// * `array` - Slice of elements to check
///
/// # Returns
/// `true` if the array is sorted in ascending order, `false` otherwise
///
/// # Example
/// ```
/// let arr = [1, 2, 3, 4, 5];
/// assert!(verif_tableau_croissant(&arr));
/// ```
pub fn is_array_sorted<T>(array: &[T]) -> bool
where
    T: Ord,
{
    let n = array.len();
    for i in 0..(n - 1) {
        if array[i + 1] < array[i] {
            return false;
        }
    }
    true
}
