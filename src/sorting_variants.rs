//! Sorting Algorithm Variants
//!
//! This module contains alternative implementations of sorting algorithms:
//! - Generic implementations using Rust traits
//! - Indirect sorting (returns permutation indices instead of sorted data)
//! - Optimized implementations with reduced memory allocations
//! - Modified versions of sorting algorithms from literature
//!
//! Author: Vincent Espitalier
//! Date: June 2024

#![allow(dead_code)]

/// Generic implementation of insertion sort.
///
/// Sorts any array whose elements implement PartialOrd and Clone traits.
/// Works with u32, i64, f32, String, and other comparable and cloneable types.
///
/// # Type Parameters
/// * `T` - Type of elements to sort (must implement PartialOrd and Clone)
///
/// # Arguments
/// * `array` - Mutable slice of elements to sort
///
/// # Example
/// ```
/// let mut arr = [5, 2, 4, 6, 1, 3];
/// generic_insertion_sort(&mut arr);
/// assert_eq!(arr, [1, 2, 3, 4, 5, 6]);
/// ```
pub fn generic_insertion_sort<T>(array: &mut [T])
where
    T: PartialOrd + Clone,
{
    println!("generic_insertion_sort > called");

    let n = array.len();

    // Sort elements of the array successively
    for i in 0..n {
        let current = array[i].clone();

        // Shift elements greater than current to make space for insertion
        let mut insert_index = 0;
        for j in (0..i).rev() {
            // Stop when elements are smaller than current
            // In case of equality, break to maintain stability
            if array[j] <= current {
                insert_index = j + 1;
                break;
            }
            array[j + 1] = array[j].clone();
        }

        // Insert the current element at the correct position
        array[insert_index] = current;

        // Loop invariant:
        // After each iteration, the first (i+1) elements of the array are sorted
    }
}

/// Finds the minimum element in an array using indirect indexing.
///
/// # Type Parameters
/// * `T` - Type of elements to compare (must implement Ord)
///
/// # Arguments
/// * `array` - Array of elements
/// * `permutation` - Array of indices to consider
///
/// # Returns
/// Index of the minimum element in the permutation
fn find_min_indirect<T: Ord>(array: &[T], permutation: &[usize]) -> usize {
    let n = permutation.len();
    let mut min_index = 0;
    for i in 1..n {
        if array[permutation[i]] < array[permutation[min_index]] {
            min_index = i;
        }
    }
    min_index
}

/// Generic indirect selection sort with branchless implementation.
///
/// Returns the permutation indices that would sort the array.
/// The original array is not modified.
///
/// # Type Parameters
/// * `T` - Type of elements to sort (must implement Ord)
///
/// # Arguments
/// * `array` - Array of elements to sort
///
/// # Returns
/// Vector of indices representing the permutation to sort the array
///
/// # Example
/// ```
/// let arr = [5, 2, 4, 6, 1, 3];
/// let permutation = generic_indirect_selection_sort(&arr);
/// let sorted = permute_copy_array(&arr, &permutation);
/// assert_eq!(sorted, [1, 2, 3, 4, 5, 6]);
/// ```
pub fn generic_indirect_selection_sort<T>(array: &[T]) -> Vec<usize>
where
    T: Ord,
{
    println!("generic_indirect_selection_sort > called");

    let n = array.len();
    let mut permutation: Vec<usize> = (0..n).collect();

    // Sort elements of the array successively
    for i in 0..n {
        let min_index = i + find_min_indirect(array, &permutation[i..n]);

        // Swap indices i and min_index in the permutation
        permutation.swap(i, min_index);
    }

    permutation
}

/// Creates a new sorted array by applying a permutation to the original array.
///
/// # Type Parameters
/// * `T` - Type of elements to permute (must implement Ord and Clone)
///
/// # Arguments
/// * `array` - Original array
/// * `permutation` - Permutation indices
///
/// # Returns
/// New array containing the sorted elements
///
/// # Example
/// ```
/// let arr = [5, 2, 4, 6, 1, 3];
/// let permutation = vec![4, 1, 5, 2, 0, 3];
/// let sorted = permute_copy_array(&arr, &permutation);
/// assert_eq!(sorted, [1, 2, 3, 4, 5, 6]);
/// ```
pub fn permute_copy_array<T>(array: &[T], permutation: &[usize]) -> Vec<T>
where
    T: Ord + Clone,
{
    let n = array.len();
    let mut sorted_array = Vec::with_capacity(n);

    for i in 0..n {
        sorted_array.push(array[permutation[i]].clone());
    }

    sorted_array
}

/// Optimized merge sort implementation with reduced memory allocations.
///
/// Uses only one additional allocation (half the size of the original array)
/// during the entire sorting process.
///
/// # Arguments
/// * `array` - Mutable slice of i32 to sort
/// * `min_index` - Optional minimum index for the current subarray
/// * `max_index` - Optional maximum index for the current subarray
/// * `temp_array` - Optional temporary array for merging
///
/// # Note
/// On the first call, only provide the array parameter. The function will
/// handle the initial allocation and call itself recursively with proper parameters.
pub fn optimized_merge_sort(
    array: &mut [i32],
    min_index: Option<usize>,
    max_index: Option<usize>,
    temp_array: Option<&mut [i32]>,
) {
    let missing_args = min_index.is_none() || max_index.is_none() || temp_array.is_none();

    if missing_args {
        println!("Calling optimized_merge_sort");
        // Handle the initial user call (non-recursive)
        let array_len = array.len();
        let temp_array_len = array_len / 2 + 1;
        // Single allocation for this algorithm - performed once during user call
        let mut temp_array_vec = vec![0; temp_array_len];
        let temp_array_slice = temp_array_vec.as_mut_slice();

        // Recall the function with allocated array and proper indices
        return optimized_merge_sort(array, Some(0), Some(array_len - 1), Some(temp_array_slice));
    }

    // From this point, we know all optional arguments are provided
    let min_idx = min_index.unwrap();
    let max_idx = max_index.unwrap();
    let temp_arr = temp_array.unwrap();

    let n = max_idx - min_idx + 1;

    // Base case: array with 0 or 1 element (nothing to sort)
    if n <= 1 {
        return;
    }

    // Base case: array with 2 elements
    if n == 2 {
        if array[min_idx] > array[max_idx] {
            array.swap(min_idx, max_idx);
        }
        return;
    }

    // General case: split, sort sub-arrays, and merge
    let mid = (min_idx + max_idx) / 2;
    let mid_plus_1 = mid + 1;

    // Recursive calls to sort sub-arrays
    optimized_merge_sort(array, Some(min_idx), Some(mid), Some(temp_arr));
    optimized_merge_sort(array, Some(mid_plus_1), Some(max_idx), Some(temp_arr));

    // Merge the two sub-arrays
    let left_len = mid - min_idx + 1;
    temp_arr[..left_len].copy_from_slice(&array[min_idx..=mid]);

    let mut merge_index_left = 0;
    let mut merge_index_right = mid_plus_1;

    for merge_index_output in min_idx..=max_idx {
        let left_not_exhausted = merge_index_left <= (mid - min_idx);
        let right_not_exhausted = merge_index_right <= max_idx;

        if left_not_exhausted {
            if right_not_exhausted {
                if array[merge_index_right] < temp_arr[merge_index_left] {
                    // Copy element from right array
                    array[merge_index_output] = array[merge_index_right];
                    merge_index_right += 1;
                } else {
                    // In case of equality, prefer element from left array
                    // This maintains stability

                    // Copy element from left array
                    array[merge_index_output] = temp_arr[merge_index_left];
                    merge_index_left += 1;
                }
            } else {
                // Right array exhausted
                // Copy element from left array
                array[merge_index_output] = temp_arr[merge_index_left];
                merge_index_left += 1;
            }
        } else if right_not_exhausted {
            // Left array exhausted
            // Copy element from right array
            array[merge_index_output] = array[merge_index_right];
            merge_index_right += 1;
        }
    }
}
