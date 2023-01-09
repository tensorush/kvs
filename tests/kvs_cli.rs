use assert_cmd::prelude::*;
use predicates::str::contains;
use std::process::Command;

// `kvs_cli` with no args should exit with a non-zero code.
#[test]
fn cli_no_args() {
    Command::cargo_bin("kvs_cli").unwrap().assert().failure();
}

// `kvs_cli -V` should print the version
#[test]
fn cli_version() {
    Command::cargo_bin("kvs_cli")
        .unwrap()
        .args(&["-V"])
        .assert()
        .stdout(contains(env!("CARGO_PKG_VERSION")));
}

// `kvs_cli get <KEY>` should print "unimplemented" to stderr and exit with non-zero code
#[test]
fn cli_get() {
    Command::cargo_bin("kvs_cli")
        .unwrap()
        .args(&["get", "key1"])
        .assert()
        .failure()
        .stderr(contains("unimplemented"));
}

// `kvs_cli set <KEY> <VALUE>` should print "unimplemented" to stderr and exit with non-zero code
#[test]
fn cli_set() {
    Command::cargo_bin("kvs_cli")
        .unwrap()
        .args(&["set", "key1", "value1"])
        .assert()
        .failure()
        .stderr(contains("unimplemented"));
}

// `kvs_cli rm <KEY>` should print "unimplemented" to stderr and exit with non-zero code
#[test]
fn cli_rm() {
    Command::cargo_bin("kvs_cli")
        .unwrap()
        .args(&["rm", "key1"])
        .assert()
        .failure()
        .stderr(contains("unimplemented"));
}

#[test]
fn cli_invalid_get() {
    Command::cargo_bin("kvs_cli")
        .unwrap()
        .args(&["get"])
        .assert()
        .failure();

    Command::cargo_bin("kvs_cli")
        .unwrap()
        .args(&["get", "extra", "field"])
        .assert()
        .failure();
}

#[test]
fn cli_invalid_set() {
    Command::cargo_bin("kvs_cli")
        .unwrap()
        .args(&["set"])
        .assert()
        .failure();

    Command::cargo_bin("kvs_cli")
        .unwrap()
        .args(&["set", "missing_field"])
        .assert()
        .failure();

    Command::cargo_bin("kvs_cli")
        .unwrap()
        .args(&["set", "extra", "extra", "field"])
        .assert()
        .failure();
}

#[test]
fn cli_invalid_rm() {
    Command::cargo_bin("kvs_cli")
        .unwrap()
        .args(&["rm"])
        .assert()
        .failure();

    Command::cargo_bin("kvs_cli")
        .unwrap()
        .args(&["rm", "extra", "field"])
        .assert()
        .failure();
}

#[test]
fn cli_invalid_subcommand() {
    Command::cargo_bin("kvs_cli")
        .unwrap()
        .args(&["unknown", "subcommand"])
        .assert()
        .failure();
}
