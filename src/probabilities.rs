//! Probability and Randomness Algorithms
//!
//! Implementation of various probabilistic algorithms and random number generators.
//! Includes:
//! - MINSTD linear congruential generator (Park-Miller, 1988)
//! - Fisher-Yates shuffle algorithm
//! - Box-Muller transform for normal distribution
//! - Generic statistical functions (mean, variance)
//!
//! WARNING: These generators are predictable and should NOT be used for cryptography or gambling.
//!
//! Author: Vincent Espitalier
//! Date: June 2024

/// Linear Congruential Generator (MINSTD/Park-Miller implementation)
///
/// WARNING: This generator is highly predictable.
/// DO NOT USE FOR CRYPTOGRAPHY OR GAMBLING.
///
/// # Reference
/// [Linear congruential generator - Wikipedia](https://en.wikipedia.org/wiki/Lehmer_random_number_generator)
pub struct MinstdRng {
    multiplier: u64,
    modulus: u64,
    state: u32,
}

impl MinstdRng {
    /// Creates a new RNG instance with a given seed.
    ///
    /// # Arguments
    /// * `seed` - Initial seed value (must be non-zero)
    ///
    /// # Panics
    /// Panics if seed is zero.
    ///
    /// # Example
    /// ```
    /// let mut rng = MinstdRng::new(42);
    /// ```
    pub fn new(seed: u32) -> MinstdRng {
        assert_ne!(seed, 0, "Seed must be non-zero.");
        // MINSTD constants (Park-Miller RNG)
        let multiplier: u64 = 16807;
        let modulus: u64 = 0x7FFFFFFF; // 2^31 - 1
        MinstdRng {
            multiplier,
            modulus,
            state: seed,
        }
    }

    /// Generates a new random number in the range [0, modulus-1].
    ///
    /// # Returns
    /// A pseudo-random number in the specified range.
    ///
    /// # Example
    /// ```
    /// let mut rng = MinstdRng::new(42);
    /// let random_num = rng.gen();
    /// ```
    pub fn gen(&mut self) -> u32 {
        let new_state: u32 = (((self.state as u64) * self.multiplier) % self.modulus) as u32;
        self.state = new_state;
        new_state
    }

    /// Generates a random number in the specified range.
    ///
    /// # Arguments
    /// * `range` - The range of values to generate (start..end)
    ///
    /// # Returns
    /// A pseudo-random number in the specified range.
    ///
    /// # Panics
    /// Panics if range size is less than 1.
    ///
    /// # Example
    /// ```
    /// let mut rng = MinstdRng::new(42);
    /// let random_num = rng.gen_range(1..10);
    /// ```
    pub fn gen_range(&mut self, range: std::ops::Range<usize>) -> u32 {
        let range_start: u32 = range.start as u32;
        let range_end: u32 = range.end as u32;
        let range_size: u32 = range_end - range_start;
        assert!(
            range_size >= 1,
            "Range size must be greater than or equal to 1."
        );

        // Unbiased transformation of RNG output
        let max_accepted_without_reject: u32 = range_size * ((self.modulus as u32) / range_size);
        let mut rng_val: u32 = self.gen();
        while rng_val > max_accepted_without_reject {
            rng_val = self.gen();
        }

        range_start + (rng_val % range_size)
    }
}

/// Performs Fisher-Yates shuffle on an array of integers.
///
/// Implements an unbiased random permutation of array elements.
///
/// # Arguments
/// * `array` - The array to shuffle
/// * `seed` - Seed value for the random number generator
///
/// # Example
/// ```
/// let mut arr = [1, 2, 3, 4, 5];
/// fisher_yates_shuffle(&mut arr, 42);
/// ```
///
/// # Reference
/// [Fisher-Yates shuffle - Wikipedia](https://en.wikipedia.org/wiki/Fisher%E2%80%93Yates_shuffle)
pub fn fisher_yates_shuffle(array: &mut [i32], seed: u32) {
    // Use local MINSTD RNG implementation to avoid external dependencies
    let mut rng: MinstdRng = MinstdRng::new(seed);

    let n: usize = array.len();
    for i in (0..n).rev() {
        // Generate random index between 0 and i (inclusive)
        let j: usize = rng.gen_range(0..(i + 1)) as usize;

        // Swap elements at positions i and j
        array.swap(i, j);
    }
}

/// Generates a pair of standard normal distributed random numbers using Box-Muller transform.
///
/// WARNING: This generator is highly predictable.
/// DO NOT USE FOR CRYPTOGRAPHY OR GAMBLING.
///
/// # Arguments
/// * `rng` - Mutable reference to a MinstdRng instance
///
/// # Returns
/// A tuple containing two independent standard normal random numbers.
///
/// # Example
/// ```
/// let mut rng = MinstdRng::new(42);
/// let (z0, z1) = box_muller_pair(&mut rng);
/// ```
///
/// # Reference
/// [Box-Muller transform - Wikipedia](https://fr.wikipedia.org/wiki/M%C3%A9thode_de_Box-Muller)
pub fn box_muller_pair(rng: &mut MinstdRng) -> (f64, f64) {
    let u1_int: u32 = rng.gen();
    let u2_int: u32 = rng.gen();

    // Avoid zero to prevent log(0) error
    let u1: f64 = if u1_int != 0 {
        (u1_int as f64) / (rng.modulus as f64)
    } else {
        f64::EPSILON
    };
    let u2: f64 = (u2_int as f64) / (rng.modulus as f64);

    let radius: f64 = f64::sqrt(-2. * u1.ln());
    let angle: f64 = 2. * std::f64::consts::PI * u2;
    let z0 = radius * f64::cos(angle);
    let z1 = radius * f64::sin(angle);

    (z0, z1)
}

/// Generates a vector of normally distributed random numbers using Box-Muller transform.
///
/// # Arguments
/// * `count` - Number of normal random numbers to generate
/// * `seed` - Seed value for the random number generator
///
/// # Returns
/// A vector containing the requested number of normally distributed values.
///
/// # Example
/// ```
/// let normals = box_muller(100, 42);
/// ```
pub fn box_muller(count: usize, seed: u32) -> Vec<f64> {
    // Use local MINSTD RNG implementation to avoid external dependencies
    let mut rng: MinstdRng = MinstdRng::new(seed);

    let complete_pairs = count / 2;
    let incomplete_pairs = count - 2 * complete_pairs;

    let mut normals: Vec<f64> = Vec::new();
    for _ in 0..complete_pairs {
        let (z0, z1) = box_muller_pair(&mut rng);
        normals.push(z0);
        normals.push(z1);
    }

    for _ in 0..incomplete_pairs {
        let (z0, _) = box_muller_pair(&mut rng);
        normals.push(z0);
    }

    normals
}

/// Calculates the mean of an array of values.
///
/// Generic implementation for any numeric type that supports
/// cloning, addition, division, and conversion from u32.
///
/// # Arguments
/// * `array` - The array of values
///
/// # Returns
/// An Option containing the mean value, or None if the array is empty.
///
/// # Example
/// ```
/// let arr = [1.0, 2.0, 3.0, 4.0];
/// let mean = mean(&arr);
/// ```
pub fn mean<T>(array: &[T]) -> Option<T>
where
    T: Clone + From<u32> + From<<T as std::ops::Div>::Output> + std::ops::AddAssign + std::ops::Div,
{
    let n = array.len();
    if n == 0 {
        return None;
    }

    let mut sum: T = array[0].clone();

    for elem in array.iter().skip(1) {
        sum += elem.clone();
    }

    let n_as_t = T::from(n as u32);
    Some(T::from(sum / n_as_t))
}

/// Calculates the unbiased variance of an array of values.
///
/// Generic implementation for any numeric type that supports
/// cloning, addition, multiplication, and division.
///
/// # Arguments
/// * `array` - The array of values
/// * `correction` - Optional correction factor (default = 1 for unbiased variance)
///
/// # Returns
/// An Option containing the variance value, or None if the array is empty.
///
/// # Example
/// ```
/// let arr = [1.0, 2.0, 3.0, 4.0];
/// let variance = variance(&arr, Some(1));
/// ```
///
/// # Reference
/// [Variance - PyTorch Documentation](https://pytorch.org/docs/stable/generated/torch.var.html)
pub fn variance<T>(array: &[T], correction: Option<usize>) -> Option<T>
where
    T: Clone + From<u32> + From<<T as std::ops::Mul>::Output> + From<<T as std::ops::Div>::Output>,
    T: std::ops::AddAssign + std::ops::Mul + std::ops::Div,
{
    let n = array.len();
    if n == 0 {
        return None;
    }
    let delta_n: usize = correction.unwrap_or(1);

    // Calculate sum of squares
    let mut sum_of_squares: T = T::from(array[0].clone() * array[0].clone());

    for elem in array.iter().skip(1) {
        sum_of_squares += T::from(elem.clone() * elem.clone());
    }

    let n_minus_delta_n_as_t = T::from((n - delta_n) as u32);
    Some(T::from(sum_of_squares / n_minus_delta_n_as_t))
}
