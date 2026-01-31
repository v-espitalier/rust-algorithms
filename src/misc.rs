//! Miscellaneous Algorithms Collection
//!
//! This module provides implementations of various classic algorithms and problems,
//! including the 8-Queens puzzle, prime number search (with multithreading),
//! GCD calculation (with inline assembly), and the Collatz conjecture.
//! Author: Vincent Espitalier
//! Date: June 2024

use std::arch::asm;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

/// Finds the k-th free position in a given array of taken positions.
///
/// # Arguments
/// * `k` - The index of the free position to find.
/// * `depth` - The number of positions already taken.
/// * `taken_positions` - An array of taken positions.
///
/// # Returns
/// The index of the k-th free position.
fn find_kth_free_position(k: usize, depth: usize, taken_positions: &[usize; 8]) -> usize {
    let mut k_decrement = k;
    for i in 0..8 {
        let mut is_taken = false;
        for (_, &elem) in taken_positions.iter().enumerate().take(depth) {
            if elem == i {
                is_taken = true;
                break;
            }
        }
        if !is_taken {
            if k_decrement == 0 {
                return i;
            } else {
                k_decrement -= 1;
            }
        }
    }
    panic!("find_kth_free_position: Internal error");
}

/// Advances to the next position in the 8-Queens problem solution space.
fn advance_to_next_position(current_relative_solution: &mut [usize; 8], pruning_index: usize) {
    let max_index = 7 - pruning_index;
    if current_relative_solution[pruning_index] == max_index {
        current_relative_solution[pruning_index] = 0;
        advance_to_next_position(current_relative_solution, pruning_index - 1);
    } else {
        current_relative_solution[pruning_index] += 1;
    }
}

/// Solves the 8-Queens problem iteratively.
///
/// # Returns
/// A vector containing all unique solutions to the 8-Queens problem.
pub fn solve_8_queens_problem() -> Vec<[usize; 8]> {
    println!("Calling solve_8_queens_problem()");
    let mut solutions: Vec<[usize; 8]> = Vec::new();
    let mut current_relative_solution = [0; 8];
    let mut current_absolute_solution = [0; 8];
    let mut diag1 = [0; 8];
    let mut diag2 = [0; 8];
    let mut positions_tested = 0;

    loop {
        current_absolute_solution[0] = current_relative_solution[0];
        for i in 0..8 {
            current_absolute_solution[i] =
                find_kth_free_position(current_relative_solution[i], i, &current_absolute_solution);
        }

        let mut pruning_index = 8;
        'diagonal_check: for i in 0..8 {
            diag1[i] = i + current_absolute_solution[i];
            diag2[i] = (i as isize) - (current_absolute_solution[i] as isize);
            for j in 0..i {
                if diag1[i] == diag1[j] || diag2[i] == diag2[j] {
                    pruning_index = i;
                    break 'diagonal_check;
                }
            }
        }

        if pruning_index == 8 {
            solutions.push(current_absolute_solution);
            pruning_index = 7;
        }
        positions_tested += 1;

        advance_to_next_position(&mut current_relative_solution, pruning_index);
        if current_relative_solution[0] > 3 {
            println!("Positions tested with pruning: {}", positions_tested);
            break;
        }
    }

    let n_solutions_without_symmetry = solutions.len();
    for sol_index in 0..n_solutions_without_symmetry {
        let previous_solution = solutions[n_solutions_without_symmetry - 1 - sol_index];
        let mut new_solution = [0; 8];
        for i in 0..8 {
            new_solution[i] = 7 - previous_solution[i];
        }
        solutions.push(new_solution);
    }

    solutions
}

/// Displays all solutions to the 8-Queens problem.
pub fn display_8_queens_solutions(solutions: &[[usize; 8]]) {
    let colored_queen = "\x1b[93m*\x1b[0m";
    for (sol_index, solution) in solutions.iter().enumerate() {
        println!("Solution #{}", sol_index + 1);
        println!();
        println!("   a b c d e f g h");
        for i in 0..8 {
            let mut queen_column = 8;
            for (j, &elem) in solution.iter().enumerate() {
                if elem == 7 - i {
                    queen_column = j;
                    break;
                }
            }
            let leading_spaces = " -".repeat(queen_column);
            let trailing_spaces = " -".repeat(7 - queen_column);
            println!(
                "{} {}{}{}{}",
                8 - i,
                leading_spaces,
                colored_queen,
                trailing_spaces,
                8 - i
            );
        }
        println!("   a b c d e f g h");
        println!();
        println!();
    }
}

/// Generates all symmetries and rotations of a given 8-Queens solution.
pub fn generate_symmetries_and_rotations(solution: &[usize; 8]) -> Vec<[usize; 8]> {
    let mut multiple_solutions = Vec::new();
    multiple_solutions.push(*solution);

    let mut transformed_solution = [0; 8];
    // Vertical symmetry
    for i in 0..8 {
        transformed_solution[i] = 7 - solution[i];
    }
    multiple_solutions.push(transformed_solution);

    // Horizontal symmetry
    for i in 0..8 {
        transformed_solution[i] = solution[7 - i];
    }
    multiple_solutions.push(transformed_solution);

    // Central symmetry (180-degree rotation)
    for i in 0..8 {
        transformed_solution[i] = 7 - solution[7 - i];
    }
    multiple_solutions.push(transformed_solution);

    // Diagonal symmetry (main diagonal)
    for i in 0..8 {
        transformed_solution[solution[i]] = i;
    }
    multiple_solutions.push(transformed_solution);

    // Diagonal symmetry (anti-diagonal)
    for i in 0..8 {
        transformed_solution[7 - solution[i]] = 7 - i;
    }
    multiple_solutions.push(transformed_solution);

    // 90-degree rotation (counter-clockwise)
    for (i, elem) in transformed_solution.iter_mut().enumerate() {
        let mut correct_index = 8;
        for (j, &value) in solution.iter().enumerate() {
            if value == i {
                correct_index = j;
                break;
            }
        }
        *elem = 7 - correct_index;
    }
    multiple_solutions.push(transformed_solution);

    // 90-degree rotation (clockwise)
    for i in 0..8 {
        let mut correct_index = 8;
        for (j, &value) in solution.iter().enumerate() {
            if value == i {
                correct_index = j;
                break;
            }
        }
        transformed_solution[7 - i] = correct_index;
    }
    multiple_solutions.push(transformed_solution);

    multiple_solutions
}

/// Extracts the 12 unique solutions from all solutions found.
pub fn extract_unique_solutions(solutions: &Vec<[usize; 8]>) -> Vec<[usize; 8]> {
    let mut unique_solutions: Vec<[usize; 8]> = Vec::new();
    let mut multiple_solutions: Vec<[usize; 8]> = Vec::new();

    for solution in solutions {
        let current_solution = solution;
        let mut found = false;
        for existing_solution in &multiple_solutions {
            let mut is_identical = true;
            for j in 0..8 {
                if current_solution[j] != existing_solution[j] {
                    is_identical = false;
                    break;
                }
            }
            if is_identical {
                found = true;
                break;
            }
        }
        if !found {
            unique_solutions.push(*current_solution);
            let current_multiple_solutions = generate_symmetries_and_rotations(current_solution);
            for sym_solution in &current_multiple_solutions {
                multiple_solutions.push(*sym_solution);
            }
        }
    }
    unique_solutions
}

/// Computes GCD using x86 assembly (Euclidean algorithm).
///
/// # Arguments
/// * `a`, `b` - Non-negative integers.
///
/// # Returns
/// Greatest common divisor of `a` and `b`.
///
/// # Example
/// ```rust
/// assert_eq!(gcd_asm(48, 18), 6);
/// ```
///
/// # Note
/// Uses unsafe inline assembly. For x86_64 only.
pub fn gcd_asm(a: u64, b: u64) -> u64 {
    println!("Calling gcd_asm");
    if a < b {
        return gcd_asm(b, a);
    }

    let mut result = a;
    unsafe {
        asm!(
            "123:",
            "cmp ecx, 0",
            "je 456f",
            "mov edx, 0",
            "div ecx",
            "mov eax, ecx",
            "mov ecx, edx",
            "jmp 123b",
            "456:",
            inout("eax") result,
            in("ecx") b
        );
    }
    result
}

/// Finds primes between `min_n` (inclusive) and `max_n` (exclusive).
///
/// Uses trial division optimized for odd numbers. Automatically includes 2.
///
/// # Arguments
/// * `min_n`, `max_n` - Range bounds.
///
/// # Returns
/// Vector of primes in [min_n, max_n).
///
/// # Example
/// ```rust
/// assert_eq!(find_primes(10, 20), vec![11, 13, 17, 19]);
/// ```
///
/// # Complexity
/// O(n√n) for range [min_n, max_n).
pub fn find_primes(min_n: usize, max_n: usize) -> Vec<usize> {
    let mut primes_found = Vec::new();
    if min_n <= 2 && max_n > 2 {
        primes_found.push(2);
    }

    let min_odd_n = if min_n <= 2 {
        3
    } else {
        min_n + (1 - min_n % 2)
    };
    for i in (min_odd_n..max_n).step_by(2) {
        let max_j = (i as f64).sqrt() as usize;
        let mut is_prime = true;
        for j in (3..=max_j).step_by(2) {
            if i % j == 0 {
                is_prime = false;
                break;
            }
        }
        if is_prime {
            primes_found.push(i);
        }
    }
    primes_found
}

/// Finds prime numbers between `min_n` (inclusive) and `max_n` (exclusive) using multithreading.
///
/// Processes the range in batches of size `batch_size`, distributed across CPU cores.
/// If `min_n` and `max_n` differ significantly in magnitude, early batches will finish faster.
/// Recommended `batch_size` is approximately (max_n - min_n) / 100 to create around 100 batches,
/// balancing load distribution while limiting thread count.
/// Faster than single-threaded version for ranges above 1-2 million numbers.
pub fn find_primes_multithreaded(min_n: usize, max_n: usize, batch_size: usize) -> Vec<usize> {
    println!("Calling find_primes_multithreaded");
    if batch_size == 0 {
        panic!("Error in find_primes_multithreaded: batch_size must be non-zero");
    }

    let n_element: usize = max_n - min_n + 1;
    let n_batch: usize = ((n_element as f64) / (batch_size as f64)).ceil() as usize;

    static GLOBAL_THREAD_COUNT: AtomicUsize = AtomicUsize::new(0);

    let primes_found: Vec<usize> = Vec::new();
    let shared_primes_found = Arc::new(Mutex::new(primes_found));

    for batch_index in 0..n_batch {
        let min_n_batch: usize = min_n + batch_size * batch_index;
        let max_n_batch_complete: usize = min_n + batch_size * (batch_index + 1);
        let max_n_batch: usize = if max_n_batch_complete < max_n {
            max_n_batch_complete
        } else {
            max_n
        };

        // Only the smart pointer to the vector is cloned, not the vector itself
        let shared_primes_found_batch = shared_primes_found.clone();

        // Parallelized section
        GLOBAL_THREAD_COUNT.fetch_add(1, Ordering::SeqCst);
        let _handle = std::thread::spawn(move || {
            let mut primes_found_batch: Vec<usize> = find_primes(min_n_batch, max_n_batch);

            let mut shared_primes_found_batch_val = shared_primes_found_batch.lock().unwrap();

            // Requires mutex/arc for thread-safe access
            shared_primes_found_batch_val.append(&mut primes_found_batch);

            GLOBAL_THREAD_COUNT.fetch_sub(1, Ordering::SeqCst);
            std::thread::sleep(std::time::Duration::from_millis(1));
        });
        //handle.join();
    }

    println!("Waiting for threads...");
    while GLOBAL_THREAD_COUNT.load(Ordering::SeqCst) != 0 {
        thread::sleep(Duration::from_millis(1));
    }

    println!("All threads completed");
    return shared_primes_found.lock().unwrap().clone();
}

/// Calculates the flight time and maximum altitude for a Collatz sequence starting at `n`.
pub fn calculate_collatz_flight_time_and_max_altitude(n: u64) -> (u64, u64) {
    let mut flight_time = 0;
    let mut max_altitude = n;
    let mut current_n = n;

    while current_n != 1 {
        if current_n.is_multiple_of(2) {
            current_n /= 2;
        } else {
            current_n = 3 * current_n + 1;
        }
        if current_n > max_altitude {
            max_altitude = current_n;
        }
        flight_time += 1;
    }
    (flight_time, max_altitude)
}

/// Finds the maximum flight time for Collatz sequences up to `n_max`.
pub fn find_max_collatz_flight_time(n_max: u64) -> (u64, u64) {
    let mut max_flight_time = 0;
    let mut max_flight_time_index = 0;

    for n in 1..=n_max {
        let (flight_time, _) = calculate_collatz_flight_time_and_max_altitude(n);
        if flight_time > max_flight_time {
            max_flight_time = flight_time;
            max_flight_time_index = n;
        }
    }
    (max_flight_time, max_flight_time_index)
}
