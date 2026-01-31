//! This module provides idiomatic Rust implementations of fundamental algorithms,
//! including mathematical functions (factorial, GCD), sequence computations (Fibonacci),
//! search algorithms (linear, binary), and the Tower of Hanoi puzzle.
//!
//! Each function is documented with:
//! - A description of the algorithm
//! - Time complexity
//! - Usage examples
//! - References to further reading
//!
//! Author: Vincent Espitalier
//! Date: June 2024

use crate::sorting::is_array_sorted;

/// Recursive implementation of the factorial function.
///
/// # Arguments
/// * `n` - A non-negative integer.
///
/// # Returns
/// The factorial of `n`, i.e., `n! = n * (n - 1) * (n - 2) * ... * 1`.
///
/// # Complexity
/// Linear: O(n)
///
/// # Example
/// ```
/// assert_eq!(factorial(5), 120);
/// ```
///
/// # See also
/// [Factorial - Wikipedia](https://en.wikipedia.org/wiki/Factorial)
pub fn factorial(n: u64) -> u64 {
    if n <= 1 {
        1
    } else {
        n * factorial(n - 1)
    }
}

/// Recursive implementation of the greatest common divisor (GCD) of two integers.
///
/// # Arguments
/// * `a` - First integer.
/// * `b` - Second integer.
///
/// # Returns
/// The GCD of `a` and `b`.
///
/// # Complexity
/// Logarithmic: O(log(min(a, b)))
///
/// # Example
/// ```
/// assert_eq!(gcd(48, 18), 6);
/// ```
///
/// # See also
/// [Greatest common divisor - Wikipedia](https://en.wikipedia.org/wiki/Greatest_common_divisor)
pub fn gcd(mut a: u64, mut b: u64) -> u64 {
    // Swap a and b if a < b is handled by the loop
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

/// Iterative implementation to compute the nth element of the Fibonacci sequence.
///
/// # Arguments
/// * `n` - A non-negative integer.
///
/// # Returns
/// The nth element of the Fibonacci sequence.
///
/// # Complexity
/// Linear: O(n)
///
/// # Example
/// ```
/// assert_eq!(fibonacci_iterative(6), 8);
/// ```
///
/// # See also
/// [Fibonacci sequence - Wikipedia](https://en.wikipedia.org/wiki/Fibonacci_sequence#Polynomial-time_algorithms)
pub fn fibonacci_iterative(n: u64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => {
            let (mut prev, mut curr) = (0, 1);
            for _ in 2..=n {
                let next = prev + curr;
                prev = curr;
                curr = next;
            }
            curr
        }
    }
}

/// Recursive implementation to compute the nth element of the Fibonacci sequence.
///
/// # Arguments
/// * `n` - A non-negative integer.
///
/// # Returns
/// The nth element of the Fibonacci sequence.
///
/// # Complexity
/// Exponential: O(2^n) (naive implementation with many repeated calculations)
///
/// # Example
/// ```
/// assert_eq!(fibonacci_recursive(6), 8);
/// ```
///
/// # See also
/// [Fibonacci sequence - Wikipedia](https://en.wikipedia.org/wiki/Fibonacci_sequence#Recursive_definition)
pub fn fibonacci_recursive(n: u64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci_recursive(n - 1) + fibonacci_recursive(n - 2),
    }
}

/// Linear search in a slice of integers.
///
/// # Arguments
/// * `slice` - A slice of integers.
/// * `target` - The value to search for.
///
/// # Returns
/// `Some(index)` if the value is found, `None` otherwise.
///
/// # Complexity
/// Linear: O(n)
///
/// # Example
/// ```
/// let slice = [1, 2, 3, 4, 5];
/// assert_eq!(linear_search(&slice, 3), Some(2));
/// ```
///
/// # See also
/// [Linear search - Wikipedia](https://en.wikipedia.org/wiki/Linear_search)
pub fn linear_search(slice: &[i32], target: i32) -> Option<usize> {
    println!("Call to linear_search.");
    slice
        .iter()
        .enumerate()
        .find(|(_, &x)| x == target)
        .map(|(i, _)| i)
}

/// Generic linear search for any type that implements `Eq`.
///
/// # Arguments
/// * `slice` - A slice of elements of type `T`.
/// * `target` - The value to search for.
///
/// # Returns
/// `Some(index)` if the value is found, `None` otherwise.
///
/// # Complexity
/// Linear: O(n)
///
/// # Example
/// ```
/// let slice = ["a", "b", "c"];
/// assert_eq!(generic_linear_search(&slice, "b"), Some(1));
/// ```
pub fn generic_linear_search<T>(slice: &[T], target: T) -> Option<usize>
where
    T: core::cmp::Eq,
{
    println!("Call to generic_linear_search.");
    //slice.iter().enumerate().find(|(_, &ref x)| *x == target).map(|(i, _)| i)
    slice
        .iter()
        .enumerate()
        .find(|(_, x)| **x == target)
        .map(|(i, _)| i)
}

#[allow(dead_code)]
/// Binary search in a sorted slice of integers.
///
/// # Arguments
/// * `slice` - A sorted slice of integers.
/// * `target` - The value to search for.
/// * `min_index` - Optional lower bound index for the search.
/// * `max_index_inclusive` - Optional upper bound index for the search.
///
/// # Returns
/// `Some(index)` if the value is found, `None` otherwise.
///
/// # Panics
/// Panics if the input slice is not sorted in ascending order.
///
/// # Complexity
/// Logarithmic: O(log n)
///
/// # Example
/// ```
/// let slice = [1, 2, 3, 4, 5];
/// assert_eq!(binary_search(&slice, 3, None, None), Some(2));
/// ```
///
/// # See also
/// [Binary search - Wikipedia](https://en.wikipedia.org/wiki/Binary_search_algorithm)
pub fn binary_search(
    slice: &[i32],
    target: i32,
    min_index: Option<usize>,
    max_index_inclusive: Option<usize>,
) -> Option<usize> {
    assert!(
        is_array_sorted(slice),
        "(binary_search) Error: the slice is not sorted in ascending order (must be sorted first)."
    );

    let n = slice.len();
    let min_index = min_index.unwrap_or(0);
    let max_index_inclusive = max_index_inclusive.unwrap_or(n - 1);

    if min_index > max_index_inclusive {
        println!("Error: binary_search: internal error, bounds are reversed");
        return None;
    }

    if min_index == max_index_inclusive {
        return if slice[min_index] == target {
            Some(min_index)
        } else {
            None
        };
    }

    if max_index_inclusive == min_index + 1 {
        if slice[min_index] == target {
            return Some(min_index);
        }
        if slice[max_index_inclusive] == target {
            return Some(max_index_inclusive);
        }
        return None;
    }

    let mid_index = (min_index + max_index_inclusive) / 2;
    let mid_value = slice[mid_index];

    if target > mid_value {
        binary_search(slice, target, Some(mid_index), Some(max_index_inclusive))
    } else {
        binary_search(slice, target, Some(min_index), Some(mid_index))
    }
}

/// Represents a Tower of Hanoi game.
struct HanoiGame {
    towers: [Vec<u32>; 3],
    verbose: bool,
}

/// Returns the top element of a slice, if it exists.
fn top<T>(slice: &[T]) -> Option<T>
where
    T: Copy,
{
    slice.last().copied()
}

impl HanoiGame {
    /// Creates a new Tower of Hanoi game with `n` disks.
    pub fn new(n: u32, verbose: bool) -> Self {
        Self {
            towers: [(1..=n).rev().collect(), Vec::new(), Vec::new()],
            verbose,
        }
    }

    /// Moves the top disk from `src` to `dest`.
    ///
    /// # Panics
    /// Panics if the move is invalid.
    pub fn move_disk(&mut self, src: usize, dest: usize) {
        if !(1..=3).contains(&src) || !(1..=3).contains(&dest) {
            panic!("Error: Indices must be in [1, 3].");
        }
        if self.towers[src - 1].is_empty() {
            panic!("The source tower is empty.");
        }
        if !self.towers[dest - 1].is_empty()
            && top(&self.towers[dest - 1]).unwrap() < top(&self.towers[src - 1]).unwrap()
        {
            println!(
                "src: index {} val {:?}, dest: index {} val {:?}",
                src,
                self.towers[src - 1],
                dest,
                self.towers[dest - 1]
            );
            panic!("Error: The top disk of the destination tower must be larger than the top disk of the source tower");
        }

        let disk = self.towers[src - 1].pop().unwrap();
        self.towers[dest - 1].push(disk);
        if self.verbose {
            self.display();
        }
    }

    /// Displays the current state of the towers.
    pub fn display(&self) {
        println!("{:?}", self.towers);
    }
}

/// Recursively moves `n` disks from `src` to `dest` in the Tower of Hanoi game.
fn move_tower_recursive(hanoi: &mut HanoiGame, src: usize, dest: usize, n: u32) {
    match n {
        0 => (),
        1 => hanoi.move_disk(src, dest),
        _ => {
            let aux_tower = 6 - src - dest;
            move_tower_recursive(hanoi, src, aux_tower, n - 1);
            move_tower_recursive(hanoi, src, dest, 1);
            move_tower_recursive(hanoi, aux_tower, dest, n - 1);
        }
    }
}

/// Solves the Tower of Hanoi problem for `n` disks.
///
/// # Arguments
/// * `n` - The number of disks.
///
/// # Example
/// ```
/// solve_tower_of_hanoi(3);
/// ```
///
/// # See also
/// [Tower of Hanoi - Wikipedia](https://en.wikipedia.org/wiki/Tower_of_Hanoi)
pub fn solve_tower_of_hanoi(n: u32) {
    let verbose = true;
    println!("Solving the Tower of Hanoi problem with {} disk(s).", n);
    let mut hanoi = HanoiGame::new(n, verbose);
    hanoi.display();
    move_tower_recursive(&mut hanoi, 1, 3, n);
}
