//! エラー型定義モジュール
//!
//! ANのエラーハンドリングを統一的に管理します。

use thiserror::Error;

/// ANのエラー型
#[derive(Error, Debug)]
pub enum AnError {
    // インストール関連 (E001-E099)
    #[error("E001: ファイルが見つかりません: {path}")]
    FileNotFound { path: String },

    #[error("E002: 不明なファイル形式です: {extension}")]
    UnknownFileType { extension: String },

    #[error("E003: アプリDBにエントリが見つかりません: {name}")]
    AppNotInDatabase { name: String },

    #[error("E004: ダウンロードに失敗しました: {url}")]
    DownloadFailed { url: String },

    #[error("E005: dpkg/権限エラー: {message}")]
    DpkgError { message: String },

    // 削除関連 (E101-E199)
    #[error("E101: アプリが見つかりません: {name}")]
    AppNotInstalled { name: String },

    #[error("E102: apt purgeに失敗しました: {message}")]
    AptPurgeError { message: String },

    #[error("E103: flatpak uninstallに失敗しました: {message}")]
    FlatpakUninstallError { message: String },

    #[error("E006: flatpak installに失敗しました: {message}")]
    FlatpakInstallError { message: String },

    // リンク関連 (E201-E299)
    #[error("E201: Flatpakがインストールされていません")]
    FlatpakNotInstalled,

    #[error("E202: シンボリックリンクの作成に失敗しました: {path}")]
    #[allow(dead_code)]
    SymlinkCreationFailed { path: String },

    // アップデート関連 (E301-E399)
    #[error("E301: アップデート情報の取得に失敗しました")]
    UpdateCheckFailed,

    // 一般 (E901-E999)
    #[error("E901: I/Oエラー: {0}")]
    IoError(#[from] std::io::Error),

    #[error("E902: TOMLパースエラー: {0}")]
    TomlError(#[from] toml::de::Error),

    #[error("E903: バリデーションエラー: {message}")]
    ValidationError { message: String },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display_file_not_found() {
        let err = AnError::FileNotFound {
            path: "/path/to/file".to_string(),
        };
        assert_eq!(
            err.to_string(),
            "E001: ファイルが見つかりません: /path/to/file"
        );
    }

    #[test]
    fn test_error_display_unknown_file_type() {
        let err = AnError::UnknownFileType {
            extension: "xyz".to_string(),
        };
        assert_eq!(err.to_string(), "E002: 不明なファイル形式です: xyz");
    }

    #[test]
    fn test_error_display_app_not_in_db() {
        let err = AnError::AppNotInDatabase {
            name: "unknownapp".to_string(),
        };
        assert_eq!(
            err.to_string(),
            "E003: アプリDBにエントリが見つかりません: unknownapp"
        );
    }
}
