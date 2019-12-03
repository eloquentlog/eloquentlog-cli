use std::panic::{self, AssertUnwindSafe};
use std::process::Command;

mod run;

/// A test runner for integration tests
pub fn run_test<T>(test: T)
where
    T: FnOnce(&mut Command) -> () + panic::UnwindSafe,
{
    setup();

    let result = panic::catch_unwind(AssertUnwindSafe(|| {
        let mut cmd = Command::new("./target/debug/eloquentlog");
        test(&mut cmd)
    }));
    assert!(result.is_ok());

    teardown();
}

fn setup() {
    // pass
}

fn teardown() {
    // pass
}
