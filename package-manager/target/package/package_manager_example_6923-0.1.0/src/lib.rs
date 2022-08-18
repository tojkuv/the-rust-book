//! # Package Manager
//! 
//! `package_manager` is a collection of utilities to make performing certain calculations more convenient.

pub mod art;

/// Adds one to the number given.
///
/// # Example
///
/// ```
/// let arg = 5;
/// let answer = package_manager::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}