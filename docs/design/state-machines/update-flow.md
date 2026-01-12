# update コマンド ステートマシン図

## 概要

`an update` コマンドの状態遷移を定義します。
AN自体のアップデートとアプリDBの更新を行います。

## 状態一覧

| 状態 | 説明 |
|------|------|
| Start | 初期状態 |
| CheckNetwork | ネットワーク接続確認 |
| CheckAnVersion | AN最新バージョン確認 |
| CompareVersion | バージョン比較 |
| DownloadAn | AN最新版ダウンロード |
| InstallAn | AN更新インストール |
| UpdateDb | アプリDB更新 |
| Summary | 結果サマリー表示 |
| Success | 成功終了 |
| Error | エラー終了 |
| UpToDate | 最新版（更新不要） |

## 状態遷移図

```
                         ┌───────────┐
                         │   Start   │
                         └─────┬─────┘
                               │
                         ┌─────▼─────┐
                         │CheckNetwork│
                         └─────┬─────┘
                               │
                   ┌───────────┴───────────┐
                   │                       │
              [connected]             [no network]
                   │                       │
             ┌─────▼─────┐           ┌─────▼─────┐
             │CheckAn    │           │   Error   │
             │Version    │           │  (E301)   │
             └─────┬─────┘           └───────────┘
                   │
             ┌─────▼─────┐
             │ Compare   │
             │ Version   │
             └─────┬─────┘
                   │
         ┌─────────┴─────────┐
         │                   │
    [new version]       [up to date]
         │                   │
   ┌─────▼─────┐       ┌─────▼─────┐
   │DownloadAn │       │  UpdateDb │◄────────┐
   └─────┬─────┘       └─────┬─────┘         │
         │                   │               │
   ┌─────▼─────┐             │               │
   │ InstallAn │             │               │
   └─────┬─────┘             │               │
         │                   │               │
         └───────────────────┤               │
                             │               │
                       ┌─────▼─────┐         │
                       │  Summary  │         │
                       └─────┬─────┘         │
                             │               │
                   ┌─────────┴─────────┐     │
                   │                   │     │
              [an updated]        [db only]  │
                   │                   │     │
             ┌─────▼─────┐       ┌─────▼─────┐
             │  Success  │       │  Success  │
             │ (restart  │       │           │
             │  needed)  │       │           │
             └───────────┘       └───────────┘
```

## バージョン確認

### CheckAnVersion

```rust
fn check_latest_version() -> Result<String> {
    // GitHub Releases APIを使用
    let url = "https://api.github.com/repos/clearclown/AN/releases/latest";

    let response: serde_json::Value = reqwest::blocking::get(url)?
        .json()?;

    let version = response["tag_name"]
        .as_str()
        .ok_or(AnError::UpdateCheckFailed)?
        .trim_start_matches('v');

    Ok(version.to_string())
}
```

### CompareVersion

```rust
use semver::Version;

fn should_update(current: &str, latest: &str) -> bool {
    let current = Version::parse(current).unwrap_or_else(|_| Version::new(0, 0, 0));
    let latest = Version::parse(latest).unwrap_or_else(|_| Version::new(0, 0, 0));

    latest > current
}
```

## アップデート処理

### DownloadAn

```rust
fn download_an(version: &str) -> Result<PathBuf> {
    let url = format!(
        "https://github.com/clearclown/AN/releases/download/v{}/an-linux-x86_64",
        version
    );

    let temp_path = std::env::temp_dir().join("an-update");

    let mut response = reqwest::blocking::get(&url)?;
    let mut file = std::fs::File::create(&temp_path)?;
    std::io::copy(&mut response, &mut file)?;

    Ok(temp_path)
}
```

### InstallAn

```rust
fn install_an(downloaded_path: &Path) -> Result<()> {
    let install_path = which::which("an")?;

    // バックアップ作成
    let backup_path = install_path.with_extension("bak");
    std::fs::copy(&install_path, &backup_path)?;

    // 新バージョンをインストール
    std::fs::copy(downloaded_path, &install_path)?;

    // 実行権限付与
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = std::fs::metadata(&install_path)?.permissions();
        perms.set_mode(0o755);
        std::fs::set_permissions(&install_path, perms)?;
    }

    Ok(())
}
```

## アプリDB更新

### UpdateDb

```rust
fn update_db() -> Result<UpdateResult> {
    let db_dir = dirs::config_dir()
        .unwrap()
        .join("an")
        .join("apps");

    // 方法1: Git pull (DBがgitリポジトリの場合)
    if db_dir.join(".git").exists() {
        Command::new("git")
            .args(["-C", db_dir.to_str().unwrap(), "pull"])
            .status()?;
    }
    // 方法2: APIから最新を取得
    else {
        fetch_db_from_api(&db_dir)?;
    }

    Ok(UpdateResult::DbUpdated)
}
```

## 出力例

### AN更新あり

```
$ an update

Checking for updates...

AN Update:
  Current version: 0.1.0
  Latest version:  0.2.0

Downloading AN v0.2.0...  [████████████████████████████████] 100%
Installing...

App Database:
  Fetching latest database...
  Updated: 15 new apps, 3 updated

✓ AN has been updated to v0.2.0
  Please restart your terminal to use the new version.
```

### 最新版の場合

```
$ an update

Checking for updates...

AN: Already up to date (v0.2.0)

App Database:
  Fetching latest database...
  Updated: 2 new apps

✓ Database updated successfully
```

## 関連ドキュメント

- [update コマンド仕様](../../spec/update.md)
- [エラーハンドリング設計](../error-handling.md)
