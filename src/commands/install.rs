//! install コマンド実装
//!
//! ローカルファイル（.deb, .AppImage）またはリモートアプリをインストールします。

use crate::db;
use crate::errors::AnError;
use crate::handlers::{appimage, deb, remote};
use crate::utils::{fs as fs_utils, ui};
use anyhow::Result;
use std::path::Path;

/// ファイルタイプ
#[derive(Debug, PartialEq)]
pub enum FileType {
    Deb,
    AppImage,
}

/// インストールオプション
#[derive(Default)]
pub struct InstallOptions {
    /// カスタムコマンド名（AppImage用）
    pub name: Option<String>,
    /// デスクトップエントリを作成（AppImage用）
    pub desktop: bool,
    /// 元ファイルを削除（移動モード）
    pub move_file: bool,
}

/// ファイルの拡張子からタイプを判定
pub fn detect_file_type(path: &str) -> Result<FileType, AnError> {
    let path = Path::new(path);
    let extension = path
        .extension()
        .and_then(|e| e.to_str())
        .map(|e| e.to_lowercase());

    match extension.as_deref() {
        Some("deb") => Ok(FileType::Deb),
        Some("appimage") => Ok(FileType::AppImage),
        Some(ext) => Err(AnError::UnknownFileType {
            extension: ext.to_string(),
        }),
        None => Err(AnError::UnknownFileType {
            extension: "なし".to_string(),
        }),
    }
}

/// installコマンドのエントリーポイント
pub fn run(target: &str) -> Result<()> {
    run_with_options(target, InstallOptions::default())
}

/// オプション付きinstallコマンドのエントリーポイント
pub fn run_with_options(target: &str, options: InstallOptions) -> Result<()> {
    let path = Path::new(target);

    if path.exists() {
        // ローカルファイル処理
        install_local(target, options)
    } else {
        // リモートアプリ処理
        install_remote(target, options)
    }
}

/// ローカルファイルのインストール
fn install_local(path: &str, options: InstallOptions) -> Result<()> {
    ui::info(&format!("ローカルファイルをインストール: {}", path));

    let file_type = detect_file_type(path)?;

    match file_type {
        FileType::Deb => {
            ui::info("検出: Debianパッケージ (.deb)");
            deb::install(Path::new(path))?;
        }
        FileType::AppImage => {
            ui::info("検出: AppImage");
            let appimage_options = appimage::InstallOptions {
                name: options.name,
                desktop_entry: options.desktop,
                remove_source: options.move_file,
            };
            appimage::install_with_options(Path::new(path), appimage_options)?;
        }
    }

    Ok(())
}

/// リモートアプリのインストール
fn install_remote(name: &str, options: InstallOptions) -> Result<()> {
    ui::info(&format!("アプリを検索中: {}", name));

    // アプリDBから検索
    let app_config = db::find_by_name(name)?
        .ok_or_else(|| AnError::AppNotInDatabase { name: name.to_string() })?;

    // URL表示と確認
    ui::info(&format!("ソース: {}", app_config.source.url));

    if !ui::confirm("続行しますか?")? {
        ui::warn("インストールをキャンセルしました");
        return Ok(());
    }

    // ダウンロード
    let downloaded_path = remote::download(&app_config.source.url, &app_config.app.name)?;

    // ファイルタイプに応じた処理
    let file_type = detect_file_type(downloaded_path.to_str().unwrap())?;
    match file_type {
        FileType::Deb => deb::install(&downloaded_path)?,
        FileType::AppImage => {
            let appimage_options = appimage::InstallOptions {
                name: options.name,
                desktop_entry: app_config.metadata
                    .as_ref()
                    .map(|m| m.desktop_entry.unwrap_or(false))
                    .unwrap_or(false) || options.desktop,
                remove_source: true, // ダウンロードしたファイルは常に削除
            };
            appimage::install_with_options(&downloaded_path, appimage_options)?;
        }
    }

    // 一時ファイル削除（debの場合）
    if file_type == FileType::Deb {
        fs_utils::remove_file(&downloaded_path)?;
    }

    ui::success(&format!("{} をインストールしました", name));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_file_type_deb() {
        let result = detect_file_type("package.deb");
        assert_eq!(result.unwrap(), FileType::Deb);
    }

    #[test]
    fn test_detect_file_type_appimage() {
        let result = detect_file_type("app.AppImage");
        assert_eq!(result.unwrap(), FileType::AppImage);
    }

    #[test]
    fn test_detect_file_type_appimage_lowercase() {
        let result = detect_file_type("app.appimage");
        assert_eq!(result.unwrap(), FileType::AppImage);
    }

    #[test]
    fn test_detect_file_type_unknown() {
        let result = detect_file_type("file.xyz");
        assert!(result.is_err());
        match result {
            Err(AnError::UnknownFileType { extension }) => {
                assert_eq!(extension, "xyz");
            }
            _ => panic!("Expected UnknownFileType error"),
        }
    }

    #[test]
    fn test_detect_file_type_no_extension() {
        let result = detect_file_type("file");
        assert!(result.is_err());
    }

    #[test]
    fn test_install_options_default() {
        let options = InstallOptions::default();
        assert!(options.name.is_none());
        assert!(!options.desktop);
        assert!(!options.move_file);
    }
}
