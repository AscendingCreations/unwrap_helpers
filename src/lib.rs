#![doc = include_str!("../README.md")]

/// Imports loop unwrap macros into the library.
pub use loop_unwrap::*;

/// Works like `.unwrap`, if it's an Err(_) or None it calls return.
/// Will return No data or Data. Can use a Simple Closure
/// but if you want a closure with return you must make it like || -> i32 {1}()
// TODO Cover more closure types.
#[macro_export]
macro_rules! unwrap_or_return {
    ($x:expr, $fn:expr, ($($arg:expr),*)) => {
        match $x {
            Some(v) => v,
            None => {
                return $fn($($arg),*);
            }
        }
    };
    ($x:expr, move || $expression:expr) => {
        match $x {
            Some(v) => v,
            None => {
                return (move || $expression)();
            }
        }
    };
    ($x:expr, || $expression:expr) => {
        match $x {
            Some(v) => v,
            None => {
                return (|| $expression)();
            }
        }
    };
    ($x:expr, $ret:expr) => {
        match $x {
            Some(v) => v,
            None => {
                return $ret;
            }
        }
    };
    ($x:expr) => {
        match $x.to_option() {
            Some(v) => v,
            None => {
                return;
            }
        }
    };
}
