#![doc = include_str!("../README.md")]

/// Imports loop unwrap macros into the library.
pub use loop_unwrap::*;

/// Works like `.unwrap`, if it's an Err(_) or None it calls return.
/// Will return No data or Data.
#[macro_export]
macro_rules! unwrap_or_return {
    ($x:expr) => {
        match $x.to_option() {
            Some(v) => v,
            None => {
                return;
            }
        }
    };
    ($x:expr, || $expression:expr) => {
        match $x.to_option() {
            Some(v) => v,
            None => {
                return || $expression();
            }
        }
    };
    ($x:expr, move || $expression:expr) => {
        match $x.to_option() {
            Some(v) => v,
            None => {
                return move || $expression();
            }
        }
    };
    ($x:expr, $return:expr) => {
        match $x.to_option() {
            Some(v) => v,
            None => {
                return $return;
            }
        }
    };
}
