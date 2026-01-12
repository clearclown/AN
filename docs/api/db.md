# DB層 API仕様

## 概要

DB層はTOML形式のアプリデータベースを管理します。

## モジュール構成

```
src/db/
├── mod.rs    # エクスポート
└── app.rs    # App構造体、読み込み、検索
```

## app.rs

### 構造体

#### AppConfig

```rust
use serde::{Deserialize, Serialize};

/// アプリ設定のルート構造体
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AppConfig {
    pub app: AppInfo,
    pub source: SourceInfo,
    #[serde(default)]
    pub metadata: Option<Metadata>,
}
```

#### AppInfo

```rust
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
```

#### SourceInfo

```rust
/// ソース情報
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SourceInfo {
    /// インストールタイプ
    #[serde(rename = "type")]
    pub source_type: SourceType,
    /// ダウンロードURL
    pub url: String,
    /// 対応アーキテクチャ
    pub architecture: Vec<String>,
}
```

#### SourceType

```rust
/// インストールタイプ列挙
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum SourceType {
    AppImage,
    Deb,
    Flatpak,
    Script,
}
```

#### Metadata

```rust
/// メタデータ（オプション）
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Metadata {
    pub categories: Option<Vec<String>>,
    #[serde(default = "default_desktop_entry")]
    pub desktop_entry: Option<bool>,
    pub version: Option<String>,
    pub maintainer: Option<String>,
}

fn default_desktop_entry() -> Option<bool> {
    Some(true)
}
```

### 関数

#### load

```rust
/// TOMLファイルからAppConfigを読み込む
///
/// # Arguments
/// * `path` - TOMLファイルのパス
///
/// # Returns
/// * `Ok(AppConfig)` - パース成功
/// * `Err(AnError)` - ファイル読み込みまたはパースエラー
///
/// # Example
/// ```
/// let config = load(Path::new("apps/firefox.toml"))?;
/// println!("App: {}", config.app.name);
/// ```
pub fn load(path: &Path) -> Result<AppConfig>;
```

#### load_all

```rust
/// 指定ディレクトリ内の全TOMLファイルを読み込む
///
/// # Arguments
/// * `dir` - アプリDBディレクトリ
///
/// # Returns
/// * 読み込んだAppConfigのベクタ
/// * 読み込みに失敗したファイルはスキップ（警告ログ出力）
///
/// # Example
/// ```
/// let apps = load_all(Path::new("apps/"))?;
/// println!("Loaded {} apps", apps.len());
/// ```
pub fn load_all(dir: &Path) -> Result<Vec<AppConfig>>;
```

#### find_by_name

```rust
/// アプリ名で検索
///
/// # Arguments
/// * `name` - 検索するアプリ名
/// * `dir` - アプリDBディレクトリ
///
/// # Returns
/// * `Some(AppConfig)` - アプリが見つかった
/// * `None` - アプリが見つからない
///
/// # Example
/// ```
/// if let Some(app) = find_by_name("firefox", Path::new("apps/"))? {
///     println!("Found: {}", app.source.url);
/// }
/// ```
pub fn find_by_name(name: &str, dir: &Path) -> Result<Option<AppConfig>>;
```

#### validate

```rust
/// AppConfigのバリデーション
///
/// # Arguments
/// * `config` - 検証するAppConfig
///
/// # Returns
/// * `Ok(())` - バリデーション成功
/// * `Err(AnError)` - バリデーションエラー
///
/// # Validation Rules
/// - name: 空でない、英数字とハイフンのみ
/// - description: 空でない
/// - url: http:// または https:// で始まる
/// - architecture: 1つ以上の要素
pub fn validate(config: &AppConfig) -> Result<()>;
```

#### filter_by_architecture

```rust
/// 現在のアーキテクチャに対応するアプリのみをフィルタ
///
/// # Arguments
/// * `apps` - フィルタ対象のAppConfigリスト
///
/// # Returns
/// * 現在のアーキテクチャに対応するアプリのみ
///
/// # Example
/// ```
/// let all_apps = load_all(Path::new("apps/"))?;
/// let compatible_apps = filter_by_architecture(all_apps);
/// ```
pub fn filter_by_architecture(apps: Vec<AppConfig>) -> Vec<AppConfig>;
```

#### expand_url

```rust
/// URL内のプレースホルダーを展開
///
/// # Arguments
/// * `url` - プレースホルダーを含むURL
/// * `config` - 展開に使用するAppConfig
///
/// # Placeholders
/// - `{version}` → metadata.version
/// - `{arch}` → 現在のアーキテクチャ
///
/// # Example
/// ```
/// let url = "https://example.com/app-{version}-{arch}.AppImage";
/// let expanded = expand_url(url, &config)?;
/// // => "https://example.com/app-1.0.0-x86_64.AppImage"
/// ```
pub fn expand_url(url: &str, config: &AppConfig) -> Result<String>;
```

### AppDatabase 構造体

```rust
/// アプリデータベースの管理
pub struct AppDatabase {
    /// DBディレクトリパス
    pub path: PathBuf,
    /// キャッシュされたアプリ設定
    cache: HashMap<String, AppConfig>,
}

impl AppDatabase {
    /// 新しいAppDatabaseインスタンスを作成
    pub fn new(path: PathBuf) -> Self;

    /// キャッシュを初期化（全ファイル読み込み）
    pub fn init(&mut self) -> Result<()>;

    /// アプリを名前で検索（キャッシュ利用）
    pub fn get(&self, name: &str) -> Option<&AppConfig>;

    /// 全アプリのリストを取得
    pub fn list(&self) -> Vec<&AppConfig>;

    /// 検索（部分一致）
    pub fn search(&self, query: &str) -> Vec<&AppConfig>;

    /// キャッシュをリロード
    pub fn reload(&mut self) -> Result<()>;
}
```

## 使用例

### 基本的な使用

```rust
use an::db::{AppDatabase, AppConfig};

fn main() -> Result<()> {
    // DBを初期化
    let mut db = AppDatabase::new(PathBuf::from("apps"));
    db.init()?;

    // アプリを検索
    if let Some(app) = db.get("firefox") {
        println!("Name: {}", app.app.name);
        println!("URL: {}", app.source.url);
        println!("Type: {:?}", app.source.source_type);
    }

    // 全アプリをリスト
    for app in db.list() {
        println!("{}: {}", app.app.name, app.app.description);
    }

    // 部分一致検索
    let results = db.search("fire");
    println!("Found {} apps matching 'fire'", results.len());

    Ok(())
}
```

### TOMLファイルの直接読み込み

```rust
use an::db::app::{load, validate};

fn main() -> Result<()> {
    let config = load(Path::new("apps/firefox.toml"))?;

    // バリデーション
    validate(&config)?;

    // URL展開
    let url = expand_url(&config.source.url, &config)?;

    println!("Download URL: {}", url);
    Ok(())
}
```

## エラー

| エラー | 説明 |
|--------|------|
| `AnError::IoError` | ファイル読み込みエラー |
| `AnError::TomlError` | TOMLパースエラー |
| `AnError::ValidationError` | バリデーションエラー |

## 関連ドキュメント

- [アプリDBスキーマ仕様](../spec/app-db-schema.md)
- [アーキテクチャ](../design/architecture.md)
