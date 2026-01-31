//! Main Program
//!
//! Entry point for testing various algorithm implementations.
//! Includes tests for:
//! - Classic algorithms (factorial, GCD, Fibonacci, Tower of Hanoi)
//! - Sorting algorithms and variants
//! - Probability and random number generation
//! - Rational numbers
//! - File operations
//! - Integer conversions
//! - Graph/maze solving
//! - Fractal generation
//!
//! Author: Vincent Espitalier
//! Date: June 2024

mod classics;
mod conversions_hexa_bin_dec;
mod files;
mod fractals;
mod graphs_mazes;
mod misc;
mod probabilities;
mod rational;
mod sorting;
mod sorting_variants;

#[cfg(test)]
mod tests;

use std::time::{SystemTime, UNIX_EPOCH};

// Get current time in seconds since epoch
fn get_current_time_epoch() -> f64 {
    (SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis() as f64)
        / 1000.
}

fn main() {
    println!("Hello, world!");

    let test_classics = true;
    let test_search_and_sort = true;
    let test_sort_variants = true;
    let test_probabilities = true;
    let test_misc_algorithms = true;
    let test_rationals = true;
    let test_files = true;
    let test_integer_conversions = true;
    let test_graphs = true;
    let test_fractals = true;

    // Test mathematical functions: factorial, GCD, Fibonacci
    if test_classics {
        println!();
        let n: u64 = 5;
        println!("Factorial({}) = {}", n, classics::factorial(n));

        let a: u64 = 90;
        let b: u64 = 28;
        println!("GCD({}, {}) = {}\n", a, b, classics::gcd(a, b));
        println!("GCD ASM({}, {}) = {}\n", a, b, misc::gcd_asm(a, b));

        let n = 7;
        for i in 0..n {
            println!(
                "Fibonacci iterative({}) = {}",
                i,
                classics::fibonacci_iterative(i)
            );
            println!(
                "Fibonacci recursive({}) = {}",
                i,
                classics::fibonacci_recursive(i)
            );
        }

        for n in 1..6 {
            classics::solve_tower_of_hanoi(n);
            println!("\n");
        }
    }

    if test_search_and_sort {
        println!();

        let seed: u32 = 1234;
        let n = 13;

        let mut array: Vec<i32> = Vec::from_iter(0..n);
        let array_slice: &mut [i32] = array.as_mut_slice();

        println!("\nInitial array: \n {:?}", &array_slice);
        probabilities::fisher_yates_shuffle(array_slice, seed);
        println!("\nShuffled array: \n {:?}\n", &array_slice);

        let value = 8;
        println!(
            "Linear search for value {}: index {} \n",
            value,
            classics::linear_search(array_slice, value).unwrap()
        );

        let value = 12;
        println!(
            "Generic linear search for value {}: index {} \n",
            value,
            classics::generic_linear_search(array_slice, value).unwrap()
        );

        sorting::heap_sort(array_slice);
        println!("Sorted array: \n{:?}", &array_slice);
        assert!(
            sorting::is_array_sorted(array_slice),
            "Error: array is not correctly sorted."
        );
    }

    if test_sort_variants {
        println!();
        let mut string_array: Vec<String> = vec![
            "rust".to_string(),
            "go".to_string(),
            "shell".to_string(),
            "ruby".to_string(),
            "python".to_string(),
        ];
        let string_array_slice: &mut [String] = string_array.as_mut_slice();

        println!("\nInitial array: \n {:?}", &string_array_slice);
        sorting_variants::generic_insertion_sort(string_array_slice);
        println!("Sorted array: \n{:?}", &string_array_slice);
        assert!(
            sorting::is_array_sorted(string_array_slice),
            "Error: array is not correctly sorted."
        );
    }

    if test_probabilities {
        println!();
        let seed: u32 = 1234;
        let n: usize = 10000;
        let normals: Vec<f64> = probabilities::box_muller(n, seed);
        let mean = probabilities::mean(normals.as_slice()).unwrap();
        let variance = probabilities::variance(normals.as_slice(), None).unwrap();
        println!("Mean, variance: {} {}", mean, variance);
    }

    if test_misc_algorithms {
        println!();
        let solutions = misc::solve_8_queens_problem();
        println!("Number of solutions: {}", solutions.len());
        let unique_solutions = misc::extract_unique_solutions(&solutions);
        println!("Number of unique solutions: {}", unique_solutions.len());
        println!(" ");
        misc::display_8_queens_solutions(&unique_solutions);

        println!(" ");

        let min_n: usize = 0;
        let max_n: usize = 2000000;

        let start_time = get_current_time_epoch();
        println!("Testing find_primes() min_n:{}, max_n:{}", min_n, max_n);
        let primes: Vec<usize> = misc::find_primes(min_n, max_n);
        println!("Total primes found: {}", primes.len());
        let end_time = get_current_time_epoch();
        let prime_search_duration = end_time - start_time;
        println!("Calculation duration: {}", prime_search_duration);

        let start_time = get_current_time_epoch();
        println!("\n");
        let batch_size: usize = (max_n - min_n) / 40;
        println!(
            "Testing find_primes_multithreaded() min_n:{}, max_n:{}, batch_size:{}",
            min_n, max_n, batch_size
        );
        let primes: Vec<usize> = misc::find_primes_multithreaded(min_n, max_n, batch_size);
        println!("Total primes found: {}", primes.len());
        let end_time = get_current_time_epoch();
        let multithread_prime_search_duration = end_time - start_time;
        println!(
            "Calculation duration: {}",
            multithread_prime_search_duration
        );
        println!(
            "Duration ratio (multithread/singlethread): {}",
            prime_search_duration / multithread_prime_search_duration
        );

        println!("\nTesting Collatz conjecture");
        let n: u64 = 15;
        let (flight_time, max_altitude) = misc::calculate_collatz_flight_time_and_max_altitude(n);
        println!(
            "n = {}; Flight time = {}; Max altitude = {}",
            n, flight_time, max_altitude
        );

        let n: u64 = 27;
        let (flight_time, max_altitude) = misc::calculate_collatz_flight_time_and_max_altitude(n);
        println!(
            "n = {}; Flight time = {}; Max altitude = {}",
            n, flight_time, max_altitude
        );

        let n_max: u64 = 100;
        let (max_flight_time, max_flight_time_index) = misc::find_max_collatz_flight_time(n_max);
        println!(
            "n_max = {}; Max flight time = {}; Index = {}",
            n_max, max_flight_time, max_flight_time_index
        );

        let n_max: u64 = 1000000;
        let (max_flight_time, max_flight_time_index) = misc::find_max_collatz_flight_time(n_max);
        println!(
            "n_max = {}; Max flight time = {}; Index = {}",
            n_max, max_flight_time, max_flight_time_index
        );
    }

    if test_rationals {
        // Test addition
        let r1 = rational::Rational::new(2i64, 3i64);
        let r2 = rational::Rational::new(5i64, 6i64);
        let r3 = &r1 + &r2;
        println!("{} + {} = {} (addition with references)", &r1, &r2, &r3);

        let r3 = r1 + r2;
        println!("Same sum = {} (addition without reference)", &r3);

        // Test AddAssign
        let r1 = rational::Rational::new(2i64, 3i64);
        let mut r2 = rational::Rational::new(5i64, 6i64);
        r2 += r1;
        println!("Same sum = {} (addition with '+=' operator)\n", &r3);

        // Test subtraction
        let r1 = rational::Rational::new(2i64, 3i64);
        let r2 = rational::Rational::new(5i64, 6i64);
        let r3 = &r1 - &r2;
        println!("{} - {} = {} (subtraction with references)", &r1, &r2, &r3);

        let r3 = r1 - r2;
        println!("Same subtraction = {} (without reference)", &r3);

        // Test SubAssign
        let r1 = rational::Rational::new(2i64, 3i64);
        let mut r2 = rational::Rational::new(5i64, 6i64);
        r2 -= r1;
        println!(
            "Same subtraction = {} (subtraction with '-=' operator)\n",
            &r3
        );

        // Test multiplication
        let r1 = rational::Rational::new(2i64, 3i64);
        let r2 = rational::Rational::new(5i64, 6i64);
        let r3 = &r1 * &r2;
        println!(
            "{} * {} = {} (multiplication with references)",
            &r1, &r2, &r3
        );

        let r3 = r1 * r2;
        println!("Same multiplication = {} (without reference)\n", &r3);

        // Test division
        let r1 = rational::Rational::new(2i64, 3i64);
        let r2 = rational::Rational::new(5i64, 6i64);
        let r3 = &r1 / &r2;
        println!("{} / {} = {} (division with references)", &r1, &r2, &r3);

        let r3 = r1 / r2;
        println!("Same division = {} (without reference)\n", &r3);

        // Test negation
        let r1 = rational::Rational::new(2i64, 3i64);
        let r1_neg = -&r1;
        println!("-({}) = {} (negation with reference)", &r1, &r1_neg);
        let r1_neg = -r1;
        println!("    = {} (without reference)\n", &r1_neg);

        // Test comparison
        let r1 = rational::Rational::new(2i64, 3i64);
        let r2 = rational::Rational::new(5i64, 6i64);
        println!("{} >= {}? Answer: {}", &r1, &r2, r1 >= r2);
        println!("{} > {}? Answer: {}", &r1, &r2, r1 > r2);
        println!("{} <= {}? Answer: {}", &r1, &r2, r1 <= r2);
        println!("{} < {}? Answer: {}", &r1, &r2, r1 < r2);
        #[allow(clippy::eq_op)]
        let cmp = r1 >= r1;
        println!("{} >= {}? Answer: {}", &r1, &r1, cmp);
        #[allow(clippy::eq_op)]
        let cmp = r1 <= r1;
        println!("{} <= {}? Answer: {}", &r1, &r1, cmp);

        // Test comparison via a generic sorting algorithm
        let mut rational_array: Vec<rational::Rational<i64>> = vec![
            rational::Rational::new(2, 3),
            rational::Rational::new(-2, 3),
            rational::Rational::new(-1, 2),
            rational::Rational::new(2, 5),
            rational::Rational::new(-7, 5),
        ];

        let rational_array_slice: &mut [rational::Rational<i64>] = rational_array.as_mut_slice();
        println!("\nInitial array: \n {:?}", &rational_array_slice);
        sorting_variants::generic_insertion_sort(rational_array_slice);
        println!("\nSorted array: \n {:?}\n", &rational_array_slice);

        // Test rational to float conversion
        let r1 = rational::Rational::new(2i32, 3i32);
        let r1_clone = r1.clone();
        let r1_float: f64 = f64::from(r1);
        println!("{} = {}", &r1_clone, r1_float);

        // Test integer to rational conversion
        let integer_i64: i64 = 5;
        let r1 = rational::Rational::<i64>::from(integer_i64);
        println!("{} = {}", &integer_i64, r1);
    }

    if test_files {
        let file_path = "./divers/my_file.txt".to_string();
        let content_to_write: String = "First line\nSecond line".to_string();
        println!("Writing to file {}", file_path);
        files::write_text_file(&file_path, &content_to_write);
        println!("Content written: {}", content_to_write);
        println!(" ");

        let exists: bool = files::test_file_existence(&file_path);
        println!("File {} exists: {}", file_path, exists);
        println!(" ");

        let file_content: String = files::read_text_file(&file_path);
        println!("File content:");
        println!("{:?}", &file_content);
        println!(" ");

        let lines: Vec<String> = files::read_text_file_lines(&file_path, None);
        println!("Lines read with default separator:");
        println!("{:?}", &lines);
        println!(" ");

        let binary_content: Vec<u8> = vec![0x42, 0x6f, 0x6e, 0x6a, 0x6f, 0x75, 0x72];
        let binary_file_path = "./divers/my_file.dat".to_string();
        println!("Writing to file {}", binary_file_path);
        files::write_binary_file(&binary_file_path, &binary_content);
        println!("Content written: {:?}", binary_content);
        println!(" ");

        let read_binary_content: Vec<u8> = files::read_binary_file(&binary_file_path);
        assert_eq!(
            binary_content, read_binary_content,
            "Error: Content differs from expected"
        );
        println!("File content:");
        println!("{:?}", &read_binary_content);
        let content: String = String::from_utf8(binary_content.clone()).unwrap();
        println!("{}", &content);
        println!(" ");

        let directory_path = "./".to_string();
        let directory_content: Vec<String> = files::list_directory(&directory_path);
        println!("Content of directory '{}':", directory_path);
        println!("{:?}", &directory_content);
        println!(" ");

        let file_path = "./divers/my_file.txt".to_string();
        let size = files::get_file_size(&file_path);
        println!("Size of file '{}': {:?}", file_path, size);
        println!(" ");

        let file_path = "./divers/my_file.txt".to_string();
        let file_info = files::get_file_info(&file_path);
        println!("Type of file '{}': {:?}", file_path, file_info);
        println!(" ");

        let file_path = "./.git".to_string();
        let file_info = files::get_file_info(&file_path);
        println!("Type of file '{}': {:?}", file_path, file_info);
        println!(" ");
    }

    if test_integer_conversions {
        conversions_hexa_bin_dec::integer_conversions();
    }

    if test_graphs {
        let maze_directory = "divers/labyrinthes".to_string();
        let maze_file_list: Vec<String> = files::list_directory(&maze_directory);

        for maze_file in maze_file_list {
            if maze_file.contains("solution") {
                continue;
            }

            let solution_file =
                maze_file.split('.').collect::<Vec<_>>()[0].to_string() + "_solution.txt";
            graphs_mazes::solve_maze(maze_file, solution_file);
        }
    }

    if test_fractals {
        let height: u32 = 1024;
        let width: u32 = 1920;
        let snowflake_svg_file = "images/flocon_Koch.svg".to_string();
        let iterations = 4;
        let lines = fractals::koch_snowflake(height, width, iterations);
        fractals::create_svg_file_from_lines(&snowflake_svg_file, height, width, lines);

        let x_fractal = 0.3;
        let y_fractal = 0.5;
        let fractal_bmp_file = "images/fractale.bmp".to_string();
        fractals::calculate_fractal_and_write_bmp(x_fractal, y_fractal, &fractal_bmp_file);
    }
}
