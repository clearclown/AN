# search コマンド ステートマシン図

## 概要

`an search [QUERY]` コマンドの状態遷移を定義します。

## 状態一覧

| 状態 | 説明 |
|------|------|
| Start | 初期状態 |
| ParseQuery | クエリ解析 |
| LoadDB | DB読み込み |
| Search | 検索実行 |
| ListAll | 全件表示 |
| ShowResults | 結果表示 |
| Success | 成功終了 |
| Error | エラー終了 |

## 状態遷移図

```
                     ┌───────────┐
                     │   Start   │
                     └─────┬─────┘
                           │
                     ┌─────▼─────┐
                     │  Parse    │
                     │  Query    │
                     └─────┬─────┘
                           │
              ┌────────────┴────────────┐
              │                         │
        ┌─────▼─────┐           ┌───────▼───────┐
        │  Query    │           │   No Query    │
        │ Provided  │           │   (List All)  │
        └─────┬─────┘           └───────┬───────┘
              │                         │
              │                         │
              └────────────┬────────────┘
                           │
                     ┌─────▼─────┐
                     │  Load     │
                     │   DB      │
                     └─────┬─────┘
                           │
              ┌────────────┼────────────┐
              │            │            │
        ┌─────▼─────┐┌─────▼─────┐┌─────▼─────┐
        │    OK     ││   Empty   ││   Error   │
        │           ││    DB     ││           │
        └─────┬─────┘└─────┬─────┘└─────┬─────┘
              │            │            │
              │      ┌─────▼─────┐      │
              │      │   Show    │      │
              │      │  Warning  │      │
              │      └─────┬─────┘      │
              │            │            │
              │            ▼            │
              │         Success         │
              │                         │
              └───────────┬─────────────┘
                          │
               ┌──────────┴──────────┐
               │                     │
         ┌─────▼─────┐       ┌───────▼───────┐
         │  Search   │       │   List All    │
         │  Filter   │       │   GroupBy     │
         └─────┬─────┘       └───────┬───────┘
               │                     │
         ┌─────┴─────┐               │
         │           │               │
   ┌─────▼─────┐┌────▼────┐          │
   │  Found    ││  Not    │          │
   │  Results  ││  Found  │          │
   └─────┬─────┘└────┬────┘          │
         │           │               │
   ┌─────▼─────┐┌────▼────┐    ┌─────▼─────┐
   │   Show    ││  Show   │    │   Show    │
   │  Results  ││ Warning │    │ Grouped   │
   └─────┬─────┘└────┬────┘    └─────┬─────┘
         │           │               │
         └───────────┴───────────────┘
                     │
               ┌─────▼─────┐
               │  Success  │
               └───────────┘
```

## 遷移条件

### Parse Query → Query Provided
- 条件: `query.len() > 0`

### Parse Query → No Query
- 条件: クエリが空文字または未指定

### Load DB → OK
- 条件: TOMLファイルが正常にパースできた

### Load DB → Empty DB
- 条件: `apps.is_empty()`

### Load DB → Error
- 条件: ファイル読み込みエラー

### Search Filter → Found Results
- 条件: 1件以上のマッチ

### Search Filter → Not Found
- 条件: マッチなし

## 各状態の処理

### Load DB

```rust
fn load_db() -> Result<Vec<AppConfig>> {
    let db_dir = get_db_dir();
    db::app::load_all(&db_dir)
}
```

### Search Filter

```rust
fn search_filter(apps: &[AppConfig], query: &str) -> Vec<&AppConfig> {
    let query_lower = query.to_lowercase();
    apps.iter()
        .filter(|app| {
            app.app.name.to_lowercase().contains(&query_lower)
                || app.app.description.to_lowercase().contains(&query_lower)
        })
        .collect()
}
```

### List All GroupBy

```rust
fn list_all_grouped(apps: &[AppConfig]) {
    // AppImage, Deb, Flatpakでグループ化
    // 各グループを名前順ソート
    // グループごとに表示
}
```

## 検索ロジック

- 大文字小文字を区別しない
- 部分一致検索
- 検索対象:
  - `app.name`
  - `app.description`

## 関連ドキュメント

- [search コマンド仕様](../../spec/search.md)
- [info フロー](./info-flow.md)
