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
    ($x:expr, $return:expr) => {
        match $x.to_option() {
            Some(v) => v,
            None => {
                return $return;
            }
        }
    };
}

/// Works like `.unwrap`, if it's an Err(_) or None it calls a fn/closure then returns with the result.
/// Will return No data or Data.
#[macro_export]
macro_rules! unwrap_or_fn_return {
    ($x:expr, $closure:expr) => {
        match $x.to_option() {
            Some(v) => v,
            None => {
                return $closure();
            }
        }
    };
}
