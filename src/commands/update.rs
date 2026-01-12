//! update コマンド実装
//!
//! AN本体のアップデートとアプリDBの更新を行います。

use crate::errors::AnError;
use crate::utils::ui;
use anyhow::Result;
use semver::Version;

/// 現在のバージョン
const CURRENT_VERSION: &str = env!("CARGO_PKG_VERSION");

/// 最新バージョンを取得
fn check_latest_version() -> Result<String> {
    // TODO: GitHub Releases APIから取得
    // 現在はスタブ実装
    ui::info("Checking for updates...");

    // 将来の実装:
    // let url = "https://api.github.com/repos/clearclown/AN/releases/latest";
    // let response: serde_json::Value = reqwest::blocking::get(url)?.json()?;
    // let version = response["tag_name"].as_str()...

    Ok(CURRENT_VERSION.to_string())
}

/// バージョン比較
pub fn should_update(current: &str, latest: &str) -> bool {
    let current = Version::parse(current).unwrap_or_else(|_| Version::new(0, 0, 0));
    let latest = Version::parse(latest).unwrap_or_else(|_| Version::new(0, 0, 0));
    latest > current
}

/// updateコマンドのエントリーポイント
pub fn run() -> Result<()> {
    ui::info("Checking for updates...");
    ui::info("");

    // 最新バージョン確認
    let latest = check_latest_version().map_err(|_| AnError::UpdateCheckFailed)?;

    ui::info("AN Update:");
    ui::info(&format!("  Current version: {}", CURRENT_VERSION));
    ui::info(&format!("  Latest version:  {}", latest));
    ui::info("");

    if should_update(CURRENT_VERSION, &latest) {
        ui::info(&format!("Downloading AN v{}...", latest));
        // TODO: ダウンロードとインストール処理
        ui::warn("Update functionality not yet implemented");
    } else {
        ui::success(&format!("AN: Already up to date (v{})", CURRENT_VERSION));
    }

    ui::info("");
    ui::info("App Database:");
    ui::info("  Fetching latest database...");
    // TODO: DB更新処理
    ui::warn("  Database update not yet implemented");

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
