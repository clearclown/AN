//! link コマンド実装
//!
//! Flatpakアプリをスキャンし、短い名前で実行できるエイリアスを生成します。

use crate::errors::AnError;
use crate::handlers::flatpak;
use crate::utils::ui;
use anyhow::Result;

/// アプリ名を正規化
///
/// Flatpak IDから短いコマンド名を生成します。
/// 例: "org.gimp.GIMP" → "gimp"
/// 例: "com.spotify.Client" → "spotify"
pub fn normalize_name(app_id: &str) -> String {
    let parts: Vec<&str> = app_id.split('.').collect();
    let suffixes = ["client", "app", "desktop"];

    // 最後のパートを取得
    let last = parts.last().unwrap_or(&app_id);
    let lower = last.to_lowercase();

    // 最後のパートが接尾辞そのものの場合（例: Client）、前のパートを使用
    if suffixes.contains(&lower.as_str()) && parts.len() > 1 {
        return parts[parts.len() - 2].to_lowercase();
    }

    // 接尾辞を含む場合は除去（例: MyAppDesktop → myapp）
    let mut normalized = lower;
    for suffix in &suffixes {
        if normalized.ends_with(suffix) && normalized.len() > suffix.len() {
            normalized = normalized[..normalized.len() - suffix.len()].to_string();
            break;
        }
    }

    // 末尾のハイフンを除去
    normalized.trim_end_matches('-').to_string()
}

/// linkコマンドのエントリーポイント
pub fn run() -> Result<()> {
    ui::info("Scanning Flatpak applications...");

    // Flatpakの存在確認
    if !flatpak::is_installed() {
        return Err(AnError::FlatpakNotInstalled.into());
    }

    // アプリスキャン
    let apps = flatpak::scan_apps()?;

    if apps.is_empty() {
        ui::warn("No Flatpak applications found.");
        return Ok(());
    }

    let mut created = 0;
    let mut skipped = 0;

    ui::info("");
    ui::info("Created links:");

    for app in &apps {
        let name = normalize_name(&app.id);

        // 衝突チェック
        if flatpak::check_name_conflict(&name) {
            ui::warn(&format!("  {} → (skipped: name conflict)", name));
            skipped += 1;
            continue;
        }

        // ラッパー作成
        match flatpak::create_wrapper(&app.id, &name) {
            Ok(_) => {
                ui::info(&format!("  {:<12} → {}", name, app.id));
                created += 1;
            }
            Err(e) => {
                ui::error(&format!("  {} → (failed: {})", name, e));
                skipped += 1;
            }
        }
    }

    ui::info("");
    ui::success(&format!(
        "Summary: {} links created, {} skipped",
        created, skipped
    ));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_name_gimp() {
        assert_eq!(normalize_name("org.gimp.GIMP"), "gimp");
    }

    #[test]
    fn test_normalize_name_firefox() {
        assert_eq!(normalize_name("org.mozilla.firefox"), "firefox");
    }

    #[test]
    fn test_normalize_name_spotify() {
        assert_eq!(normalize_name("com.spotify.Client"), "spotify");
    }

    #[test]
    fn test_normalize_name_simple() {
        // "App" 接尾辞が除去される
        assert_eq!(normalize_name("SimpleApp"), "simple");
    }

    #[test]
    fn test_normalize_name_with_desktop_suffix() {
        assert_eq!(normalize_name("org.example.MyAppDesktop"), "myapp");
    }

    #[test]
    fn test_normalize_name_trailing_hyphen() {
        assert_eq!(normalize_name("org.example.App-"), "app");
    }
}
