use std::process::Command;

use crate::run_test;

#[test]
fn test_run_with_unknown_option() {
    run_test(|cmd: &mut Command| {
        let out = cmd.arg("--unknown").output().unwrap();

        assert!(String::from_utf8_lossy(&out.stderr).contains("error:"));
        assert_eq!(String::from_utf8_lossy(&out.stdout), "");
    })
}

#[test]
fn test_run_with_help() {
    run_test(|cmd: &mut Command| {
        let out = cmd.arg("--help").output().unwrap();

        assert_eq!(String::from_utf8_lossy(&out.stderr), "");
        assert!(String::from_utf8_lossy(&out.stdout).contains("USAGE"));
    })
}
