//! アプリ設定モジュール
//!
//! TOMLファイルからアプリ設定を読み込み、検索する機能を提供します。

use crate::errors::AnError;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::Path;

/// アプリ設定のルート構造体
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AppConfig {
    pub app: AppInfo,
    pub source: SourceInfo,
    #[serde(default)]
    pub metadata: Option<Metadata>,
}

/// アプリ基本情報
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AppInfo {
    /// アプリ名（コマンド名）
    pub name: String,
    /// 説明
    pub description: String,
    /// 公式サイトURL
    pub homepage: Option<String>,
}

/// ソース情報
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SourceInfo {
    /// インストールタイプ
    #[serde(rename = "type")]
    pub source_type: SourceType,
    /// ダウンロードURL（AppImage, Deb用）
    #[serde(default)]
    pub url: String,
    /// Flatpak ID（Flatpak用）
    pub flatpak_id: Option<String>,
    /// 対応アーキテクチャ
    pub architecture: Vec<String>,
}

/// インストールタイプ列挙
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum SourceType {
    AppImage,
    Deb,
    Flatpak,
    Script,
}

/// メタデータ（オプション）
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Metadata {
    pub categories: Option<Vec<String>>,
    pub desktop_entry: Option<bool>,
    pub version: Option<String>,
    pub maintainer: Option<String>,
}

/// アプリDBディレクトリを取得
fn db_dir() -> std::path::PathBuf {
    // 1. 環境変数 AN_DB_DIR
    // 2. ~/.config/an/apps/
    // 3. カレントディレクトリの apps/
    if let Ok(dir) = std::env::var("AN_DB_DIR") {
        return std::path::PathBuf::from(dir);
    }

    if let Some(config_dir) = dirs::config_dir() {
        let an_db = config_dir.join("an").join("apps");
        if an_db.exists() {
            return an_db;
        }
    }

    std::path::PathBuf::from("apps")
}

/// TOMLファイルからAppConfigを読み込む
pub fn load(path: &Path) -> Result<AppConfig> {
    let content = std::fs::read_to_string(path)?;
    let config: AppConfig = toml::from_str(&content)?;
    Ok(config)
}

/// 指定ディレクトリ内の全TOMLファイルを読み込む
pub fn load_all(dir: &Path) -> Result<Vec<AppConfig>> {
    let mut apps = Vec::new();

    if !dir.exists() {
        return Ok(apps);
    }

    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.extension().map(|e| e == "toml").unwrap_or(false) {
            match load(&path) {
                Ok(config) => apps.push(config),
                Err(e) => {
                    eprintln!("Warning: Failed to load {:?}: {}", path, e);
                }
            }
        }
    }

    Ok(apps)
}

/// アプリ名で検索
pub fn find_by_name(name: &str) -> Result<Option<AppConfig>> {
    let dir = db_dir();

    // まず直接ファイル名で検索
    let direct_path = dir.join(format!("{}.toml", name));
    if direct_path.exists() {
        return Ok(Some(load(&direct_path)?));
    }

    // 全ファイルをスキャンして検索
    let apps = load_all(&dir)?;
    Ok(apps.into_iter().find(|app| app.app.name == name))
}

/// AppConfigのバリデーション
#[allow(dead_code)]
pub fn validate(config: &AppConfig) -> Result<()> {
    // name は空でない
    if config.app.name.is_empty() {
        return Err(AnError::ValidationError {
            message: "name is required".to_string(),
        }
        .into());
    }

    // description は空でない
    if config.app.description.is_empty() {
        return Err(AnError::ValidationError {
            message: "description is required".to_string(),
        }
        .into());
    }

    // Flatpak以外はURLが必要
    if config.source.source_type != SourceType::Flatpak {
        if config.source.url.is_empty()
            || (!config.source.url.starts_with("http://") && !config.source.url.starts_with("https://")) {
            return Err(AnError::ValidationError {
                message: "url must be a valid HTTP(S) URL".to_string(),
            }
            .into());
        }
    } else {
        // Flatpakはflatpak_idが必要
        if config.source.flatpak_id.is_none() {
            return Err(AnError::ValidationError {
                message: "flatpak_id is required for Flatpak apps".to_string(),
            }
            .into());
        }
    }

    // architecture は1つ以上
    if config.source.architecture.is_empty() {
        return Err(AnError::ValidationError {
            message: "architecture must have at least one entry".to_string(),
        }
        .into());
    }

    Ok(())
}

/// 現在のアーキテクチャに対応するアプリのみをフィルタ
#[allow(dead_code)]
pub fn filter_by_architecture(apps: Vec<AppConfig>) -> Vec<AppConfig> {
    let current_arch = std::env::consts::ARCH;

    apps.into_iter()
        .filter(|app| app.source.architecture.iter().any(|a| a == current_arch))
        .collect()
}

/// URL内のプレースホルダーを展開
pub fn expand_url(url: &str, config: &AppConfig) -> String {
    let mut expanded = url.to_string();

    // {version} → metadata.version
    if let Some(ref metadata) = config.metadata {
        if let Some(ref version) = metadata.version {
            expanded = expanded.replace("{version}", version);
        }
    }

    // {arch} → 現在のアーキテクチャ
    expanded = expanded.replace("{arch}", std::env::consts::ARCH);

    expanded
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_config() -> AppConfig {
        AppConfig {
            app: AppInfo {
                name: "testapp".to_string(),
                description: "Test Application".to_string(),
                homepage: Some("https://example.com".to_string()),
            },
            source: SourceInfo {
                source_type: SourceType::AppImage,
                url: "https://example.com/app-{version}-{arch}.AppImage".to_string(),
                flatpak_id: None,
                architecture: vec!["x86_64".to_string()],
            },
            metadata: Some(Metadata {
                categories: Some(vec!["Utility".to_string()]),
                desktop_entry: Some(true),
                version: Some("1.0.0".to_string()),
                maintainer: None,
            }),
        }
    }

    #[test]
    fn test_source_type_deserialization() {
        let toml_str = r#"
[app]
name = "test"
description = "Test app"

[source]
type = "appimage"
url = "https://example.com/app.AppImage"
architecture = ["x86_64"]
"#;
        let config: AppConfig = toml::from_str(toml_str).unwrap();
        assert_eq!(config.source.source_type, SourceType::AppImage);
    }

    #[test]
    fn test_source_type_deb() {
        let toml_str = r#"
[app]
name = "test"
description = "Test app"

[source]
type = "deb"
url = "https://example.com/app.deb"
architecture = ["x86_64"]
"#;
        let config: AppConfig = toml::from_str(toml_str).unwrap();
        assert_eq!(config.source.source_type, SourceType::Deb);
    }

    #[test]
    fn test_validate_valid_config() {
        let config = sample_config();
        assert!(validate(&config).is_ok());
    }

    #[test]
    fn test_validate_empty_name() {
        let mut config = sample_config();
        config.app.name = String::new();
        assert!(validate(&config).is_err());
    }

    #[test]
    fn test_validate_empty_description() {
        let mut config = sample_config();
        config.app.description = String::new();
        assert!(validate(&config).is_err());
    }

    #[test]
    fn test_validate_invalid_url() {
        let mut config = sample_config();
        config.source.url = "not-a-url".to_string();
        assert!(validate(&config).is_err());
    }

    #[test]
    fn test_validate_empty_architecture() {
        let mut config = sample_config();
        config.source.architecture = vec![];
        assert!(validate(&config).is_err());
    }

    #[test]
    fn test_expand_url() {
        let config = sample_config();
        let expanded = expand_url(&config.source.url, &config);
        assert!(expanded.contains("1.0.0"));
        assert!(!expanded.contains("{version}"));
    }
}
