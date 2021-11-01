pub use unwrap_helpers::*;

#[test]
pub fn runner() {
    assert_eq! {1, ret_test_pass() };
    assert_eq! {0, ret_test_fail() };
    assert_eq! {1, ret_fn_pass() };
    assert_eq! {6, ret_fn_fail() };
    assert_eq! {1, ret_closure_pass() };
    assert_eq! {6, ret_closure_fail() };
    assert_eq! {1, ret_closure_ret_pass() };
    assert_eq! {6, ret_closure_ret_fail() };
    assert_eq! {None, ret_as_option_pass() };
    assert_eq! {Some(0), ret_as_option_fail() };
    ret_nothing();
}

fn ret_test_pass() -> i32 {
    let opt = Some(50);

    let _ = unwrap_or_return!(opt, -1);

    1
}

fn ret_test_fail() -> i32 {
    let opt = None;

    let _ = unwrap_or_return!(opt, 0);

    1
}

fn ret_as_option_pass() -> Option<i32> {
    let opt = Some(50);

    let _ = unwrap_or_return!(opt, Some(0));

    None
}

fn ret_as_option_fail() -> Option<i32> {
    let opt = None;

    let _ = unwrap_or_return!(opt, Some(0));

    None
}

fn test_send(x: i32) -> i32 {
    x + 1
}

fn ret_fn_pass() -> i32 {
    let x = 5;
    let opt = Some(50);

    let _ = unwrap_or_return!(opt, test_send(x));

    1
}

fn ret_fn_fail() -> i32 {
    let x = 5;
    let opt = None;

    let _ = unwrap_or_return!(opt, test_send(x));

    1
}

fn ret_closure_pass() -> i32 {
    let x = 5;
    let opt = Some(50);

    let _ = unwrap_or_return!(opt, || x + 1);

    1
}

fn ret_closure_fail() -> i32 {
    let x = 5;
    let opt = None;

    let _ = unwrap_or_return!(opt, || x + 1);

    1
}

fn ret_closure_ret_pass() -> i32 {
    let x = 5;
    let opt = Some(50);

    let _ = unwrap_or_return!(opt, || -> i32 { x + 1 }());

    1
}

fn ret_closure_ret_fail() -> i32 {
    let x = 5;
    let opt = None;

    let _ = unwrap_or_return!(opt, || -> i32 { x + 1 }());

    1
}

fn ret_nothing() {
    let opt = None;

    let _ = unwrap_or_return!(opt);
}
