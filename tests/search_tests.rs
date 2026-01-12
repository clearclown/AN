//! search コマンド統合テスト

use assert_cmd::Command;
use predicates::prelude::*;
use std::env;
use std::fs;
use tempfile::tempdir;

/// テスト用のアプリDBを作成するヘルパー
fn setup_test_db() -> tempfile::TempDir {
    let dir = tempdir().unwrap();

    // テスト用のTOMLファイルを作成
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

    let vscode_toml = r#"
[app]
name = "code"
description = "Visual Studio Code - Code Editing"
homepage = "https://code.visualstudio.com/"

[source]
type = "deb"
url = "https://example.com/code.deb"
architecture = ["x86_64"]

[metadata]
categories = ["Development", "IDE"]
"#;

    let telegram_toml = r#"
[app]
name = "telegram"
description = "Telegram Desktop Messenger"
homepage = "https://telegram.org/"

[source]
type = "flatpak"
flatpak_id = "org.telegram.desktop"
architecture = ["x86_64"]

[metadata]
categories = ["Network", "InstantMessaging"]
"#;

    fs::write(dir.path().join("firefox.toml"), firefox_toml).unwrap();
    fs::write(dir.path().join("code.toml"), vscode_toml).unwrap();
    fs::write(dir.path().join("telegram.toml"), telegram_toml).unwrap();

    dir
}

/// search (クエリなし) で全アプリ表示
#[test]
fn test_search_list_all() {
    let db_dir = setup_test_db();

    let mut cmd = Command::cargo_bin("an").unwrap();
    cmd.env("AN_DB_DIR", db_dir.path())
        .arg("search")
        .assert()
        .success()
        .stdout(predicate::str::contains("利用可能なアプリ"))
        .stdout(predicate::str::contains("firefox"))
        .stdout(predicate::str::contains("code"))
        .stdout(predicate::str::contains("telegram"));
}

/// search でキーワード検索
#[test]
fn test_search_with_query() {
    let db_dir = setup_test_db();

    let mut cmd = Command::cargo_bin("an").unwrap();
    cmd.env("AN_DB_DIR", db_dir.path())
        .args(["search", "firefox"])
        .assert()
        .success()
        .stdout(predicate::str::contains("firefox"))
        .stdout(predicate::str::contains("Mozilla Firefox"));
}

/// search で存在しないアプリ
#[test]
fn test_search_not_found() {
    let db_dir = setup_test_db();

    let mut cmd = Command::cargo_bin("an").unwrap();
    cmd.env("AN_DB_DIR", db_dir.path())
        .args(["search", "nonexistent_app_12345"])
        .assert()
        .success()
        .stderr(predicate::str::contains("見つかりません"));
}

/// search の大文字小文字無視
#[test]
fn test_search_case_insensitive() {
    let db_dir = setup_test_db();

    let mut cmd = Command::cargo_bin("an").unwrap();
    cmd.env("AN_DB_DIR", db_dir.path())
        .args(["search", "FIREFOX"])
        .assert()
        .success()
        .stdout(predicate::str::contains("firefox"));
}

/// search 's' エイリアス
#[test]
fn test_search_alias() {
    let db_dir = setup_test_db();

    let mut cmd = Command::cargo_bin("an").unwrap();
    cmd.env("AN_DB_DIR", db_dir.path())
        .args(["s", "code"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Visual Studio Code"));
}

/// タイプ別のグループ表示
#[test]
fn test_search_grouped_by_type() {
    let db_dir = setup_test_db();

    let mut cmd = Command::cargo_bin("an").unwrap();
    cmd.env("AN_DB_DIR", db_dir.path())
        .arg("search")
        .assert()
        .success()
        .stdout(predicate::str::contains("AppImage"))
        .stdout(predicate::str::contains("Deb"))
        .stdout(predicate::str::contains("Flatpak"));
}
