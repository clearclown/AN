# アプリDBスキーマ仕様

## 概要

ANのアプリデータベースはTOML形式で管理されます。
各アプリは `apps/<app_name>.toml` として保存されます。

## ファイル構造

```
apps/
├── firefox.toml
├── gimp.toml
├── obsidian.toml
├── vscode.toml
└── ...
```

## スキーマ定義

### 基本構造

```toml
[app]
name = "string"           # 必須: アプリ名（コマンド名）
description = "string"    # 必須: アプリの説明
homepage = "string"       # オプション: 公式サイトURL

[source]
type = "string"           # 必須: appimage | deb | flatpak | script
url = "string"            # 必須: ダウンロードURL
architecture = ["string"] # 必須: 対応アーキテクチャ

[metadata]
categories = ["string"]   # オプション: カテゴリ
desktop_entry = bool      # オプション: デスクトップエントリ生成
version = "string"        # オプション: バージョン情報
maintainer = "string"     # オプション: メンテナ情報
```

### フィールド詳細

#### [app] セクション

| フィールド | 型 | 必須 | 説明 |
|-----------|-----|------|------|
| name | String | Yes | アプリ名。コマンド名として使用される |
| description | String | Yes | アプリの説明（1行） |
| homepage | String | No | 公式サイトのURL |

#### [source] セクション

| フィールド | 型 | 必須 | 説明 |
|-----------|-----|------|------|
| type | String | Yes | インストールタイプ |
| url | String | Yes | ダウンロードURL |
| architecture | [String] | Yes | 対応アーキテクチャ |

**type の値:**
- `appimage`: AppImageファイル
- `deb`: Debianパッケージ
- `flatpak`: Flatpakアプリ（参照用）
- `script`: カスタムインストールスクリプト

**architecture の値:**
- `x86_64`: 64bit Intel/AMD
- `aarch64`: 64bit ARM
- `i686`: 32bit Intel/AMD

#### [metadata] セクション

| フィールド | 型 | 必須 | 説明 |
|-----------|-----|------|------|
| categories | [String] | No | アプリのカテゴリ |
| desktop_entry | bool | No | デスクトップエントリ生成 (default: true) |
| version | String | No | 現在のバージョン |
| maintainer | String | No | TOMLファイルのメンテナ |

## 具体例

### AppImage (firefox.toml)

```toml
[app]
name = "firefox"
description = "Mozilla Firefox Web Browser"
homepage = "https://www.mozilla.org/firefox/"

[source]
type = "appimage"
url = "https://github.com/nickvandewiele/firefox-appimage/releases/latest/download/Firefox_x86_64.AppImage"
architecture = ["x86_64"]

[metadata]
categories = ["Network", "WebBrowser"]
desktop_entry = true
```

### Deb (vscode.toml)

```toml
[app]
name = "code"
description = "Visual Studio Code - Code Editor"
homepage = "https://code.visualstudio.com/"

[source]
type = "deb"
url = "https://code.visualstudio.com/sha/download?build=stable&os=linux-deb-x64"
architecture = ["x86_64"]

[metadata]
categories = ["Development", "IDE"]
desktop_entry = true
```

### 複数アーキテクチャ対応 (obsidian.toml)

```toml
[app]
name = "obsidian"
description = "Obsidian - A second brain, for you, forever"
homepage = "https://obsidian.md/"

[source]
type = "appimage"
url = "https://github.com/obsidianmd/obsidian-releases/releases/latest/download/Obsidian-{version}-{arch}.AppImage"
architecture = ["x86_64", "aarch64"]

[metadata]
categories = ["Office", "NoteTaking"]
version = "1.5.3"
```

### URL パターン

動的なURL生成のためのプレースホルダー:

| プレースホルダー | 説明 | 例 |
|-----------------|------|-----|
| `{version}` | アプリのバージョン | 1.5.3 |
| `{arch}` | アーキテクチャ | x86_64, aarch64 |

## Rust構造体定義

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct AppConfig {
    pub app: AppInfo,
    pub source: SourceInfo,
    pub metadata: Option<Metadata>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AppInfo {
    pub name: String,
    pub description: String,
    pub homepage: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SourceInfo {
    #[serde(rename = "type")]
    pub source_type: SourceType,
    pub url: String,
    pub architecture: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum SourceType {
    AppImage,
    Deb,
    Flatpak,
    Script,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Metadata {
    pub categories: Option<Vec<String>>,
    pub desktop_entry: Option<bool>,
    pub version: Option<String>,
    pub maintainer: Option<String>,
}
```

## バリデーションルール

### 必須フィールド検証

```rust
fn validate(config: &AppConfig) -> Result<()> {
    // name は空でない
    if config.app.name.is_empty() {
        return Err(AnError::ValidationError("name is required".into()));
    }

    // description は空でない
    if config.app.description.is_empty() {
        return Err(AnError::ValidationError("description is required".into()));
    }

    // url は有効なURL形式
    if !config.source.url.starts_with("http") {
        return Err(AnError::ValidationError("url must be a valid HTTP(S) URL".into()));
    }

    // architecture は1つ以上
    if config.source.architecture.is_empty() {
        return Err(AnError::ValidationError("architecture must have at least one entry".into()));
    }

    Ok(())
}
```

### アーキテクチャフィルタリング

```rust
fn filter_by_arch(configs: Vec<AppConfig>) -> Vec<AppConfig> {
    let current_arch = std::env::consts::ARCH;

    configs
        .into_iter()
        .filter(|c| c.source.architecture.contains(&current_arch.to_string()))
        .collect()
}
```

## テストケース

| ID | テスト内容 | 入力 | 期待結果 |
|----|-----------|------|----------|
| TC401 | 有効なTOMLパース | firefox.toml | AppConfig構造体 |
| TC402 | 必須フィールド欠損 | name欠損 | ValidationError |
| TC403 | 無効なURL | "not-a-url" | ValidationError |
| TC404 | 空のアーキテクチャ | [] | ValidationError |
| TC405 | 不明なtype | "unknown" | ParseError |
| TC406 | プレースホルダー展開 | "{version}" | 実際の値に置換 |

## 関連ドキュメント

- [install コマンド仕様](./install.md)
- [DB層API](../api/db.md)
