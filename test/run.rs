use std::process::Command;

use crate::run_test;

#[test]
fn test_run_with_unknown_option() {
    run_test(|cmd: &mut Command| {
        let out = cmd
            .arg("--config-file=./test/eloquentlog.toml")
            .arg("--unknown")
            .output()
            .unwrap();
        assert!(String::from_utf8_lossy(&out.stderr).contains("error:"));
        assert_eq!(String::from_utf8_lossy(&out.stdout), "");
    });
}

#[test]
#[ignore]
fn test_run_in_debug_mode() {
    let msg = "debug mode: on";

    run_test(|cmd: &mut Command| {
        let out = cmd
            .arg("--config-file=./test/eloquentlog.toml")
            .output()
            .unwrap();
        assert_eq!(String::from_utf8_lossy(&out.stderr), "");
        assert!(!String::from_utf8_lossy(&out.stdout).contains(msg));
    });

    run_test(|cmd: &mut Command| {
        let out = cmd
            .arg("--config-file=./test/eloquentlog.toml")
            .arg("-d")
            .output()
            .unwrap();
        assert_eq!(String::from_utf8_lossy(&out.stderr), "");
        assert!(String::from_utf8_lossy(&out.stdout).contains(msg));
    });

    run_test(|cmd: &mut Command| {
        let out = cmd
            .arg("--config-file=./test/eloquentlog.toml")
            .arg("--debug")
            .output()
            .unwrap();
        assert_eq!(String::from_utf8_lossy(&out.stderr), "");
        assert!(String::from_utf8_lossy(&out.stdout).contains(msg));
    });
}

#[test]
fn test_run_with_help() {
    run_test(|cmd: &mut Command| {
        let out = cmd
            .arg("--config-file=./test/eloquentlog.toml")
            .arg("--help")
            .output()
            .unwrap();

        assert_eq!(String::from_utf8_lossy(&out.stderr), "");
        assert!(String::from_utf8_lossy(&out.stdout).contains("USAGE"));
    });
}
