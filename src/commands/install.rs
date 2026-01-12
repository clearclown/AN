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
    let path = Path::new(target);

    if path.exists() {
        // ローカルファイル処理
        install_local(target)
    } else {
        // リモートアプリ処理
        install_remote(target)
    }
}

/// ローカルファイルのインストール
fn install_local(path: &str) -> Result<()> {
    ui::info(&format!("Installing local file: {}", path));

    let file_type = detect_file_type(path)?;

    match file_type {
        FileType::Deb => {
            ui::info("Detected: Debian package (.deb)");
            deb::install(Path::new(path))?;
        }
        FileType::AppImage => {
            ui::info("Detected: AppImage");
            appimage::install(Path::new(path))?;
        }
    }

    ui::success("Installation completed successfully");
    Ok(())
}

/// リモートアプリのインストール
fn install_remote(name: &str) -> Result<()> {
    ui::info(&format!("Searching for app: {}", name));

    // アプリDBから検索
    let app_config = db::find_by_name(name)?
        .ok_or_else(|| AnError::AppNotInDatabase { name: name.to_string() })?;

    // URL表示と確認
    ui::info(&format!("Source: {}", app_config.source.url));

    if !ui::confirm("Continue?")? {
        ui::warn("Installation cancelled");
        return Ok(());
    }

    // ダウンロード
    let downloaded_path = remote::download(&app_config.source.url, &app_config.app.name)?;

    // ファイルタイプに応じた処理
    let file_type = detect_file_type(downloaded_path.to_str().unwrap())?;
    match file_type {
        FileType::Deb => deb::install(&downloaded_path)?,
        FileType::AppImage => appimage::install(&downloaded_path)?,
    }

    // 一時ファイル削除
    fs_utils::remove_file(&downloaded_path)?;

    ui::success(&format!("{} installed successfully", name));
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
}
