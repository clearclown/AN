# AN エラーハンドリング設計

## 概要

ANのエラーハンドリングは `thiserror` と `anyhow` クレートを使用し、
型安全で情報量の多いエラー処理を実現します。

## エラー型定義

### AnError (src/errors.rs)

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AnError {
    // インストール関連
    #[error("E001: ファイルが見つかりません: {path}")]
    FileNotFound { path: String },

    #[error("E002: 不明なファイル形式です: {extension}")]
    UnknownFileType { extension: String },

    #[error("E003: アプリDBにエントリが見つかりません: {name}")]
    AppNotInDatabase { name: String },

    #[error("E004: ダウンロードに失敗しました: {url}")]
    DownloadFailed { url: String, source: reqwest::Error },

    #[error("E005: dpkg/権限エラー: {message}")]
    DpkgError { message: String },

    // 削除関連
    #[error("E101: アプリが見つかりません: {name}")]
    AppNotInstalled { name: String },

    #[error("E102: apt purgeに失敗しました: {message}")]
    AptPurgeError { message: String },

    #[error("E103: flatpak uninstallに失敗しました: {message}")]
    FlatpakUninstallError { message: String },

    // リンク関連
    #[error("E201: Flatpakがインストールされていません")]
    FlatpakNotInstalled,

    #[error("E202: シンボリックリンクの作成に失敗しました: {path}")]
    SymlinkCreationFailed { path: String },

    // アップデート関連
    #[error("E301: アップデート情報の取得に失敗しました")]
    UpdateCheckFailed,

    // 一般
    #[error("E901: I/Oエラー: {0}")]
    IoError(#[from] std::io::Error),

    #[error("E902: TOMLパースエラー: {0}")]
    TomlError(#[from] toml::de::Error),
}
```

## エラーコード体系

| 範囲 | カテゴリ |
|------|----------|
| E001-E099 | インストール関連エラー |
| E101-E199 | 削除関連エラー |
| E201-E299 | リンク関連エラー |
| E301-E399 | アップデート関連エラー |
| E901-E999 | 一般/システムエラー |

## エラー出力フォーマット

```
Error: E001: ファイルが見つかりません: /path/to/file.deb

原因: 指定されたパスにファイルが存在しません
対処: ファイルパスを確認してください
```

## 使用例

### Command層でのエラー処理

```rust
use anyhow::{Context, Result};

pub fn install(target: &str) -> Result<()> {
    let path = Path::new(target);

    if !path.exists() {
        return Err(AnError::FileNotFound {
            path: target.to_string(),
        }.into());
    }

    // 処理続行...
    Ok(())
}
```

### Handler層でのエラー伝播

```rust
pub fn install_deb(path: &Path) -> Result<()> {
    let output = Command::new("dpkg")
        .arg("-i")
        .arg(path)
        .output()
        .context("dpkgの実行に失敗しました")?;

    if !output.status.success() {
        return Err(AnError::DpkgError {
            message: String::from_utf8_lossy(&output.stderr).to_string(),
        }.into());
    }

    Ok(())
}
```

## 終了コード

| コード | 説明 |
|--------|------|
| 0 | 成功 |
| 1 | 一般エラー |
| 2 | 引数エラー |
| 3 | ファイルエラー |
| 4 | ネットワークエラー |
| 5 | 権限エラー |

## ユーザーフレンドリーなエラー表示

```rust
fn print_error(error: &AnError) {
    use colored::*;

    eprintln!("{} {}", "Error:".red().bold(), error);

    // エラーに応じたヒントを表示
    match error {
        AnError::FileNotFound { .. } => {
            eprintln!("{} ファイルパスを確認してください", "Hint:".yellow());
        }
        AnError::AppNotInDatabase { name } => {
            eprintln!("{} 'an search {}' でアプリを検索してみてください",
                      "Hint:".yellow(), name);
        }
        _ => {}
    }
}
```
