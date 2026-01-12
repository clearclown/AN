# list コマンド ステートマシン図

## 概要

`an list` コマンドの状態遷移を定義します。

## 状態一覧

| 状態 | 説明 |
|------|------|
| Start | 初期状態 |
| ScanAppImage | AppImage一覧取得 |
| ScanFlatpak | Flatpak一覧取得 |
| ShowDeb | Deb案内表示 |
| ShowSummary | 合計表示 |
| Success | 成功終了 |

## 状態遷移図

```
                     ┌───────────┐
                     │   Start   │
                     └─────┬─────┘
                           │
                     ┌─────▼─────┐
                     │   Print   │
                     │  Header   │
                     └─────┬─────┘
                           │
                     ┌─────▼─────┐
                     │   Scan    │
                     │ AppImage  │
                     └─────┬─────┘
                           │
              ┌────────────┼────────────┐
              │            │            │
        ┌─────▼─────┐┌─────▼─────┐┌─────▼─────┐
        │  Found    ││  Empty    ││  Error    │
        │   Apps    ││           ││           │
        └─────┬─────┘└─────┬─────┘└─────┬─────┘
              │            │            │
        ┌─────▼─────┐┌─────▼─────┐┌─────▼─────┐
        │  Print    ││  Print    ││  Print    │
        │   List    ││  (なし)   ││  Warning  │
        └─────┬─────┘└─────┬─────┘└─────┬─────┘
              │            │            │
              └────────────┴────────────┘
                           │
                     ┌─────▼─────┐
                     │   Scan    │
                     │  Flatpak  │
                     └─────┬─────┘
                           │
              ┌────────────┼────────────┐
              │            │            │
        ┌─────▼─────┐┌─────▼─────┐┌─────▼─────┐
        │  Found    ││  Not      ││  Error    │
        │   Apps    ││ Installed ││           │
        └─────┬─────┘└─────┬─────┘└─────┬─────┘
              │            │            │
        ┌─────▼─────┐┌─────▼─────┐┌─────▼─────┐
        │  Print    ││  Print    ││  Print    │
        │   List    ││  Message  ││  Warning  │
        └─────┬─────┘└─────┬─────┘└─────┬─────┘
              │            │            │
              └────────────┴────────────┘
                           │
                     ┌─────▼─────┐
                     │  ShowDeb  │
                     │  Message  │
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

### Start → Print Header
- 常に遷移

### Scan AppImage → Found Apps
- 条件: `appimage::list_installed()` が1件以上のアプリを返す

### Scan AppImage → Empty
- 条件: リストが空

### Scan AppImage → Error
- 条件: エラー発生

### Scan Flatpak → Found Apps
- 条件: Flatpakインストール済みかつアプリあり

### Scan Flatpak → Not Installed
- 条件: `flatpak::is_installed()` が false

### Scan Flatpak → Error
- 条件: エラー発生

## 各状態の処理

### Scan AppImage

```rust
fn scan_appimage() -> Result<Vec<String>> {
    appimage::list_installed()
}
```

### Scan Flatpak

```rust
fn scan_flatpak() -> Result<Vec<FlatpakApp>> {
    if !flatpak::is_installed() {
        return Err(...);
    }
    flatpak::scan_apps()
}
```

### Show Summary

```rust
fn show_summary(total: usize) {
    println!("✓ 合計: {} アプリ", total);
}
```

## 特記事項

- このコマンドは常に成功終了する
- 個別のスキャンエラーは警告表示のみで処理続行
- Debパッケージは案内メッセージのみ（ANで直接管理しないため）

## 関連ドキュメント

- [list コマンド仕様](../../spec/list.md)
- [install フロー](./install-flow.md)
