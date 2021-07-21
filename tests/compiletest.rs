#![warn(rust_2018_idioms, single_use_lifetimes)]

use std::env;

#[rustversion::attr(before(2021-07-20), ignore)] // Note: This date is commit-date and the day before the toolchain date.
#[test]
fn ui() {
    if env::var_os("CI").is_none() {
        env::set_var("TRYBUILD", "overwrite");
    }

    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/*.rs");
    t.pass("tests/run-pass/*.rs");
}
