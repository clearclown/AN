//! list コマンド実装
//!
//! インストール済みアプリの一覧を表示します。

use crate::handlers::{appimage, flatpak};
use crate::utils::ui;
use anyhow::Result;
use std::process::Command;

/// インストール済みアプリの種別
#[derive(Debug)]
pub struct InstalledApp {
    pub name: String,
    pub source: AppSource,
    pub path: Option<String>,
}

/// アプリのインストール元
#[derive(Debug)]
pub enum AppSource {
    AppImage,
    Flatpak,
    Deb,
}

impl std::fmt::Display for AppSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppSource::AppImage => write!(f, "AppImage"),
            AppSource::Flatpak => write!(f, "Flatpak"),
            AppSource::Deb => write!(f, "Deb"),
        }
    }
}

/// listコマンドのエントリーポイント
pub fn run() -> Result<()> {
    ui::info("インストール済みアプリ一覧:\n");

    let mut total = 0;

    // AppImage一覧
    ui::info("=== AppImage ===");
    match appimage::list_installed() {
        Ok(apps) if !apps.is_empty() => {
            for app in &apps {
                println!("  {} ({})", app, appimage::apps_dir().join(format!("{}.AppImage", app)).display());
            }
            total += apps.len();
        }
        Ok(_) => {
            println!("  (なし)");
        }
        Err(e) => {
            ui::warn(&format!("AppImage一覧取得エラー: {}", e));
        }
    }
    println!();

    // Flatpak一覧
    ui::info("=== Flatpak ===");
    if flatpak::is_installed() {
        match flatpak::scan_apps() {
            Ok(apps) if !apps.is_empty() => {
                for app in &apps {
                    println!("  {} ({})", app.id.split('.').last().unwrap_or(&app.id), app.id);
                }
                total += apps.len();
            }
            Ok(_) => {
                println!("  (なし)");
            }
            Err(e) => {
                ui::warn(&format!("Flatpak一覧取得エラー: {}", e));
            }
        }
    } else {
        println!("  (Flatpakがインストールされていません)");
    }
    println!();

    // Deb パッケージ一覧（ANでインストールしたもののみ表示は困難なので、最近インストールしたものを表示）
    ui::info("=== Deb (apt経由で管理) ===");
    println!("  `dpkg -l | grep <パッケージ名>` で確認してください");
    println!();

    ui::success(&format!("合計: {} アプリ", total));
    Ok(())
}

/// 特定のアプリがインストールされているか確認
pub fn is_installed(name: &str) -> Option<InstalledApp> {
    // AppImageをチェック
    if appimage::detect(name).is_some() {
        return Some(InstalledApp {
            name: name.to_string(),
            source: AppSource::AppImage,
            path: Some(appimage::apps_dir().join(format!("{}.AppImage", name)).display().to_string()),
        });
    }

    // Flatpakをチェック
    if let Some(app_id) = flatpak::detect(name) {
        return Some(InstalledApp {
            name: name.to_string(),
            source: AppSource::Flatpak,
            path: Some(app_id),
        });
    }

    // Debをチェック
    if let Ok(output) = Command::new("dpkg").args(["-l", name]).output() {
        if output.status.success() {
            return Some(InstalledApp {
                name: name.to_string(),
                source: AppSource::Deb,
                path: None,
            });
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_source_display() {
        assert_eq!(format!("{}", AppSource::AppImage), "AppImage");
        assert_eq!(format!("{}", AppSource::Flatpak), "Flatpak");
        assert_eq!(format!("{}", AppSource::Deb), "Deb");
    }
}
