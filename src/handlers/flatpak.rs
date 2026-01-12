//! Flatpakハンドラ
//!
//! Flatpakアプリのスキャン・エイリアス生成・削除を処理します。

use crate::errors::AnError;
use crate::utils::ui;
use anyhow::Result;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use std::process::Command;

/// Flatpakアプリ情報
#[derive(Debug, Clone)]
pub struct FlatpakApp {
    pub id: String,
    #[allow(dead_code)]
    pub name: String,
}

/// シンボリックリンク配置先を取得
fn bin_dir() -> PathBuf {
    dirs::home_dir()
        .expect("ホームディレクトリが見つかりません")
        .join(".local")
        .join("bin")
}

/// Flatpakがインストールされているか確認
pub fn is_installed() -> bool {
    Command::new("flatpak")
        .arg("--version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

/// インストール済みFlatpakアプリをスキャン
pub fn scan_apps() -> Result<Vec<FlatpakApp>> {
    let output = Command::new("flatpak")
        .args(["list", "--app", "--columns=application,name"])
        .output()?;

    if !output.status.success() {
        return Err(AnError::FlatpakNotInstalled.into());
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let apps: Vec<FlatpakApp> = stdout
        .lines()
        .filter_map(|line| {
            let parts: Vec<&str> = line.split('\t').collect();
            if parts.len() >= 2 {
                Some(FlatpakApp {
                    id: parts[0].to_string(),
                    name: parts[1].to_string(),
                })
            } else if !line.is_empty() {
                Some(FlatpakApp {
                    id: line.to_string(),
                    name: line.to_string(),
                })
            } else {
                None
            }
        })
        .collect();

    Ok(apps)
}

/// コマンド名の衝突をチェック
pub fn check_name_conflict(name: &str) -> bool {
    // システムバイナリとの衝突確認
    which::which(name).is_ok()
}

/// ラッパースクリプトを生成
pub fn create_wrapper(app_id: &str, name: &str) -> Result<()> {
    let bin_directory = bin_dir();
    if !bin_directory.exists() {
        fs::create_dir_all(&bin_directory)?;
    }

    let wrapper_path = bin_directory.join(name);

    let script = format!(
        r#"#!/bin/bash
# AN-generated wrapper for {}
exec flatpak run {} "$@"
"#,
        app_id, app_id
    );

    fs::write(&wrapper_path, script)?;

    // 実行権限付与
    let mut perms = fs::metadata(&wrapper_path)?.permissions();
    perms.set_mode(0o755);
    fs::set_permissions(&wrapper_path, perms)?;

    Ok(())
}

/// Flatpakアプリを削除
pub fn remove(app_id: &str) -> Result<()> {
    ui::info(&format!("Removing Flatpak app: {}", app_id));

    let output = Command::new("flatpak")
        .args(["uninstall", "--delete-data", "-y", app_id])
        .output()?;

    if !output.status.success() {
        return Err(AnError::FlatpakUninstallError {
            message: String::from_utf8_lossy(&output.stderr).to_string(),
        }
        .into());
    }

    // 関連するラッパースクリプトも削除（存在すれば）
    // 名前を推測して削除を試みる
    let name_guess = app_id
        .split('.')
        .next_back()
        .unwrap_or(app_id)
        .to_lowercase();
    let wrapper_path = bin_dir().join(&name_guess);
    if wrapper_path.exists() {
        // ANが生成したラッパーか確認
        if let Ok(content) = fs::read_to_string(&wrapper_path) {
            if content.contains("AN-generated wrapper") && content.contains(app_id) {
                ui::info(&format!("Removing wrapper: {:?}", wrapper_path));
                let _ = fs::remove_file(wrapper_path);
            }
        }
    }

    ui::success("Flatpak app removed");
    Ok(())
}

/// Flatpakアプリを検出
pub fn detect(name: &str) -> Option<String> {
    let output = Command::new("flatpak")
        .args(["list", "--app", "--columns=application"])
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    let stdout = String::from_utf8_lossy(&output.stdout);

    // 完全一致またはIDの末尾一致で検索
    for line in stdout.lines() {
        let app_id = line.trim();
        if app_id.eq_ignore_ascii_case(name) {
            return Some(app_id.to_string());
        }
        // IDの末尾（アプリ名部分）で検索
        if let Some(last_part) = app_id.split('.').next_back() {
            if last_part.eq_ignore_ascii_case(name) {
                return Some(app_id.to_string());
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bin_dir() {
        let dir = bin_dir();
        assert!(dir.ends_with(".local/bin"));
    }

    // is_installed, scan_apps, remove などは統合テストで実行
}
