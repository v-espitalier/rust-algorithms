//! Hexadecimal, Binary, and Decimal Conversion Utilities in Rust
//!
//! Provides functions for converting between decimal, hexadecimal, binary, and octal representations.
//! Includes examples of parsing and formatting integers in different bases.
//! Author: Vincent Espitalier
//! Date: June 2024

#![warn(dead_code)]

use std::fmt::Write;

/// Demonstrates integer conversions between decimal, hexadecimal, binary, and octal formats.
///
/// # Example
/// ```
/// conversions_entier();
/// ```
pub fn integer_conversions() {
    // Writing decimal/hexadecimal/binary/octal values in Rust code
    let decimal_value: u64 = 37;
    let hex_value: u64 = 0x25;
    let binary_value: u64 = 0b100101;
    let octal_value: u64 = 0o45;
    assert_eq!(decimal_value, hex_value, "Conversion error (1)");
    assert_eq!(decimal_value, binary_value, "Conversion error (2)");
    assert_eq!(decimal_value, octal_value, "Conversion error (3)");

    // Convert decimal to hexadecimal, binary, and octal strings
    let expected_hex_string: String = "0x25".to_string();
    let expected_binary_string: String = "0b100101".to_string();
    let expected_octal_string: String = "0o45".to_string();
    let mut buffer: String = String::new();

    write!(buffer, "{:#x}", decimal_value).expect("Error in write! (1)");
    assert_eq!(buffer, expected_hex_string, "Conversion error (4)");

    buffer = "".to_string();
    write!(buffer, "{:#b}", decimal_value).expect("Error in write! (2)");
    assert_eq!(buffer, expected_binary_string, "Conversion error (5)");

    buffer = "".to_string();
    write!(buffer, "{:#o}", decimal_value).expect("Error in write! (3)");
    assert_eq!(buffer, expected_octal_string, "Conversion error (6)");

    // Convert hexadecimal, binary, and octal strings back to decimal
    let value = u64::from_str_radix(&expected_hex_string[2..], 16)
        .expect("Error in u64::from_str_radix (1)");
    assert_eq!(value, decimal_value, "Conversion error (7)");

    let value = u64::from_str_radix(&expected_binary_string[2..], 2)
        .expect("Error in u64::from_str_radix (2)");
    assert_eq!(value, decimal_value, "Conversion error (8)");

    let value = u64::from_str_radix(&expected_octal_string[2..], 8)
        .expect("Error in u64::from_str_radix (3)");
    assert_eq!(value, decimal_value, "Conversion error (9)");
}
