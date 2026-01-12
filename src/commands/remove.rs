//! remove コマンド実装
//!
//! インストールされたアプリを完全に削除します（パージ処理）。

use crate::errors::AnError;
use crate::handlers::{appimage, deb, flatpak};
use crate::utils::ui;
use anyhow::Result;

/// 検出されたインストール形式
#[derive(Debug, PartialEq)]
pub enum InstallType {
    AppImage,
    Deb,
    Flatpak,
}

/// 検出情報
#[derive(Debug)]
pub struct DetectionInfo {
    pub install_type: InstallType,
    pub identifier: String, // パッケージ名、アプリID、パスなど
}

/// アプリのインストール形式を検出
pub fn detect_type(target: &str) -> Option<DetectionInfo> {
    // 1. AppImageリンクをチェック
    if let Some(info) = appimage::detect(target) {
        return Some(DetectionInfo {
            install_type: InstallType::AppImage,
            identifier: info,
        });
    }

    // 2. Debパッケージをチェック
    if let Some(info) = deb::detect(target) {
        return Some(DetectionInfo {
            install_type: InstallType::Deb,
            identifier: info,
        });
    }

    // 3. Flatpakをチェック
    if let Some(info) = flatpak::detect(target) {
        return Some(DetectionInfo {
            install_type: InstallType::Flatpak,
            identifier: info,
        });
    }

    None
}

/// removeコマンドのエントリーポイント
pub fn run(target: &str) -> Result<()> {
    ui::info(&format!("Detecting installation type for '{}'...", target));

    let detection = detect_type(target)
        .ok_or_else(|| AnError::AppNotInstalled { name: target.to_string() })?;

    ui::info(&format!("Found: {:?}", detection.install_type));

    match detection.install_type {
        InstallType::AppImage => {
            appimage::remove(&detection.identifier)?;
        }
        InstallType::Deb => {
            deb::remove(&detection.identifier)?;
        }
        InstallType::Flatpak => {
            flatpak::remove(&detection.identifier)?;
        }
    }

    ui::success(&format!("{} removed successfully", target));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_install_type_variants() {
        assert_eq!(InstallType::AppImage, InstallType::AppImage);
        assert_eq!(InstallType::Deb, InstallType::Deb);
        assert_eq!(InstallType::Flatpak, InstallType::Flatpak);
        assert_ne!(InstallType::AppImage, InstallType::Deb);
    }
}
