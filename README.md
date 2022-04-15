# unwrap_helpers
Unwrap Macros to help Clean up code and improve production.
This does include a pub use of https://github.com/Mrp1Dev/loop_unwrap to gain access to unwrap loop macros.

[![https://crates.io/crates/unwrap_helpers](https://img.shields.io/badge/crates.io-v0.3.0-blue)](https://crates.io/crates/unwrap_helpers)
[![Docs](https://docs.rs/unwrap_helpers/badge.svg)](https://docs.rs/unwrap_helpers)

/// Works like `.unwrap`, if it's an Err(_) or None it calls return.
/// Will return no data or data. Can use Closures or functions that returns the parent functions return type.

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

    let _ = unwrap_or_return!(opt, || -> i32 { x + 1 });

    1
}
```
```
fn ret_closure_ret_fail_insert() -> i32 {
    let x = 5;
    let opt = None;

    let _ = unwrap_or_return!(opt, |x1| x1 + 1, x);

    1
}
```
```
fn test_no_return() {
    let input = Option::None;
    let parsed_input = unwrap_or_return!(input);
}
```

# Help

If you need help with this library please go to our [Discord Group](https://discord.gg/xKkm7UhM36)