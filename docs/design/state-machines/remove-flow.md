# remove コマンド ステートマシン図

## 概要

`an remove <target>` コマンドの状態遷移を定義します。

## 状態一覧

| 状態 | 説明 |
|------|------|
| Start | 初期状態 |
| ParseTarget | ターゲット解析 |
| DetectType | インストール形式の検出 |
| AppImageLink | AppImageリンク検出 |
| DebPackage | Debパッケージ検出 |
| FlatpakApp | Flatpakアプリ検出 |
| AmManaged | AM管理下アプリ検出 |
| NotFound | アプリ未検出 |
| RemoveSymlink | シンボリックリンク削除 |
| RemoveAppImage | AppImage実体削除 |
| AptPurge | apt purge実行 |
| AptAutoremove | apt autoremove実行 |
| FlatpakUninstall | flatpak uninstall実行 |
| Success | 成功終了 |
| Error | エラー終了 |

## 状態遷移図

```
                              ┌───────────┐
                              │   Start   │
                              └─────┬─────┘
                                    │
                              ┌─────▼─────┐
                              │ParseTarget│
                              └─────┬─────┘
                                    │
                              ┌─────▼─────┐
                              │DetectType │
                              └─────┬─────┘
                                    │
    ┌───────────┬───────────────────┼───────────────────┬───────────┐
    │           │                   │                   │           │
┌───▼───┐ ┌─────▼─────┐       ┌─────▼─────┐       ┌─────▼─────┐ ┌───▼───┐
│AppImage│ │DebPackage │       │FlatpakApp │       │ AmManaged │ │NotFound│
│ Link  │ │           │       │           │       │           │ │       │
└───┬───┘ └─────┬─────┘       └─────┬─────┘       └─────┬─────┘ └───┬───┘
    │           │                   │                   │           │
┌───▼───┐ ┌─────▼─────┐       ┌─────▼─────┐       ┌─────▼─────┐ ┌───▼───┐
│Remove │ │ AptPurge  │       │ Flatpak   │       │ am remove │ │ Error │
│Symlink│ │           │       │ Uninstall │       │           │ │(E101) │
└───┬───┘ └─────┬─────┘       │--delete-  │       └─────┬─────┘ └───────┘
    │           │             │   data    │             │
┌───▼───┐ ┌─────▼─────┐       └─────┬─────┘             │
│Remove │ │   Apt     │             │                   │
│AppImage│ │Autoremove│             │                   │
└───┬───┘ └─────┬─────┘             │                   │
    │           │                   │                   │
    └───────────┴───────────────────┴───────────────────┘
                              │
                        ┌─────▼─────┐
                        │  Success  │
                        └───────────┘
```

## 検出優先順位

1. **AppImageリンク** (`~/.local/bin/<target>`)
   - シンボリックリンクの存在を確認
   - リンク先が `~/Applications/` 内を指しているか確認

2. **Debパッケージ** (`dpkg -l | grep <target>`)
   - dpkgデータベースで検索

3. **Flatpakアプリ** (`flatpak list | grep <target>`)
   - アプリID or 名前で検索

4. **AM管理下** (`am list | grep <target>`)
   - AM/AppManデータベースで検索 (レガシー対応)

## 遷移条件

### DetectType の分岐ロジック

```rust
fn detect_type(target: &str) -> DetectionResult {
    // 1. AppImageリンクをチェック
    let link_path = dirs::home_dir()
        .unwrap()
        .join(".local/bin")
        .join(target);

    if link_path.is_symlink() {
        let dest = link_path.read_link()?;
        if dest.starts_with(dirs::home_dir().unwrap().join("Applications")) {
            return DetectionResult::AppImage { link_path, app_path: dest };
        }
    }

    // 2. Debパッケージをチェック
    let dpkg_output = Command::new("dpkg")
        .args(["-l", target])
        .output()?;
    if dpkg_output.status.success() {
        return DetectionResult::Deb { package: target.to_string() };
    }

    // 3. Flatpakをチェック
    let flatpak_output = Command::new("flatpak")
        .args(["list", "--app", "--columns=application"])
        .output()?;
    // ID検索ロジック...

    // 4. AM管理下をチェック (レガシー)
    // ...

    DetectionResult::NotFound
}
```

## 各状態の処理

### RemoveSymlink + RemoveAppImage
```rust
fn remove_appimage(link_path: &Path, app_path: &Path) -> Result<()> {
    // 1. シンボリックリンク削除
    std::fs::remove_file(link_path)?;

    // 2. AppImage実体削除
    std::fs::remove_file(app_path)?;

    // 3. デスクトップエントリ削除（存在すれば）
    let desktop_path = dirs::data_dir()
        .unwrap()
        .join("applications")
        .join(format!("{}.desktop", app_name));
    if desktop_path.exists() {
        std::fs::remove_file(desktop_path)?;
    }
}
```

### AptPurge + AptAutoremove
```rust
fn remove_deb(package: &str) -> Result<()> {
    // 1. apt purge (設定ファイルも削除)
    Command::new("sudo")
        .args(["apt", "purge", "-y", package])
        .status()?;

    // 2. apt autoremove (不要な依存削除)
    Command::new("sudo")
        .args(["apt", "autoremove", "-y"])
        .status()?;
}
```

### FlatpakUninstall
```rust
fn remove_flatpak(app_id: &str) -> Result<()> {
    // --delete-data で設定データも削除
    Command::new("flatpak")
        .args(["uninstall", "--delete-data", "-y", app_id])
        .status()?;
}
```

## 関連ドキュメント

- [remove コマンド仕様](../../spec/remove.md)
- [エラーハンドリング設計](../error-handling.md)
