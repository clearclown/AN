# Handler層 API仕様

## 概要

Handler層は各パッケージ形式の処理を担当します。

## モジュール構成

```
src/handlers/
├── mod.rs       # 共通trait、エクスポート
├── deb.rs       # Debパッケージ処理
├── appimage.rs  # AppImage処理
├── flatpak.rs   # Flatpak処理
└── remote.rs    # リモートダウンロード処理
```

## 共通trait

### InstallHandler

```rust
pub trait InstallHandler {
    /// インストール処理を実行
    fn install(&self, source: &Path) -> Result<InstallResult>;

    /// インストール可能か確認
    fn can_install(&self, path: &Path) -> bool;
}
```

### RemoveHandler

```rust
pub trait RemoveHandler {
    /// 削除処理を実行
    fn remove(&self, target: &str) -> Result<RemoveResult>;

    /// 検出結果を返す
    fn detect(&self, target: &str) -> Option<DetectionInfo>;
}
```

## deb.rs

### 構造体

```rust
pub struct DebHandler;
```

### メソッド

#### install

```rust
impl DebHandler {
    /// .debファイルをインストール
    ///
    /// # Arguments
    /// * `path` - .debファイルのパス
    ///
    /// # Returns
    /// * `Ok(InstallResult)` - インストール成功
    /// * `Err(AnError)` - インストール失敗
    ///
    /// # Example
    /// ```
    /// let handler = DebHandler;
    /// handler.install(Path::new("/tmp/vscode.deb"))?;
    /// ```
    pub fn install(&self, path: &Path) -> Result<InstallResult>;
}
```

#### remove

```rust
impl DebHandler {
    /// Debパッケージを削除（パージ）
    ///
    /// # Arguments
    /// * `package` - パッケージ名
    ///
    /// # Example
    /// ```
    /// let handler = DebHandler;
    /// handler.remove("code")?;
    /// ```
    pub fn remove(&self, package: &str) -> Result<RemoveResult>;
}
```

#### detect

```rust
impl DebHandler {
    /// dpkgデータベースでパッケージを検索
    ///
    /// # Arguments
    /// * `name` - 検索するパッケージ名
    ///
    /// # Returns
    /// * `Some(DetectionInfo)` - パッケージが見つかった
    /// * `None` - パッケージが見つからない
    pub fn detect(&self, name: &str) -> Option<DetectionInfo>;
}
```

## appimage.rs

### 構造体

```rust
pub struct AppImageHandler {
    /// AppImage格納ディレクトリ (default: ~/Applications)
    pub apps_dir: PathBuf,
    /// シンボリックリンク配置先 (default: ~/.local/bin)
    pub bin_dir: PathBuf,
}
```

### メソッド

#### install

```rust
impl AppImageHandler {
    /// AppImageをインストール
    ///
    /// 1. apps_dirにファイルを移動
    /// 2. 実行権限を付与
    /// 3. bin_dirにシンボリックリンクを作成
    ///
    /// # Arguments
    /// * `path` - AppImageファイルのパス
    /// * `name` - コマンド名（リンク名）
    pub fn install(&self, path: &Path, name: &str) -> Result<InstallResult>;
}
```

#### remove

```rust
impl AppImageHandler {
    /// AppImageを削除
    ///
    /// 1. シンボリックリンクを削除
    /// 2. AppImage実体を削除
    /// 3. デスクトップエントリを削除（存在すれば）
    pub fn remove(&self, name: &str) -> Result<RemoveResult>;
}
```

#### detect

```rust
impl AppImageHandler {
    /// AppImageリンクを検出
    ///
    /// bin_dir内のシンボリックリンクを確認し、
    /// apps_dirを指しているものを検出
    pub fn detect(&self, name: &str) -> Option<DetectionInfo>;
}
```

## flatpak.rs

### 構造体

```rust
pub struct FlatpakHandler;
```

### メソッド

#### scan_apps

```rust
impl FlatpakHandler {
    /// インストール済みFlatpakアプリをスキャン
    ///
    /// # Returns
    /// * アプリIDと表示名のリスト
    pub fn scan_apps(&self) -> Result<Vec<FlatpakApp>>;
}
```

#### create_wrapper

```rust
impl FlatpakHandler {
    /// ラッパースクリプトを生成
    ///
    /// # Arguments
    /// * `app_id` - Flatpak アプリID (例: org.gimp.GIMP)
    /// * `name` - 生成するコマンド名 (例: gimp)
    pub fn create_wrapper(&self, app_id: &str, name: &str) -> Result<()>;
}
```

#### remove

```rust
impl FlatpakHandler {
    /// Flatpakアプリを削除
    ///
    /// `--delete-data` オプション付きで削除
    pub fn remove(&self, app_id: &str) -> Result<RemoveResult>;
}
```

#### detect

```rust
impl FlatpakHandler {
    /// Flatpakアプリを検出
    ///
    /// アプリ名またはIDで検索
    pub fn detect(&self, name: &str) -> Option<DetectionInfo>;
}
```

## remote.rs

### 構造体

```rust
pub struct RemoteHandler {
    /// ダウンロード先一時ディレクトリ
    pub temp_dir: PathBuf,
}
```

### メソッド

#### download

```rust
impl RemoteHandler {
    /// URLからファイルをダウンロード
    ///
    /// # Arguments
    /// * `url` - ダウンロードURL
    /// * `filename` - 保存ファイル名
    ///
    /// # Returns
    /// * ダウンロードしたファイルのパス
    pub fn download(&self, url: &str, filename: &str) -> Result<PathBuf>;
}
```

#### download_with_progress

```rust
impl RemoteHandler {
    /// プログレスバー付きダウンロード
    ///
    /// # Arguments
    /// * `url` - ダウンロードURL
    /// * `filename` - 保存ファイル名
    /// * `callback` - 進捗コールバック (downloaded, total)
    pub fn download_with_progress<F>(
        &self,
        url: &str,
        filename: &str,
        callback: F,
    ) -> Result<PathBuf>
    where
        F: Fn(u64, u64);
}
```

## 共通型定義

### InstallResult

```rust
pub struct InstallResult {
    /// インストールしたアプリ名
    pub name: String,
    /// インストール先パス
    pub path: PathBuf,
    /// 作成したリンク（AppImageの場合）
    pub link: Option<PathBuf>,
}
```

### RemoveResult

```rust
pub struct RemoveResult {
    /// 削除したアプリ名
    pub name: String,
    /// 削除したファイル一覧
    pub removed_files: Vec<PathBuf>,
}
```

### DetectionInfo

```rust
pub struct DetectionInfo {
    /// 検出されたアプリ名
    pub name: String,
    /// インストール形式
    pub install_type: InstallType,
    /// 追加情報（パスやIDなど）
    pub details: HashMap<String, String>,
}

pub enum InstallType {
    Deb,
    AppImage,
    Flatpak,
}
```

## 関連ドキュメント

- [アーキテクチャ](../design/architecture.md)
- [エラーハンドリング](../design/error-handling.md)
