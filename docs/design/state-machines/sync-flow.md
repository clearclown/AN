# sync コマンド ステートマシン図

## 概要

`an sync` コマンドの状態遷移を定義します。

## 状態一覧

| 状態 | 説明 |
|------|------|
| Start | 初期状態 |
| PrepareDir | DBディレクトリ準備 |
| GitClone | git sparse-checkout実行 |
| CopyFiles | ファイルコピー |
| Fallback | curlフォールバック |
| ShowSummary | 結果表示 |
| Success | 成功終了 |
| Error | エラー終了 |

## 状態遷移図

```
                     ┌───────────┐
                     │   Start   │
                     └─────┬─────┘
                           │
                     ┌─────▼─────┐
                     │  Prepare  │
                     │    Dir    │
                     └─────┬─────┘
                           │
              ┌────────────┴────────────┐
              │                         │
        ┌─────▼─────┐           ┌───────▼───────┐
        │  Exists   │           │  Not Exists   │
        └─────┬─────┘           └───────┬───────┘
              │                         │
              │                   ┌─────▼─────┐
              │                   │  Create   │
              │                   │    Dir    │
              │                   └─────┬─────┘
              │                         │
              └────────────┬────────────┘
                           │
                     ┌─────▼─────┐
                     │  Prepare  │
                     │  TempDir  │
                     └─────┬─────┘
                           │
                     ┌─────▼─────┐
                     │    Git    │
                     │   Clone   │
                     └─────┬─────┘
                           │
              ┌────────────┼────────────┐
              │            │            │
        ┌─────▼─────┐┌─────▼─────┐┌─────▼─────┐
        │  Success  ││  Failed   ││  Git Not  │
        │           ││           ││ Available │
        └─────┬─────┘└─────┬─────┘└─────┬─────┘
              │            │            │
              │            └────────────┘
              │                   │
        ┌─────▼─────┐       ┌─────▼─────┐
        │  Sparse   │       │ Fallback  │
        │ Checkout  │       │   Curl    │
        └─────┬─────┘       └─────┬─────┘
              │                   │
        ┌─────▼─────┐       ┌─────▼─────┐
        │   Copy    │       │ Download  │
        │   Files   │       │   Files   │
        └─────┬─────┘       └─────┬─────┘
              │                   │
              └─────────┬─────────┘
                        │
                  ┌─────▼─────┐
                  │  Cleanup  │
                  │  TempDir  │
                  └─────┬─────┘
                        │
                  ┌─────▼─────┐
                  │   Show    │
                  │  Summary  │
                  └─────┬─────┘
                        │
                  ┌─────▼─────┐
                  │  Success  │
                  └───────────┘
```

## 遷移条件

### Prepare Dir → Exists
- 条件: DBディレクトリが存在する

### Prepare Dir → Not Exists
- 条件: DBディレクトリが存在しない

### Git Clone → Success
- 条件: `git clone` が正常終了

### Git Clone → Failed
- 条件: `git clone` がエラーを返す

### Git Clone → Git Not Available
- 条件: gitコマンドが見つからない

## 各状態の処理

### Prepare Dir

```rust
fn prepare_dir() -> PathBuf {
    let db_dir = db_dir();
    if !db_dir.exists() {
        fs::create_dir_all(&db_dir)?;
    }
    db_dir
}
```

### Git Clone (Sparse Checkout)

```bash
git clone --depth=1 --filter=blob:none --sparse <REPO_URL> <TEMP_DIR>
git sparse-checkout set apps
```

```rust
fn git_clone(temp_dir: &Path) -> Result<()> {
    Command::new("git")
        .args([
            "clone",
            "--depth=1",
            "--filter=blob:none",
            "--sparse",
            REPO_URL,
            temp_dir.to_str().unwrap(),
        ])
        .output()?;

    Command::new("git")
        .args(["sparse-checkout", "set", "apps"])
        .current_dir(temp_dir)
        .output()?;

    Ok(())
}
```

### Copy Files

```rust
fn copy_files(source: &Path, dest: &Path) -> (usize, usize) {
    let mut added = 0;
    let mut updated = 0;

    for entry in fs::read_dir(source)? {
        let path = entry.path();
        if path.extension() == Some("toml") {
            let dest_path = dest.join(path.file_name().unwrap());

            if dest_path.exists() {
                // 内容比較して更新
                if fs::read_to_string(&path)? != fs::read_to_string(&dest_path)? {
                    fs::copy(&path, &dest_path)?;
                    updated += 1;
                }
            } else {
                // 新規コピー
                fs::copy(&path, &dest_path)?;
                added += 1;
            }
        }
    }

    (added, updated)
}
```

### Fallback Curl

```rust
fn sync_via_curl(db_dir: &Path) -> Result<()> {
    let apps = ["firefox", "brave", "gimp", ...];
    let base_url = "https://raw.githubusercontent.com/clearclown/AN/main/apps";

    for app in apps {
        let url = format!("{}/{}.toml", base_url, app);
        Command::new("curl")
            .args(["-sf", "-o", dest.to_str().unwrap(), &url])
            .output();
    }

    Ok(())
}
```

## 同期方式の比較

| 方式 | メリット | デメリット |
|------|----------|------------|
| git sparse-checkout | 全ファイル取得、差分検出 | gitが必要 |
| curl | git不要 | 既知ファイルのみ |

## 関連ドキュメント

- [sync コマンド仕様](../../spec/sync.md)
- [update フロー](./update-flow.md)
- [アプリDBスキーマ](../../spec/app-db-schema.md)
