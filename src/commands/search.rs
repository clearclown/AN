//! search コマンド実装
//!
//! アプリDBを検索してアプリ情報を表示します。

use crate::db;
use crate::utils::ui;
use anyhow::Result;
use std::path::PathBuf;

/// searchコマンドのエントリーポイント
pub fn run(query: &str) -> Result<()> {
    ui::info(&format!("「{}」を検索中...\n", query));

    let db_dir = get_db_dir();
    let apps = db::app::load_all(&db_dir)?;

    if apps.is_empty() {
        ui::warn("アプリDBが空です");
        return Ok(());
    }

    let query_lower = query.to_lowercase();
    let results: Vec<_> = apps
        .iter()
        .filter(|app| {
            app.app.name.to_lowercase().contains(&query_lower)
                || app.app.description.to_lowercase().contains(&query_lower)
        })
        .collect();

    if results.is_empty() {
        ui::warn(&format!("「{}」に一致するアプリが見つかりません", query));
        return Ok(());
    }

    ui::success(&format!("{} 件のアプリが見つかりました:\n", results.len()));

    for app in results {
        println!("  {} - {}", app.app.name, app.app.description);
        println!("    タイプ: {:?}", app.source.source_type);
        if let Some(ref homepage) = app.app.homepage {
            println!("    ホームページ: {}", homepage);
        }
        println!();
    }

    Ok(())
}

/// アプリDBディレクトリを取得
fn get_db_dir() -> PathBuf {
    // 1. 環境変数 AN_DB_DIR
    // 2. ~/.config/an/apps/
    // 3. カレントディレクトリの apps/
    if let Ok(dir) = std::env::var("AN_DB_DIR") {
        return PathBuf::from(dir);
    }

    if let Some(config_dir) = dirs::config_dir() {
        let an_db = config_dir.join("an").join("apps");
        if an_db.exists() {
            return an_db;
        }
    }

    PathBuf::from("apps")
}

/// アプリ詳細を表示
pub fn show_details(name: &str) -> Result<()> {
    let app = db::find_by_name(name)?;

    match app {
        Some(config) => {
            println!("=== {} ===", config.app.name);
            println!("説明: {}", config.app.description);
            println!("タイプ: {:?}", config.source.source_type);

            // Flatpakの場合はflatpak_idを表示、それ以外はURL
            if let Some(ref flatpak_id) = config.source.flatpak_id {
                println!("Flatpak ID: {}", flatpak_id);
            }
            if !config.source.url.is_empty() {
                println!("URL: {}", config.source.url);
            }

            println!("アーキテクチャ: {}", config.source.architecture.join(", "));

            if let Some(ref homepage) = config.app.homepage {
                println!("ホームページ: {}", homepage);
            }

            if let Some(ref metadata) = config.metadata {
                if let Some(ref version) = metadata.version {
                    println!("バージョン: {}", version);
                }
                if let Some(ref categories) = metadata.categories {
                    println!("カテゴリ: {}", categories.join(", "));
                }
            }

            Ok(())
        }
        None => {
            ui::warn(&format!("アプリ「{}」が見つかりません", name));
            Ok(())
        }
    }
}

/// DB内の全アプリ一覧
pub fn list_all() -> Result<()> {
    let db_dir = get_db_dir();
    let apps = db::app::load_all(&db_dir)?;

    if apps.is_empty() {
        ui::warn("アプリDBが空です");
        return Ok(());
    }

    ui::info(&format!("利用可能なアプリ ({} 件):\n", apps.len()));

    // タイプ別に分類
    let mut appimages: Vec<_> = apps
        .iter()
        .filter(|a| a.source.source_type == db::app::SourceType::AppImage)
        .collect();
    let mut debs: Vec<_> = apps
        .iter()
        .filter(|a| a.source.source_type == db::app::SourceType::Deb)
        .collect();
    let mut flatpaks: Vec<_> = apps
        .iter()
        .filter(|a| a.source.source_type == db::app::SourceType::Flatpak)
        .collect();

    appimages.sort_by(|a, b| a.app.name.cmp(&b.app.name));
    debs.sort_by(|a, b| a.app.name.cmp(&b.app.name));
    flatpaks.sort_by(|a, b| a.app.name.cmp(&b.app.name));

    if !appimages.is_empty() {
        println!("=== AppImage ({}) ===", appimages.len());
        for app in appimages {
            println!("  {} - {}", app.app.name, app.app.description);
        }
        println!();
    }

    if !debs.is_empty() {
        println!("=== Deb ({}) ===", debs.len());
        for app in debs {
            println!("  {} - {}", app.app.name, app.app.description);
        }
        println!();
    }

    if !flatpaks.is_empty() {
        println!("=== Flatpak ({}) ===", flatpaks.len());
        for app in flatpaks {
            println!("  {} - {}", app.app.name, app.app.description);
        }
        println!();
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_db_dir() {
        let dir = get_db_dir();
        // デフォルトは "apps"
        assert!(dir.to_string_lossy().contains("apps"));
    }
}
