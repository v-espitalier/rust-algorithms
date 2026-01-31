use crate::classics;
use crate::misc;
use crate::rational;

#[test]
fn test_classics_factorial() {
    assert_eq!(classics::factorial(0), 1);
    assert_eq!(classics::factorial(1), 1);
    assert_eq!(classics::factorial(4), 24);
    assert_eq!(classics::factorial(5), 120);
}

#[test]
fn test_classics_gcd() {
    assert_eq!(classics::gcd(15, 18), 3);
    assert_eq!(classics::gcd(90, 28), 2);
}

#[test]
fn test_classics_iterative_fibonacci() {
    assert_eq!(classics::fibonacci_iterative(8), 21);
    assert_eq!(classics::fibonacci_iterative(15), 610);
}

#[test]
fn test_classics_recursive_fibonacci() {
    assert_eq!(classics::fibonacci_recursive(8), 21);
    assert_eq!(classics::fibonacci_recursive(15), 610);
}

#[test]
fn test_classics_linear_search() {
    let array: &[i32] = &[5, 10, 3, 7, 15];
    assert_eq!(classics::linear_search(array, 3), Some(2));
    assert_eq!(classics::linear_search(array, 7), Some(3));
    assert_eq!(classics::linear_search(array, 19), None);
}

#[test]
fn test_classics_generic_linear_search() {
    let array: &[i32] = &[5, 10, 3, 7, 15];
    assert_eq!(classics::generic_linear_search(array, 3), Some(2));
    assert_eq!(classics::generic_linear_search(array, 7), Some(3));
    assert_eq!(classics::generic_linear_search(array, 19), None);

    let string_array: &[String] = &[
        "aa".to_string(),
        "bb".to_string(),
        "cc".to_string(),
        "dd".to_string(),
    ];
    assert_eq!(
        classics::generic_linear_search(string_array, "cc".to_string()),
        Some(2)
    );
    assert_eq!(
        classics::generic_linear_search(string_array, "aa".to_string()),
        Some(0)
    );
    assert_eq!(
        classics::generic_linear_search(string_array, "ee".to_string()),
        None
    );
}

#[test]
fn test_classics_binary_search() {
    let array: &[i32] = &[5, 10, 17, 24, 29, 37, 50];
    assert_eq!(classics::binary_search(array, 17, None, None), Some(2));
    assert_eq!(classics::binary_search(array, 50, None, None), Some(6));
    assert_eq!(classics::binary_search(array, 13, None, None), None);
}

#[test]
fn test_asm_gcd() {
    assert_eq!(misc::gcd_asm(15, 18), 3, "Failed test_asm_gcd (1)");
    assert_eq!(misc::gcd_asm(90, 28), 2, "Failed test_asm_gcd (2)");
}

#[test]
fn test_rationals() {
    let r1 = rational::Rational::new(2i64, 3i64);
    let r2 = rational::Rational::new(5i64, 6i64);
    let r1_plus_r2 = rational::Rational::new(3i64, 2i64);
    let r1_minus_r2 = rational::Rational::new(-1i64, 6i64);
    let r1_mult_r2 = rational::Rational::new(5i64, 9i64);
    let r1_div_r2 = rational::Rational::new(4i64, 5i64);
    assert_eq!(
        &r1 + &r2,
        r1_plus_r2,
        "Failed test_rationals (1): Reference addition."
    );
    assert_eq!(
        &r1 - &r2,
        r1_minus_r2,
        "Failed test_rationals (2): Reference subtraction."
    );
    assert_eq!(
        &r1 * &r2,
        r1_mult_r2,
        "Failed test_rationals (3): Reference multiplication."
    );
    assert_eq!(
        &r1 / &r2,
        r1_div_r2,
        "Failed test_rationals (4): Reference division."
    );
}
