//! Debパッケージハンドラ
//!
//! .debファイルのインストール・削除を処理します。

use crate::errors::AnError;
use crate::utils::ui;
use anyhow::Result;
use std::path::Path;
use std::process::Command;

/// .debファイルをインストール
pub fn install(path: &Path) -> Result<()> {
    ui::info("Installing Debian package...");

    // dpkg -i でインストール
    let output = Command::new("sudo")
        .args(["dpkg", "-i", path.to_str().unwrap()])
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        ui::warn(&format!("dpkg warning: {}", stderr));

        // 依存関係エラーの場合、apt -f install で解決
        ui::info("Resolving dependencies...");
        let fix_output = Command::new("sudo")
            .args(["apt", "-f", "install", "-y"])
            .output()?;

        if !fix_output.status.success() {
            return Err(AnError::DpkgError {
                message: String::from_utf8_lossy(&fix_output.stderr).to_string(),
            }
            .into());
        }
    }

    ui::success("Debian package installed");
    Ok(())
}

/// Debパッケージを削除（パージ）
pub fn remove(package: &str) -> Result<()> {
    ui::info(&format!("Removing package: {}", package));

    // apt purge で設定ファイルも削除
    let output = Command::new("sudo")
        .args(["apt", "purge", "-y", package])
        .output()?;

    if !output.status.success() {
        return Err(AnError::AptPurgeError {
            message: String::from_utf8_lossy(&output.stderr).to_string(),
        }
        .into());
    }

    // 不要な依存関係を削除
    ui::info("Removing unused dependencies...");
    let _ = Command::new("sudo")
        .args(["apt", "autoremove", "-y"])
        .output()?;

    ui::success("Package removed");
    Ok(())
}

/// Debパッケージを検出
pub fn detect(name: &str) -> Option<String> {
    let output = Command::new("dpkg")
        .args(["-l", name])
        .output()
        .ok()?;

    if output.status.success() {
        // パッケージが見つかった
        Some(name.to_string())
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    // 統合テストで実際のdpkg操作をテストする
    // ユニットテストでは構造のみ確認
}
