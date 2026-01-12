//! list コマンド統合テスト

use assert_cmd::Command;
use predicates::prelude::*;

/// listコマンドが正常に実行される
#[test]
fn test_list_runs_successfully() {
    let mut cmd = Command::cargo_bin("an").unwrap();
    cmd.arg("list")
        .assert()
        .success()
        .stdout(predicate::str::contains("インストール済みアプリ一覧"))
        .stdout(predicate::str::contains("AppImage"))
        .stdout(predicate::str::contains("Flatpak"))
        .stdout(predicate::str::contains("Deb"));
}

/// lsエイリアスでも同様の出力
#[test]
fn test_ls_alias_runs_successfully() {
    let mut cmd = Command::cargo_bin("an").unwrap();
    cmd.arg("ls")
        .assert()
        .success()
        .stdout(predicate::str::contains("インストール済みアプリ一覧"));
}

/// 合計数が表示される
#[test]
fn test_list_shows_total() {
    let mut cmd = Command::cargo_bin("an").unwrap();
    cmd.arg("list")
        .assert()
        .success()
        .stdout(predicate::str::contains("合計"));
}
