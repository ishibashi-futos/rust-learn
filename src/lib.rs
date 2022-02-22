//! # My Crate
//!
//! `my_crate` is a collection of utilities to make performing certain
//! calculations more convenient.

pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

pub mod closures;
pub mod logger;
pub mod tests;

/// Adds one to the number given.
/// # Example
///
/// ```
/// let five = 5;
/// let actual = rust_learn::add_one(5);
/// assert_eq!(6, actual);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}

pub mod art;
