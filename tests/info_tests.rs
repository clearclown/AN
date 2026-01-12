//! info コマンド統合テスト

use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::tempdir;
use std::fs;

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
version = "120.0"
"#;

    let telegram_toml = r#"
[app]
name = "telegram"
description = "Telegram Desktop Messenger"
homepage = "https://telegram.org/"

[source]
type = "flatpak"
flatpak_id = "org.telegram.desktop"
architecture = ["x86_64", "aarch64"]

[metadata]
categories = ["Network", "InstantMessaging"]
"#;

    fs::write(dir.path().join("firefox.toml"), firefox_toml).unwrap();
    fs::write(dir.path().join("telegram.toml"), telegram_toml).unwrap();

    dir
}

/// info コマンドでアプリ詳細表示
#[test]
fn test_info_appimage() {
    let db_dir = setup_test_db();

    let mut cmd = Command::cargo_bin("an").unwrap();
    cmd.env("AN_DB_DIR", db_dir.path())
        .args(["info", "firefox"])
        .assert()
        .success()
        .stdout(predicate::str::contains("firefox"))
        .stdout(predicate::str::contains("Mozilla Firefox"))
        .stdout(predicate::str::contains("AppImage"))
        .stdout(predicate::str::contains("https://example.com/firefox.AppImage"))
        .stdout(predicate::str::contains("x86_64"));
}

/// info コマンドでFlatpakアプリ詳細表示
#[test]
fn test_info_flatpak() {
    let db_dir = setup_test_db();

    let mut cmd = Command::cargo_bin("an").unwrap();
    cmd.env("AN_DB_DIR", db_dir.path())
        .args(["info", "telegram"])
        .assert()
        .success()
        .stdout(predicate::str::contains("telegram"))
        .stdout(predicate::str::contains("Flatpak"))
        .stdout(predicate::str::contains("org.telegram.desktop"));
}

/// info コマンドでホームページ表示
#[test]
fn test_info_shows_homepage() {
    let db_dir = setup_test_db();

    let mut cmd = Command::cargo_bin("an").unwrap();
    cmd.env("AN_DB_DIR", db_dir.path())
        .args(["info", "firefox"])
        .assert()
        .success()
        .stdout(predicate::str::contains("ホームページ"))
        .stdout(predicate::str::contains("https://www.mozilla.org/firefox/"));
}

/// info コマンドでカテゴリ表示
#[test]
fn test_info_shows_categories() {
    let db_dir = setup_test_db();

    let mut cmd = Command::cargo_bin("an").unwrap();
    cmd.env("AN_DB_DIR", db_dir.path())
        .args(["info", "firefox"])
        .assert()
        .success()
        .stdout(predicate::str::contains("カテゴリ"))
        .stdout(predicate::str::contains("Network"));
}

/// info コマンドで存在しないアプリ
#[test]
fn test_info_not_found() {
    let db_dir = setup_test_db();

    let mut cmd = Command::cargo_bin("an").unwrap();
    cmd.env("AN_DB_DIR", db_dir.path())
        .args(["info", "nonexistent_app"])
        .assert()
        .success()
        .stderr(predicate::str::contains("見つかりません"));
}

/// info コマンドでアーキテクチャ表示
#[test]
fn test_info_shows_architecture() {
    let db_dir = setup_test_db();

    let mut cmd = Command::cargo_bin("an").unwrap();
    cmd.env("AN_DB_DIR", db_dir.path())
        .args(["info", "telegram"])
        .assert()
        .success()
        .stdout(predicate::str::contains("アーキテクチャ"))
        .stdout(predicate::str::contains("x86_64"))
        .stdout(predicate::str::contains("aarch64"));
}
