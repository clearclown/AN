//! CLI統合テスト
//!
//! ANのCLIコマンドをエンドツーエンドでテストします。

use assert_cmd::Command;
use predicates::prelude::*;

/// ヘルプ表示のテスト
#[test]
fn test_help() {
    let mut cmd = Command::cargo_bin("an").unwrap();
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("AN"))
        .stdout(predicate::str::contains("install"))
        .stdout(predicate::str::contains("remove"))
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("search"));
}

/// バージョン表示のテスト
#[test]
fn test_version() {
    let mut cmd = Command::cargo_bin("an").unwrap();
    cmd.arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("an"));
}

/// 引数なしでヘルプが表示されることを確認
#[test]
fn test_no_args_shows_help() {
    let mut cmd = Command::cargo_bin("an").unwrap();
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Usage"));
}

/// 不正なコマンドでエラー
#[test]
fn test_invalid_command() {
    let mut cmd = Command::cargo_bin("an").unwrap();
    cmd.arg("invalid_command")
        .assert()
        .failure();
}

/// install --help
#[test]
fn test_install_help() {
    let mut cmd = Command::cargo_bin("an").unwrap();
    cmd.args(["install", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("TARGET"))
        .stdout(predicate::str::contains("--name"))
        .stdout(predicate::str::contains("--desktop"))
        .stdout(predicate::str::contains("--move"));
}

/// remove --help
#[test]
fn test_remove_help() {
    let mut cmd = Command::cargo_bin("an").unwrap();
    cmd.args(["remove", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("TARGET"));
}

/// list --help
#[test]
fn test_list_help() {
    let mut cmd = Command::cargo_bin("an").unwrap();
    cmd.args(["list", "--help"])
        .assert()
        .success();
}

/// search --help
#[test]
fn test_search_help() {
    let mut cmd = Command::cargo_bin("an").unwrap();
    cmd.args(["search", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("QUERY"));
}

/// info --help
#[test]
fn test_info_help() {
    let mut cmd = Command::cargo_bin("an").unwrap();
    cmd.args(["info", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("NAME"));
}

/// sync --help
#[test]
fn test_sync_help() {
    let mut cmd = Command::cargo_bin("an").unwrap();
    cmd.args(["sync", "--help"])
        .assert()
        .success();
}

/// link --help
#[test]
fn test_link_help() {
    let mut cmd = Command::cargo_bin("an").unwrap();
    cmd.args(["link", "--help"])
        .assert()
        .success();
}

/// update --help
#[test]
fn test_update_help() {
    let mut cmd = Command::cargo_bin("an").unwrap();
    cmd.args(["update", "--help"])
        .assert()
        .success();
}

/// エイリアス 'i' のテスト
#[test]
fn test_install_alias() {
    let mut cmd = Command::cargo_bin("an").unwrap();
    cmd.args(["i", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("TARGET"));
}

/// エイリアス 'rm' のテスト
#[test]
fn test_remove_alias_rm() {
    let mut cmd = Command::cargo_bin("an").unwrap();
    cmd.args(["rm", "--help"])
        .assert()
        .success();
}

/// エイリアス 'uninstall' のテスト
#[test]
fn test_remove_alias_uninstall() {
    let mut cmd = Command::cargo_bin("an").unwrap();
    cmd.args(["uninstall", "--help"])
        .assert()
        .success();
}

/// エイリアス 'ls' のテスト
#[test]
fn test_list_alias() {
    let mut cmd = Command::cargo_bin("an").unwrap();
    cmd.args(["ls", "--help"])
        .assert()
        .success();
}

/// エイリアス 's' のテスト
#[test]
fn test_search_alias() {
    let mut cmd = Command::cargo_bin("an").unwrap();
    cmd.args(["s", "--help"])
        .assert()
        .success();
}

/// エイリアス 'l' のテスト
#[test]
fn test_link_alias() {
    let mut cmd = Command::cargo_bin("an").unwrap();
    cmd.args(["l", "--help"])
        .assert()
        .success();
}
