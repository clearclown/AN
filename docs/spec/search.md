# search コマンド仕様

## 概要

アプリデータベースを検索し、利用可能なアプリの情報を表示します。

## シグネチャ

```bash
an search [QUERY]
an s [QUERY]
```

## ユースケース

### UC1: キーワード検索

```bash
# "browser"を含むアプリを検索
an search browser
```

### UC2: 全アプリ一覧

```bash
# クエリなしで全アプリ表示
an search
```

### UC3: 短縮コマンド

```bash
an s edit
```

## 入力

| パラメータ | 型 | 必須 | 説明 |
|-----------|-----|------|------|
| QUERY | String | No | 検索キーワード（省略時は全件表示） |

### 検索対象

- アプリ名（name）
- 説明文（description）

検索は大文字小文字を区別しません。

## 出力

### 検索結果あり（クエリ指定時）

```
「browser」を検索中...

3 件のアプリが見つかりました:

  firefox - Mozilla Firefox Web Browser
    タイプ: AppImage
    ホームページ: https://www.mozilla.org/firefox/

  brave - Brave Browser - Secure, Fast & Private Web Browser
    タイプ: AppImage
    ホームページ: https://brave.com/

  chromium - Chromium Web Browser
    タイプ: Deb
```

### 全件表示（クエリなし時）

```
利用可能なアプリ (30 件):

=== AppImage (21) ===
  audacity - Audacity - Free Audio Editor and Recorder
  brave - Brave Browser
  ...

=== Deb (2) ===
  code - Visual Studio Code
  discord - Discord

=== Flatpak (7) ===
  blender - Blender 3D Creation Suite
  ...
```

### 検索結果なし

```
「browser」を検索中...

「browser」に一致するアプリが見つかりません
```

### DBが空の場合

```
アプリDBが空です
```

## 処理フロー

→ [ステートマシン図](../design/state-machines/search-flow.md) 参照

### 検索処理手順

1. DBディレクトリを取得
   - `$AN_DB_DIR` 環境変数
   - `~/.config/an/apps/`
   - `./apps/`
2. 全TOML ファイルを読み込み
3. クエリで部分一致検索
4. 結果をフォーマットして表示

### 全件表示処理手順

1. 全アプリをロード
2. タイプ別に分類（AppImage/Deb/Flatpak）
3. 各グループを名前順でソート
4. グループごとに表示

## 内部API

### run 関数

```rust
pub fn run(query: &str) -> Result<()>
```

### list_all 関数

```rust
pub fn list_all() -> Result<()>
```

### show_details 関数

```rust
pub fn show_details(name: &str) -> Result<()>
```

## エラーケース

| コード | 説明 | 対処 |
|--------|------|------|
| - | DBディレクトリが存在しない | 警告表示 |
| - | TOMLパースエラー | 該当ファイルをスキップ |

## テストケース

### ユニットテスト

| ID | テスト内容 | 入力 | 期待結果 |
|----|-----------|------|----------|
| TC001 | DBディレクトリ取得 | デフォルト | "apps"を含むパス |
| TC002 | 大文字小文字無視 | "FIREFOX" | firefoxがヒット |

### 統合テスト

| ID | テスト内容 | 前提条件 | 期待結果 |
|----|-----------|----------|----------|
| IT001 | 存在するアプリ検索 | firefoxがDB内 | 1件以上ヒット |
| IT002 | 存在しないアプリ検索 | - | "見つかりません" |
| IT003 | 全件表示 | DBに複数アプリ | タイプ別表示 |
| IT004 | 空DB | DB空 | "DBが空です" |

## 関連ドキュメント

- [ステートマシン図](../design/state-machines/search-flow.md)
- [info コマンド仕様](./info.md)
- [アプリDBスキーマ](./app-db-schema.md)
