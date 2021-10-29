# unwrap_helpers
Unwrap Macros to help Clean up code and improve production.
This does include a pub use of https://github.com/Mrp1Dev/loop_unwrap to gain access to unwrap loop macros.

[![https://crates.io/crates/unwrap_helpers](https://img.shields.io/badge/crates.io-v0.2.2-blue)](https://crates.io/crates/unwrap_helpers)
[![Docs](https://docs.rs/unwrap_helpers/badge.svg)](https://docs.rs/unwrap_helpers)

/// Works like `.unwrap`, if it's an Err(_) or None it calls return.
/// Will return No data or Data. Can use a Simple Closure
/// but if you want a closure with return type set you must make it like || -> i32 {1}()
# Examples
```
fn ret_test_fail() -> i32 {
    let opt = None;

    let _ = unwrap_or_return!(opt, 0);

    1
}
```
```
fn ret_as_option_fail() -> Option<i32> {
    let opt = None;

    let _ = unwrap_or_return!(opt, Some(0));

    None
}
```
```
fn test_send(x: i32) -> i32 {
    x + 1
}

fn ret_fn_fail() -> i32 {
    let x = 5;
    let opt = None;

    let _ = unwrap_or_return!(opt, test_send(x));

    1
}
```
```
fn ret_closure_fail() -> i32 {
    let x = 5;
    let opt = None;

    let _ = unwrap_or_return!(opt, || x + 1);

    1
}
```
```
fn ret_closure_ret_fail() -> i32 {
    let x = 5;
    let opt = None;

    let _ = unwrap_or_return!(opt, || -> i32 { x + 1 }());

    1
}
```
```
fn test_no_return() {
    let input = Option::None;
    let parsed_input = unwrap_or_return!(input);
}
```