# link コマンド ステートマシン図

## 概要

`an link` コマンドの状態遷移を定義します。
Flatpakアプリをスキャンし、短い名前で実行できるエイリアスを生成します。

## 状態一覧

| 状態 | 説明 |
|------|------|
| Start | 初期状態 |
| CheckFlatpak | Flatpakの存在確認 |
| ScanApps | インストール済みアプリのスキャン |
| ProcessApps | 各アプリの処理ループ |
| NormalizeName | アプリ名の正規化 |
| CheckConflict | 名前衝突の確認 |
| CreateWrapper | ラッパースクリプト生成 |
| Summary | 結果サマリー表示 |
| Success | 成功終了 |
| Error | エラー終了 |
| NoApps | アプリなし終了 |

## 状態遷移図

```
                         ┌───────────┐
                         │   Start   │
                         └─────┬─────┘
                               │
                         ┌─────▼─────┐
                         │CheckFlatpak│
                         └─────┬─────┘
                               │
                   ┌───────────┴───────────┐
                   │                       │
             [installed]              [not found]
                   │                       │
             ┌─────▼─────┐           ┌─────▼─────┐
             │ ScanApps  │           │   Error   │
             └─────┬─────┘           │  (E201)   │
                   │                 └───────────┘
         ┌─────────┴─────────┐
         │                   │
    [apps found]        [no apps]
         │                   │
   ┌─────▼─────┐       ┌─────▼─────┐
   │ProcessApps│       │  NoApps   │
   └─────┬─────┘       │  (exit 0) │
         │             └───────────┘
         ▼
   ┌───────────────────────────────────┐
   │    For each app:                  │
   │                                   │
   │  ┌─────────────┐                  │
   │  │NormalizeName│                  │
   │  └──────┬──────┘                  │
   │         │                         │
   │  ┌──────▼──────┐                  │
   │  │CheckConflict│                  │
   │  └──────┬──────┘                  │
   │         │                         │
   │    ┌────┴────┐                    │
   │    │         │                    │
   │ [unique] [conflict]               │
   │    │         │                    │
   │    │    ┌────▼────┐               │
   │    │    │ AddSuffix│              │
   │    │    └────┬────┘               │
   │    │         │                    │
   │  ┌─▼─────────▼─┐                  │
   │  │CreateWrapper │                 │
   │  └──────┬──────┘                  │
   │         │                         │
   └─────────┼─────────────────────────┘
             │
       ┌─────▼─────┐
       │  Summary  │
       └─────┬─────┘
             │
       ┌─────▼─────┐
       │  Success  │
       └───────────┘
```

## 名前正規化ルール

### NormalizeName の処理

```rust
fn normalize_name(app_id: &str) -> String {
    // 例: "org.gimp.GIMP" → "gimp"
    // 例: "org.mozilla.firefox" → "firefox"
    // 例: "com.spotify.Client" → "spotify"

    let parts: Vec<&str> = app_id.split('.').collect();

    // 最後のパートを取得（通常これがアプリ名）
    let name = parts.last().unwrap_or(&app_id);

    // 小文字化
    let normalized = name.to_lowercase();

    // "client", "app" などの一般的な接尾辞を除去
    let normalized = normalized
        .trim_end_matches("client")
        .trim_end_matches("app")
        .trim_end_matches("-");

    normalized.to_string()
}
```

### 衝突解決ルール

1. **既存リンクと衝突**
   - 既にANが作成したリンクなら上書き
   - 他のプログラムと衝突なら、ID末尾を付加 (例: `gimp-org`)

2. **同一セッション内で衝突**
   - 先着優先、後続には連番付加 (例: `app`, `app-2`)

## ラッパースクリプト

### CreateWrapper で生成するスクリプト

```bash
#!/bin/bash
# AN-generated wrapper for org.gimp.GIMP
exec flatpak run org.gimp.GIMP "$@"
```

### スクリプト配置先

```
~/.local/bin/<normalized_name>
```

例:
- `~/.local/bin/gimp` → `flatpak run org.gimp.GIMP`
- `~/.local/bin/firefox` → `flatpak run org.mozilla.firefox`

## 各状態の処理

### CheckFlatpak
```rust
fn check_flatpak() -> Result<()> {
    let output = Command::new("flatpak")
        .arg("--version")
        .output()?;

    if !output.status.success() {
        return Err(AnError::FlatpakNotInstalled.into());
    }
    Ok(())
}
```

### ScanApps
```rust
fn scan_flatpak_apps() -> Result<Vec<FlatpakApp>> {
    let output = Command::new("flatpak")
        .args(["list", "--app", "--columns=application,name"])
        .output()?;

    // パースしてFlatpakApp構造体のベクタを返す
    parse_flatpak_output(&output.stdout)
}
```

### CreateWrapper
```rust
fn create_wrapper(app_id: &str, name: &str) -> Result<()> {
    let wrapper_path = dirs::home_dir()
        .unwrap()
        .join(".local/bin")
        .join(name);

    let script = format!(
        r#"#!/bin/bash
# AN-generated wrapper for {}
exec flatpak run {} "$@"
"#,
        app_id, app_id
    );

    std::fs::write(&wrapper_path, script)?;

    // 実行権限付与
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = std::fs::metadata(&wrapper_path)?.permissions();
        perms.set_mode(0o755);
        std::fs::set_permissions(&wrapper_path, perms)?;
    }

    Ok(())
}
```

## 出力例

```
$ an link

Scanning Flatpak applications...

Created links:
  gimp         → org.gimp.GIMP
  firefox      → org.mozilla.firefox
  spotify      → com.spotify.Client
  vscode       → com.visualstudio.code

Skipped (already exists):
  code         → (system binary)

Summary: 4 links created, 1 skipped
```

## 関連ドキュメント

- [link コマンド仕様](../../spec/link.md)
- [エラーハンドリング設計](../error-handling.md)
