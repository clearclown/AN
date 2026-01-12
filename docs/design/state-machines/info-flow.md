# info コマンド ステートマシン図

## 概要

`an info <NAME>` コマンドの状態遷移を定義します。

## 状態一覧

| 状態 | 説明 |
|------|------|
| Start | 初期状態 |
| ParseInput | 入力解析 |
| SearchDB | DB検索 |
| ShowDetails | 詳細表示 |
| Success | 成功終了 |
| NotFound | 見つからない |

## 状態遷移図

```
                     ┌───────────┐
                     │   Start   │
                     └─────┬─────┘
                           │
                     ┌─────▼─────┐
                     │  Parse    │
                     │  Input    │
                     └─────┬─────┘
                           │
                     ┌─────▼─────┐
                     │  Search   │
                     │    DB     │
                     └─────┬─────┘
                           │
              ┌────────────┴────────────┐
              │                         │
        ┌─────▼─────┐           ┌───────▼───────┐
        │   Found   │           │   Not Found   │
        │           │           │               │
        └─────┬─────┘           └───────┬───────┘
              │                         │
        ┌─────▼─────┐           ┌───────▼───────┐
        │   Show    │           │    Show       │
        │  Details  │           │   Warning     │
        └─────┬─────┘           └───────┬───────┘
              │                         │
              │                         │
        ┌─────▼─────┐           ┌───────▼───────┐
        │  Success  │           │    Success    │
        └───────────┘           └───────────────┘
```

## 遷移条件

### Search DB → Found
- 条件: `db::find_by_name(name)` が `Some(config)` を返す

### Search DB → Not Found
- 条件: `db::find_by_name(name)` が `None` を返す

## 各状態の処理

### Parse Input

```rust
fn parse_input(name: &str) -> &str {
    name.trim()
}
```

### Search DB

```rust
fn search_db(name: &str) -> Result<Option<AppConfig>> {
    db::find_by_name(name)
}
```

### Show Details

```rust
fn show_details(config: &AppConfig) {
    println!("=== {} ===", config.app.name);
    println!("説明: {}", config.app.description);
    println!("タイプ: {:?}", config.source.source_type);

    // タイプに応じてURL/FlatpakID表示
    if let Some(ref flatpak_id) = config.source.flatpak_id {
        println!("Flatpak ID: {}", flatpak_id);
    }
    if !config.source.url.is_empty() {
        println!("URL: {}", config.source.url);
    }

    println!("アーキテクチャ: {}", config.source.architecture.join(", "));

    if let Some(ref homepage) = config.app.homepage {
        println!("ホームページ: {}", homepage);
    }

    if let Some(ref metadata) = config.metadata {
        // バージョン、カテゴリ表示
    }
}
```

## 表示項目の詳細

### 基本情報（常に表示）
- 名前
- 説明
- タイプ（AppImage/Deb/Flatpak）
- アーキテクチャ

### タイプ依存情報
| タイプ | 表示項目 |
|--------|----------|
| AppImage | URL |
| Deb | URL |
| Flatpak | Flatpak ID |

### オプション情報（存在時のみ）
- ホームページ
- バージョン
- カテゴリ

## 関連ドキュメント

- [info コマンド仕様](../../spec/info.md)
- [search フロー](./search-flow.md)
- [アプリDBスキーマ](../../spec/app-db-schema.md)
