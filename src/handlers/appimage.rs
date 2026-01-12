//! AppImageハンドラ
//!
//! AppImageファイルのインストール・削除を処理します。

use crate::errors::AnError;
use crate::utils::ui;
use anyhow::Result;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};

/// AppImage格納ディレクトリを取得
fn apps_dir() -> PathBuf {
    dirs::home_dir()
        .expect("ホームディレクトリが見つかりません")
        .join("Applications")
}

/// シンボリックリンク配置先を取得
fn bin_dir() -> PathBuf {
    dirs::home_dir()
        .expect("ホームディレクトリが見つかりません")
        .join(".local")
        .join("bin")
}

/// ファイル名からアプリ名を抽出
fn extract_app_name(path: &Path) -> String {
    path.file_stem()
        .and_then(|s| s.to_str())
        .map(|s| {
            // バージョン番号などを除去して簡潔な名前にする
            // 例: "Firefox-123.0" → "firefox"
            let name = s.split('-').next().unwrap_or(s);
            name.to_lowercase()
        })
        .unwrap_or_else(|| "app".to_string())
}

/// AppImageをインストール
pub fn install(path: &Path) -> Result<()> {
    let app_name = extract_app_name(path);
    ui::info(&format!("Installing AppImage: {}", app_name));

    // 格納ディレクトリの確認・作成
    let apps_directory = apps_dir();
    if !apps_directory.exists() {
        ui::info(&format!("Creating directory: {:?}", apps_directory));
        fs::create_dir_all(&apps_directory)?;
    }

    // ファイル移動
    let dest_path = apps_directory.join(path.file_name().unwrap());
    ui::info(&format!("Moving to: {:?}", dest_path));
    fs::copy(path, &dest_path)?;

    // 実行権限付与
    let mut perms = fs::metadata(&dest_path)?.permissions();
    perms.set_mode(0o755);
    fs::set_permissions(&dest_path, perms)?;

    // シンボリックリンク作成
    let bin_directory = bin_dir();
    if !bin_directory.exists() {
        fs::create_dir_all(&bin_directory)?;
    }

    let link_path = bin_directory.join(&app_name);
    if link_path.exists() {
        fs::remove_file(&link_path)?;
    }

    ui::info(&format!("Creating symlink: {:?}", link_path));
    std::os::unix::fs::symlink(&dest_path, &link_path)?;

    ui::success(&format!("AppImage '{}' installed", app_name));
    Ok(())
}

/// AppImageを削除
pub fn remove(identifier: &str) -> Result<()> {
    ui::info(&format!("Removing AppImage: {}", identifier));

    let link_path = bin_dir().join(identifier);

    // シンボリックリンクの確認
    if !link_path.is_symlink() {
        return Err(AnError::AppNotInstalled {
            name: identifier.to_string(),
        }
        .into());
    }

    // リンク先（AppImage実体）を取得
    let app_path = fs::read_link(&link_path)?;

    // シンボリックリンク削除
    ui::info(&format!("Removing symlink: {:?}", link_path));
    fs::remove_file(&link_path)?;

    // AppImage実体削除
    if app_path.exists() {
        ui::info(&format!("Removing file: {:?}", app_path));
        fs::remove_file(&app_path)?;
    }

    // デスクトップエントリ削除（存在すれば）
    let desktop_path = dirs::data_dir()
        .map(|d| d.join("applications").join(format!("{}.desktop", identifier)));

    if let Some(path) = desktop_path {
        if path.exists() {
            ui::info(&format!("Removing desktop entry: {:?}", path));
            fs::remove_file(path)?;
        }
    }

    ui::success("AppImage removed");
    Ok(())
}

/// AppImageリンクを検出
pub fn detect(name: &str) -> Option<String> {
    let link_path = bin_dir().join(name);

    if link_path.is_symlink() {
        // リンク先がApplicationsディレクトリを指しているか確認
        if let Ok(target) = fs::read_link(&link_path) {
            if target.starts_with(apps_dir()) {
                return Some(name.to_string());
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_app_name_simple() {
        let path = Path::new("/tmp/Firefox.AppImage");
        assert_eq!(extract_app_name(path), "firefox");
    }

    #[test]
    fn test_extract_app_name_with_version() {
        let path = Path::new("/tmp/Firefox-123.0.AppImage");
        assert_eq!(extract_app_name(path), "firefox");
    }

    #[test]
    fn test_extract_app_name_complex() {
        let path = Path::new("/tmp/Obsidian-1.5.3-x86_64.AppImage");
        assert_eq!(extract_app_name(path), "obsidian");
    }

    #[test]
    fn test_apps_dir() {
        let dir = apps_dir();
        assert!(dir.ends_with("Applications"));
    }

    #[test]
    fn test_bin_dir() {
        let dir = bin_dir();
        assert!(dir.ends_with(".local/bin"));
    }
}
