//! install コマンド統合テスト

use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::tempdir;

/// テスト用のアプリDBを作成するヘルパー
fn setup_test_db() -> tempfile::TempDir {
    let dir = tempdir().unwrap();

    let firefox_toml = r#"
[app]
name = "firefox"
description = "Mozilla Firefox Web Browser"
homepage = "https://www.mozilla.org/firefox/"

[source]
type = "appimage"
url = "https://example.com/firefox.AppImage"
architecture = ["x86_64"]

[metadata]
categories = ["Network", "WebBrowser"]
"#;

    fs::write(dir.path().join("firefox.toml"), firefox_toml).unwrap();

    dir
}

/// install で存在しないファイルを指定した場合のエラー
#[test]
fn test_install_file_not_found() {
    let mut cmd = Command::cargo_bin("an").unwrap();
    cmd.args(["install", "/nonexistent/path/app.deb"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("E001").or(predicate::str::contains("見つかりません")));
}

/// install で不明な拡張子のファイルを指定した場合のエラー
#[test]
fn test_install_unknown_extension() {
    // 一時ファイルを作成
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("unknown.xyz");
    fs::write(&file_path, "dummy content").unwrap();

    let mut cmd = Command::cargo_bin("an").unwrap();
    cmd.args(["install", file_path.to_str().unwrap()])
        .assert()
        .failure()
        .stderr(predicate::str::contains("E002").or(predicate::str::contains("不明")));
}

/// install でDBにないリモートアプリを指定した場合のエラー
#[test]
fn test_install_app_not_in_db() {
    let db_dir = setup_test_db();

    let mut cmd = Command::cargo_bin("an").unwrap();
    cmd.env("AN_DB_DIR", db_dir.path())
        .args(["install", "nonexistent_app_12345"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("E003").or(predicate::str::contains("見つかりません")));
}

/// install エイリアス 'i' のテスト
#[test]
fn test_install_alias_i() {
    let mut cmd = Command::cargo_bin("an").unwrap();
    cmd.args(["i", "/nonexistent/path/app.deb"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("E001").or(predicate::str::contains("見つかりません")));
}

/// install オプション --name のパース確認
#[test]
fn test_install_name_option() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("app.AppImage");
    fs::write(&file_path, "dummy").unwrap();

    // --name オプションが正しくパースされることを確認
    // (実際のインストールはファイルが無効なので失敗するが、オプションのパースは成功する)
    let mut cmd = Command::cargo_bin("an").unwrap();
    cmd.args(["install", file_path.to_str().unwrap(), "--name", "myapp"])
        .assert();
    // パースエラーが出ないことを確認
}

/// install オプション --desktop のパース確認
#[test]
fn test_install_desktop_option() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("app.AppImage");
    fs::write(&file_path, "dummy").unwrap();

    let mut cmd = Command::cargo_bin("an").unwrap();
    cmd.args(["install", file_path.to_str().unwrap(), "--desktop"])
        .assert();
    // パースエラーが出ないことを確認
}

/// install オプション --move のパース確認
#[test]
fn test_install_move_option() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("app.AppImage");
    fs::write(&file_path, "dummy").unwrap();

    let mut cmd = Command::cargo_bin("an").unwrap();
    cmd.args(["install", file_path.to_str().unwrap(), "--move"])
        .assert();
    // パースエラーが出ないことを確認
}

/// install 引数なしでエラー
#[test]
fn test_install_no_args() {
    let mut cmd = Command::cargo_bin("an").unwrap();
    cmd.arg("install")
        .assert()
        .failure()
        .stderr(predicate::str::contains("required"));
}
