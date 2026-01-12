//! update コマンド実装
//!
//! AN本体のアップデートとアプリDBの更新を行います。

use crate::commands::sync;
use crate::errors::AnError;
use crate::utils::ui;
use anyhow::Result;
use semver::Version;
use std::fs;
use std::io::Write;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;

/// 現在のバージョン
const CURRENT_VERSION: &str = env!("CARGO_PKG_VERSION");

/// GitHubリポジトリ
const REPO: &str = "clearclown/AN";

/// GitHub Releases APIのレスポンス
#[derive(Debug, serde::Deserialize)]
struct GitHubRelease {
    tag_name: String,
    assets: Vec<GitHubAsset>,
}

#[derive(Debug, serde::Deserialize)]
struct GitHubAsset {
    name: String,
    browser_download_url: String,
}

/// システムアーキテクチャを取得
fn get_arch() -> &'static str {
    #[cfg(target_arch = "x86_64")]
    {
        "x86_64"
    }
    #[cfg(target_arch = "aarch64")]
    {
        "aarch64"
    }
    #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
    {
        "unknown"
    }
}

/// 最新バージョンを取得
fn check_latest_version() -> Result<GitHubRelease> {
    let url = format!("https://api.github.com/repos/{}/releases/latest", REPO);

    let client = reqwest::blocking::Client::builder()
        .user_agent("AN-Package-Manager")
        .build()?;

    let response: GitHubRelease = client.get(&url).send()?.json()?;

    Ok(response)
}

/// バージョン文字列からsemverを抽出（v0.1.0 → 0.1.0）
fn parse_version(version: &str) -> String {
    version.trim_start_matches('v').to_string()
}

/// バージョン比較
pub fn should_update(current: &str, latest: &str) -> bool {
    let current = Version::parse(current).unwrap_or_else(|_| Version::new(0, 0, 0));
    let latest_cleaned = parse_version(latest);
    let latest = Version::parse(&latest_cleaned).unwrap_or_else(|_| Version::new(0, 0, 0));
    latest > current
}

/// バイナリをダウンロードしてインストール
fn download_and_install(release: &GitHubRelease) -> Result<()> {
    let arch = get_arch();
    let expected_name = format!("an-linux-{}", arch);

    // 対応するアセットを検索
    let asset = release
        .assets
        .iter()
        .find(|a| a.name == expected_name)
        .ok_or_else(|| AnError::DownloadFailed {
            message: format!("アーキテクチャ {} に対応するバイナリが見つかりません", arch),
        })?;

    ui::info(&format!("ダウンロード中: {}", asset.name));

    // 一時ファイルにダウンロード
    let temp_path = std::env::temp_dir().join("an-update");

    let client = reqwest::blocking::Client::builder()
        .user_agent("AN-Package-Manager")
        .build()?;

    let response = client.get(&asset.browser_download_url).send()?;

    if !response.status().is_success() {
        return Err(AnError::DownloadFailed {
            message: format!("HTTPエラー: {}", response.status()),
        }
        .into());
    }

    let bytes = response.bytes()?;

    let mut file = fs::File::create(&temp_path)?;
    file.write_all(&bytes)?;

    // 実行権限を付与
    let mut perms = fs::metadata(&temp_path)?.permissions();
    perms.set_mode(0o755);
    fs::set_permissions(&temp_path, perms)?;

    // 現在のバイナリの場所を取得
    let current_exe = std::env::current_exe()?;

    // バックアップを作成
    let backup_path = current_exe.with_extension("bak");
    if current_exe.exists() {
        fs::copy(&current_exe, &backup_path)?;
        ui::info(&format!("バックアップ作成: {:?}", backup_path));
    }

    // 新しいバイナリで置き換え
    match fs::rename(&temp_path, &current_exe) {
        Ok(_) => {
            ui::success(&format!("AN を {} に更新しました", release.tag_name));
            // バックアップを削除
            let _ = fs::remove_file(&backup_path);
        }
        Err(e) => {
            // 失敗時はバックアップから復元
            ui::error(&format!("更新失敗: {}", e));
            if backup_path.exists() {
                let _ = fs::rename(&backup_path, &current_exe);
                ui::info("バックアップから復元しました");
            }
            return Err(e.into());
        }
    }

    Ok(())
}

/// インストール先ディレクトリを取得
#[allow(dead_code)]
fn install_dir() -> PathBuf {
    dirs::home_dir()
        .expect("ホームディレクトリが見つかりません")
        .join(".local")
        .join("bin")
}

/// updateコマンドのエントリーポイント
pub fn run() -> Result<()> {
    ui::info("アップデートを確認中...\n");

    // 最新バージョン確認
    let release = match check_latest_version() {
        Ok(r) => r,
        Err(e) => {
            ui::warn(&format!("バージョン確認失敗: {}", e));
            ui::info("アプリDBのみ更新します...\n");

            // DB更新のみ実行
            sync::run()?;
            return Ok(());
        }
    };

    let latest_version = parse_version(&release.tag_name);

    ui::info("AN アップデート:");
    ui::info(&format!("  現在のバージョン: {}", CURRENT_VERSION));
    ui::info(&format!("  最新バージョン:   {}", latest_version));
    println!();

    if should_update(CURRENT_VERSION, &release.tag_name) {
        ui::info(&format!("AN v{} をダウンロード中...", latest_version));

        match download_and_install(&release) {
            Ok(_) => {
                ui::success(&format!("AN を v{} に更新しました！", latest_version));
            }
            Err(e) => {
                ui::error(&format!("更新失敗: {}", e));
                ui::info("手動で更新してください:");
                ui::info("  curl -fsSL https://raw.githubusercontent.com/clearclown/AN/main/install.sh | bash");
            }
        }
    } else {
        ui::success(&format!("AN: 最新版です (v{})", CURRENT_VERSION));
    }

    println!();
    ui::info("アプリDB:");

    // DB更新 (syncコマンドを呼び出し)
    sync::run()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_should_update_newer() {
        assert!(should_update("0.1.0", "0.2.0"));
    }

    #[test]
    fn test_should_update_same() {
        assert!(!should_update("0.1.0", "0.1.0"));
    }

    #[test]
    fn test_should_update_older() {
        assert!(!should_update("0.2.0", "0.1.0"));
    }

    #[test]
    fn test_should_update_major() {
        assert!(should_update("0.9.9", "1.0.0"));
    }

    #[test]
    fn test_should_update_patch() {
        assert!(should_update("1.0.0", "1.0.1"));
    }
}
