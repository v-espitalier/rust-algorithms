//! Rational Numbers Implementation
//!
//! Generic implementation of rational numbers (fractions) with arithmetic operations.
//! Supports addition, subtraction, multiplication, division, and comparison operations.
//!
//! Author: Vincent Espitalier
//! Date: June 2024

use std::cmp::{Ordering, PartialEq, PartialOrd};
use std::convert::TryFrom;
use std::fmt::{Debug, Display, Formatter, Result};
use std::ops::{Add, AddAssign, Div, Mul, Neg, Rem, Sub, SubAssign};

/// Struct representing a rational number (fraction) with a numerator and a denominator.
/// The denominator is always positive, and the fraction is in its irreducible form.
#[derive(Clone)]
pub struct Rational<T> {
    numerator: T,
    denominator: T,
}

impl<T> Rational<T>
where
    T: PartialEq
        + PartialOrd
        + Clone
        + Neg<Output = T>
        + TryFrom<i8>
        + Div<Output = T>
        + Rem<T, Output = T>,
    <T as TryFrom<i8>>::Error: Debug,
{
    /// Creates a new `Rational` number.
    ///
    /// # Arguments
    /// * `numerator` - The numerator of the fraction.
    /// * `denominator` - The denominator of the fraction (must not be zero).
    ///
    /// # Panics
    /// Panics if the denominator is zero.
    pub fn new(numerator: T, denominator: T) -> Self {
        let mut ret_num: T = numerator;
        let mut ret_den: T = denominator;

        // Handle division by zero
        let zero: T = T::try_from(0i8).expect("rational.rs zero(): Problem converting zero.");
        if ret_den == zero {
            panic!("Error: Division by zero");
        }

        // Uniqueness of representation: Signs and irreducibility

        // The numerator carries the sign
        // The denominator is always positive.
        if ret_den < zero {
            ret_den = -ret_den;
            ret_num = -ret_num;
        }

        // Convert to irreducible fraction
        let mut abs_num: T = ret_num.clone();
        if abs_num < zero {
            abs_num = -abs_num;
        }

        if ret_num != zero {
            let gcd: T = generic_gcd(&abs_num, &ret_den);
            let gcd_clone: T = gcd.clone(); // To avoid manipulating references, only one clone
            ret_num = ret_num / gcd;
            ret_den = ret_den / gcd_clone;
        }

        // Return value
        Self {
            numerator: ret_num,
            denominator: ret_den,
        }
    }
}

// Trait Add: c = a + b
impl<T> Add for Rational<T>
where
    T: PartialEq
        + PartialOrd
        + Clone
        + Neg<Output = T>
        + TryFrom<i8>
        + Div<Output = T>
        + Rem<T, Output = T>,
    <T as TryFrom<i8>>::Error: Debug,
    T: Add<Output = T> + Mul<Output = T> + Copy, // For addition
{
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let ret_num = self.numerator * other.denominator + self.denominator * other.numerator;
        let ret_den = self.denominator * other.denominator;
        Rational::<T>::new(ret_num, ret_den)
    }
}

// Trait Add (on refs): c = &a + &b (borrow: a and b remain available)
impl<T> Add for &Rational<T>
where
    T: PartialEq
        + PartialOrd
        + Clone
        + Neg<Output = T>
        + TryFrom<i8>
        + Div<Output = T>
        + Rem<T, Output = T>,
    <T as TryFrom<i8>>::Error: Debug,
    T: Add<Output = T> + Mul<Output = T> + Copy, // For addition
{
    type Output = Rational<T>;

    fn add(self, other: &Rational<T>) -> Rational<T> {
        let ret_num = self.numerator * other.denominator + self.denominator * other.numerator;
        let ret_den = self.denominator * other.denominator;
        Rational::<T>::new(ret_num, ret_den)
    }
}

// Trait AddAssign: Combines addition and assignment: a += b
impl<T> AddAssign for Rational<T>
where
    T: PartialEq
        + PartialOrd
        + Clone
        + Neg<Output = T>
        + TryFrom<i8>
        + Div<Output = T>
        + Rem<T, Output = T>,
    <T as TryFrom<i8>>::Error: Debug,
    T: Add<Output = T> + Mul<Output = T> + Copy, // For addition
{
    fn add_assign(&mut self, other: Rational<T>) {
        let ret_num = self.numerator * other.denominator + self.denominator * other.numerator;
        let ret_den = self.denominator * other.denominator;
        let rational = Rational::<T>::new(ret_num, ret_den);
        self.numerator = rational.numerator;
        self.denominator = rational.denominator;
    }
}

// Trait Sub: c = a - b
impl<T> Sub for Rational<T>
where
    T: PartialEq
        + PartialOrd
        + Clone
        + Neg<Output = T>
        + TryFrom<i8>
        + Div<Output = T>
        + Rem<T, Output = T>,
    <T as TryFrom<i8>>::Error: Debug,
    T: Sub<Output = T> + Mul<Output = T> + Copy, // For subtraction
{
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let ret_num = self.numerator * other.denominator - self.denominator * other.numerator;
        let ret_den = self.denominator * other.denominator;
        Rational::<T>::new(ret_num, ret_den)
    }
}

// Trait Sub (on refs): c = &a - &b (borrow: a and b remain available)
impl<T> Sub for &Rational<T>
where
    T: PartialEq
        + PartialOrd
        + Clone
        + Neg<Output = T>
        + TryFrom<i8>
        + Div<Output = T>
        + Rem<T, Output = T>,
    <T as TryFrom<i8>>::Error: Debug,
    T: Sub<Output = T> + Mul<Output = T> + Copy, // For subtraction
{
    type Output = Rational<T>;

    fn sub(self, other: &Rational<T>) -> Rational<T> {
        let ret_num = self.numerator * other.denominator - self.denominator * other.numerator;
        let ret_den = self.denominator * other.denominator;
        Rational::<T>::new(ret_num, ret_den)
    }
}

// Trait SubAssign: Combines subtraction and assignment: a -= b
impl<T> SubAssign for Rational<T>
where
    T: PartialEq
        + PartialOrd
        + Clone
        + Neg<Output = T>
        + TryFrom<i8>
        + Div<Output = T>
        + Rem<T, Output = T>,
    <T as TryFrom<i8>>::Error: Debug,
    T: Sub<Output = T> + Mul<Output = T> + Copy, // For subtraction
{
    fn sub_assign(&mut self, other: Rational<T>) {
        let ret_num = self.numerator * other.denominator - self.denominator * other.numerator;
        let ret_den = self.denominator * other.denominator;
        let rational = Rational::<T>::new(ret_num, ret_den);
        self.numerator = rational.numerator;
        self.denominator = rational.denominator;
    }
}

// Trait Mul: c = a * b
impl<T> Mul for Rational<T>
where
    T: PartialEq
        + PartialOrd
        + Clone
        + Neg<Output = T>
        + TryFrom<i8>
        + Div<Output = T>
        + Rem<T, Output = T>,
    <T as TryFrom<i8>>::Error: Debug,
    T: Mul<Output = T> + Copy, // For multiplication
{
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let ret_num = self.numerator * other.numerator;
        let ret_den = self.denominator * other.denominator;
        Rational::<T>::new(ret_num, ret_den)
    }
}

// Trait Mul (on refs): c = &a * &b
impl<T> Mul for &Rational<T>
where
    T: PartialEq
        + PartialOrd
        + Clone
        + Neg<Output = T>
        + TryFrom<i8>
        + Div<Output = T>
        + Rem<T, Output = T>,
    <T as TryFrom<i8>>::Error: Debug,
    T: Mul<Output = T> + Copy, // For multiplication
{
    type Output = Rational<T>;

    fn mul(self, other: &Rational<T>) -> Rational<T> {
        let ret_num = self.numerator * other.numerator;
        let ret_den = self.denominator * other.denominator;
        Rational::<T>::new(ret_num, ret_den)
    }
}

// Trait Div: c = a / b
impl<T> Div for Rational<T>
where
    T: PartialEq
        + PartialOrd
        + Clone
        + Neg<Output = T>
        + TryFrom<i8>
        + Div<Output = T>
        + Rem<T, Output = T>,
    <T as TryFrom<i8>>::Error: Debug,
    T: Mul<Output = T> + Copy, // For division
{
    type Output = Self;

    fn div(self, other: Self) -> Self {
        let ret_num = self.numerator * other.denominator;
        let ret_den = self.denominator * other.numerator;
        Rational::<T>::new(ret_num, ret_den)
    }
}

// Trait Div (on refs): c = &a / &b
impl<T> Div for &Rational<T>
where
    T: PartialEq
        + PartialOrd
        + Clone
        + Neg<Output = T>
        + TryFrom<i8>
        + Div<Output = T>
        + Rem<T, Output = T>,
    <T as TryFrom<i8>>::Error: Debug,
    T: Mul<Output = T> + Copy, // For division
{
    type Output = Rational<T>;

    fn div(self, other: &Rational<T>) -> Rational<T> {
        let ret_num = self.numerator * other.denominator;
        let ret_den = self.denominator * other.numerator;
        Rational::<T>::new(ret_num, ret_den)
    }
}

// Trait Neg: c = -a
impl<T> Neg for Rational<T>
where
    T: PartialEq
        + PartialOrd
        + Clone
        + Neg<Output = T>
        + TryFrom<i8>
        + Div<Output = T>
        + Rem<T, Output = T>,
    <T as TryFrom<i8>>::Error: Debug,
    T: Neg<Output = T> + Copy, // For negation
{
    type Output = Self;

    fn neg(self) -> Self {
        let ret_num = -self.numerator;
        let ret_den = self.denominator;
        Rational::<T>::new(ret_num, ret_den)
    }
}

// Trait Neg (on refs): c = -&a
impl<T> Neg for &Rational<T>
where
    T: PartialEq
        + PartialOrd
        + Clone
        + Neg<Output = T>
        + TryFrom<i8>
        + Div<Output = T>
        + Rem<T, Output = T>,
    <T as TryFrom<i8>>::Error: Debug,
    T: Neg<Output = T> + Copy, // For negation
{
    type Output = Rational<T>;

    fn neg(self) -> Rational<T> {
        let ret_num = -self.numerator;
        let ret_den = self.denominator;
        Rational::<T>::new(ret_num, ret_den)
    }
}

// Trait PartialEq (on refs): equality test (&a == &b)
impl<T> PartialEq for Rational<T>
where
    T: Sub<Output = T> + Mul<Output = T> + Copy + PartialEq + TryFrom<i8>,
    <T as TryFrom<i8>>::Error: Debug,
{
    fn eq(&self, other: &Rational<T>) -> bool {
        let num_diff = self.numerator * other.denominator - self.denominator * other.numerator;
        let zero: T = T::try_from(0i8).expect("rational.rs zero(): Problem converting zero.");
        num_diff == zero
    }
}

// Trait PartialOrd (on refs): Implements the 4 comparisons: a > b, a >= b, a < b, a <= b
impl<T> PartialOrd for Rational<T>
where
    T: PartialEq
        + PartialOrd
        + Clone
        + Neg<Output = T>
        + TryFrom<i8>
        + Div<Output = T>
        + Rem<T, Output = T>,
    <T as TryFrom<i8>>::Error: Debug,
    T: Sub<Output = T> + Mul<Output = T> + Copy, // For PartialOrd trait
{
    fn partial_cmp(&self, other: &Rational<T>) -> Option<Ordering> {
        let sub_num: T = (self - other).numerator;
        let zero: T = T::try_from(0i8).expect("rational.rs zero(): Problem converting zero.");
        match sub_num {
            tmp if tmp > zero => Some(Ordering::Greater),
            tmp if tmp < zero => Some(Ordering::Less),
            _ => Some(Ordering::Equal),
        }
    }
}

// Conversion to f64: Fraction -> Floating point (approximate floating division)
impl<T> From<Rational<T>> for f64
where
    f64: From<T>,
{
    fn from(input: Rational<T>) -> f64 {
        let num_f64 = f64::from(input.numerator);
        let den_f64 = f64::from(input.denominator);
        if den_f64 == 0. {
            panic!("Error in from: Division by zero.");
        }

        num_f64 / den_f64
    }
}

// Conversion from i64: Integer -> Fraction (numerator = integer, denominator = 1)
impl<T> From<i64> for Rational<T>
where
    T: PartialEq
        + PartialOrd
        + Clone
        + Neg<Output = T>
        + TryFrom<i8>
        + Div<Output = T>
        + Rem<T, Output = T>,
    <T as TryFrom<i8>>::Error: Debug,
    T: From<i64>, // For numerator conversion
{
    fn from(input: i64) -> Rational<T> {
        let ret_num: T = T::from(input);
        let one: T = T::try_from(1i8).expect("rational.rs: Problem converting 'one'.");
        let ret_den = one;
        Rational::<T>::new(ret_num, ret_den)
    }
}

// Traits for display
impl<T> Display for Rational<T>
where
    T: Display + Clone,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}/{}", self.numerator, self.denominator)
    }
}

// Debug trait implementation
impl<T> Debug for Rational<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}/{}", self.numerator, self.denominator)
    }
}

/// Generic GCD (Greatest Common Divisor) function for type T.
///
/// # Arguments
/// * `a` - First number.
/// * `b` - Second number.
///
/// # Panics
/// Panics if b is zero.
pub fn generic_gcd<'a, T>(a: &'a T, b: &'a T) -> T
where
    T: PartialOrd + TryFrom<i8> + Clone,
    <T as TryFrom<i8>>::Error: Debug,
    T: Rem<T, Output = T>,
{
    // Swap a and b if a < b
    if a < b {
        return generic_gcd(b, a);
    }

    let zero: T = T::try_from(0i8).expect("rational.rs zero(): Problem converting zero.");
    if b == &zero {
        panic!("Error: Division by zero in generic_gcd.");
    }
    let m: T = a.clone() % b.clone();
    // Handle the special case (end of recursive calls)
    if m == zero {
        return b.clone();
    }

    // Recursive call
    generic_gcd(b, &m)

    // The number of recursions is finite because a and b are positive integers
    // and strictly decrease with each call
}
