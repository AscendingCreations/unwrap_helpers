# unwrap_helpers
Unwrap Macros to help Clean up code and improve production.
This does include a pub use of https://github.com/Mrp1Dev/loop_unwrap to gain access to unwrap loop macros.

[![https://crates.io/crates/unwrap_helpers](https://img.shields.io/badge/crates.io-v0.1.0-blue)](https://crates.io/crates/unwrap_helpers)
[![Docs](https://docs.rs/unwrap_helpers/badge.svg)](https://docs.rs/unwrap_helpers)

Works like `.unwrap`, if it's an Err(_) or None it calls return.
Will return No data or Data, Also will run a Closure if one inserted.loop_unwrap
# Examples
```
{
    let input = Option::None;
    let parsed_input = unwrap_or_return!(input);
}
```
```
{
    let input = Option::None;
    let parsed_input = unwrap_or_return!(input, 1);
}
```
```
{
    let input = Option::None;
    let mut x = 5;
    let parsed_input = unwrap_or_return!(input, || x + 1);
}
```