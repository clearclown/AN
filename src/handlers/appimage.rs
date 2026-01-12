//! AppImageハンドラ
//!
//! AppImageファイルのインストール・削除を処理します。
//! - ~/Applications/ にAppImage本体を配置
//! - ~/.local/bin/ にシンボリックリンクを作成
//! - ~/.local/share/applications/ にデスクトップエントリを作成

use crate::errors::AnError;
use crate::utils::ui;
use anyhow::Result;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};

/// AppImage格納ディレクトリを取得
pub fn apps_dir() -> PathBuf {
    dirs::home_dir()
        .expect("ホームディレクトリが見つかりません")
        .join("Applications")
}

/// シンボリックリンク配置先を取得
pub fn bin_dir() -> PathBuf {
    dirs::home_dir()
        .expect("ホームディレクトリが見つかりません")
        .join(".local")
        .join("bin")
}

/// デスクトップエントリ配置先を取得
fn desktop_dir() -> PathBuf {
    dirs::data_dir()
        .unwrap_or_else(|| {
            dirs::home_dir()
                .expect("ホームディレクトリが見つかりません")
                .join(".local")
                .join("share")
        })
        .join("applications")
}

/// ファイル名からアプリ名を抽出
///
/// # 命名パターン対応
/// - "Firefox.AppImage" → "firefox"
/// - "Firefox-123.0.AppImage" → "firefox"
/// - "Obsidian-1.5.3-x86_64.AppImage" → "obsidian"
/// - "Some_App_Name.AppImage" → "some_app_name" → "some-app-name"
pub fn extract_app_name(path: &Path) -> String {
    path.file_stem()
        .and_then(|s| s.to_str())
        .map(|s| {
            // バージョン番号パターンを検出して除去
            // パターン: -数字.数字 または -x86_64 などのアーキテクチャ指定
            let name = s
                .split(|c| c == '-' || c == '_')
                .take_while(|part| {
                    // バージョン番号やアーキテクチャっぽいものは除外
                    !part.chars().next().map(|c| c.is_ascii_digit()).unwrap_or(false)
                        && *part != "x86_64"
                        && *part != "x64"
                        && *part != "amd64"
                        && *part != "aarch64"
                        && *part != "arm64"
                })
                .collect::<Vec<_>>()
                .join("-");

            if name.is_empty() {
                s.split(|c| c == '-' || c == '_')
                    .next()
                    .unwrap_or(s)
                    .to_lowercase()
            } else {
                name.to_lowercase()
            }
        })
        .unwrap_or_else(|| "app".to_string())
}

/// インストールオプション
#[derive(Default)]
pub struct InstallOptions {
    /// カスタムコマンド名（指定しない場合は自動抽出）
    pub name: Option<String>,
    /// デスクトップエントリを作成するか
    pub desktop_entry: bool,
    /// 元ファイルを削除するか（moveモード）
    pub remove_source: bool,
}

/// AppImageをインストール
pub fn install(path: &Path) -> Result<()> {
    install_with_options(path, InstallOptions::default())
}

/// オプション付きでAppImageをインストール
pub fn install_with_options(path: &Path, options: InstallOptions) -> Result<()> {
    // ファイル存在確認
    if !path.exists() {
        return Err(AnError::FileNotFound {
            path: path.display().to_string(),
        }
        .into());
    }

    let app_name = options.name.unwrap_or_else(|| extract_app_name(path));
    ui::info(&format!("AppImageをインストール中: {}", app_name));

    // 格納ディレクトリの確認・作成
    let apps_directory = apps_dir();
    if !apps_directory.exists() {
        ui::info(&format!("ディレクトリ作成: {:?}", apps_directory));
        fs::create_dir_all(&apps_directory)?;
    }

    // ファイルコピーまたは移動
    let original_filename = path.file_name().unwrap();
    let dest_path = apps_directory.join(original_filename);

    if dest_path.exists() {
        ui::warn(&format!("既存ファイルを上書き: {:?}", dest_path));
    }

    ui::info(&format!("コピー先: {:?}", dest_path));
    fs::copy(path, &dest_path)?;

    // 元ファイルを削除（オプション）
    if options.remove_source {
        fs::remove_file(path)?;
        ui::info("元ファイルを削除しました");
    }

    // 実行権限付与
    let mut perms = fs::metadata(&dest_path)?.permissions();
    perms.set_mode(0o755);
    fs::set_permissions(&dest_path, perms)?;
    ui::info("実行権限を付与");

    // シンボリックリンク作成
    let bin_directory = bin_dir();
    if !bin_directory.exists() {
        fs::create_dir_all(&bin_directory)?;
    }

    let link_path = bin_directory.join(&app_name);
    if link_path.exists() || link_path.is_symlink() {
        fs::remove_file(&link_path)?;
    }

    ui::info(&format!("シンボリックリンク作成: {:?}", link_path));
    std::os::unix::fs::symlink(&dest_path, &link_path)?;

    // デスクトップエントリ作成
    if options.desktop_entry {
        create_desktop_entry(&app_name, &dest_path)?;
    }

    ui::success(&format!(
        "AppImage '{}' をインストールしました",
        app_name
    ));
    ui::info(&format!("  コマンド: {}", app_name));
    ui::info(&format!("  場所: {:?}", dest_path));

    Ok(())
}

/// デスクトップエントリを作成
fn create_desktop_entry(app_name: &str, exec_path: &Path) -> Result<()> {
    let desktop_directory = desktop_dir();
    if !desktop_directory.exists() {
        fs::create_dir_all(&desktop_directory)?;
    }

    let desktop_path = desktop_directory.join(format!("{}.desktop", app_name));

    // タイトルケースの名前を生成（例: firefox → Firefox）
    let display_name = app_name
        .split('-')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(c) => c.to_uppercase().collect::<String>() + chars.as_str(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ");

    let desktop_content = format!(
        r#"[Desktop Entry]
Type=Application
Name={display_name}
Exec={exec_path}
Terminal=false
Categories=Application;
Comment=Installed via AN
"#,
        display_name = display_name,
        exec_path = exec_path.display()
    );

    fs::write(&desktop_path, desktop_content)?;
    ui::info(&format!("デスクトップエントリ作成: {:?}", desktop_path));

    Ok(())
}

/// AppImageを削除
pub fn remove(identifier: &str) -> Result<()> {
    ui::info(&format!("AppImageを削除中: {}", identifier));

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
    ui::info(&format!("シンボリックリンク削除: {:?}", link_path));
    fs::remove_file(&link_path)?;

    // AppImage実体削除
    if app_path.exists() {
        ui::info(&format!("ファイル削除: {:?}", app_path));
        fs::remove_file(&app_path)?;
    }

    // デスクトップエントリ削除（存在すれば）
    let desktop_path = desktop_dir().join(format!("{}.desktop", identifier));
    if desktop_path.exists() {
        ui::info(&format!("デスクトップエントリ削除: {:?}", desktop_path));
        fs::remove_file(&desktop_path)?;
    }

    ui::success("AppImageを削除しました");
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

/// インストール済みAppImage一覧を取得
pub fn list_installed() -> Result<Vec<String>> {
    let bin_directory = bin_dir();
    let apps_directory = apps_dir();
    let mut installed = Vec::new();

    if !bin_directory.exists() {
        return Ok(installed);
    }

    for entry in fs::read_dir(&bin_directory)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_symlink() {
            if let Ok(target) = fs::read_link(&path) {
                if target.starts_with(&apps_directory) {
                    if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                        installed.push(name.to_string());
                    }
                }
            }
        }
    }

    installed.sort();
    Ok(installed)
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
    fn test_extract_app_name_underscore() {
        let path = Path::new("/tmp/Some_Cool_App-2.0.AppImage");
        assert_eq!(extract_app_name(path), "some-cool-app");
    }

    #[test]
    fn test_extract_app_name_keepassxc() {
        let path = Path::new("/tmp/KeePassXC-2.7.6-x86_64.AppImage");
        assert_eq!(extract_app_name(path), "keepassxc");
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

    #[test]
    fn test_desktop_dir() {
        let dir = desktop_dir();
        assert!(dir.to_string_lossy().contains("applications"));
    }
}
