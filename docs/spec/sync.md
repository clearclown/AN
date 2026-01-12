# sync コマンド仕様

## 概要

GitHubリポジトリからアプリデータベースを同期・更新します。

## シグネチャ

```bash
an sync
```

## ユースケース

### UC1: DBの初回取得

```bash
# 初めてDBを取得
an sync
```

### UC2: DBの更新

```bash
# 新しいアプリ定義を取得
an sync
```

## 入力

なし（引数・オプションなし）

## 出力

### 成功時（git使用）

```
アプリDBを同期中...

DBディレクトリを作成: ~/.config/an/apps
GitHubからデータを取得中...
✓ 同期完了: 16 件追加, 0 件更新

現在のアプリDB: 30 件
```

### 成功時（curlフォールバック）

```
アプリDBを同期中...

GitHubからデータを取得中...
git clone失敗: ...
代替方法でダウンロードを試行中...
✓ ダウンロード完了

現在のアプリDB: 14 件
```

### エラー時

```
アプリDBを同期中...

gitが利用できません: ...
代替方法でダウンロードを試行中...
```

## 処理フロー

→ [ステートマシン図](../design/state-machines/sync-flow.md) 参照

### 主要処理（git sparse-checkout）

1. DBディレクトリ確認・作成
   - `$AN_DB_DIR` または `~/.config/an/apps/`
2. 一時ディレクトリ準備
   - `/tmp/an-sync/` をクリア
3. git sparse-checkoutでクローン
   ```bash
   git clone --depth=1 --filter=blob:none --sparse <REPO_URL>
   git sparse-checkout set apps
   ```
4. TOMLファイルをコピー
   - 既存: 内容比較→更新
   - 新規: コピー
5. 一時ディレクトリを削除
6. アプリ数を表示

### フォールバック処理（curl）

gitが利用できない場合:

1. 既知のアプリ一覧を使用
2. 各アプリのTOMLを直接ダウンロード
   ```bash
   curl -sf -o <dest> <URL>
   ```
3. 失敗したファイルはスキップ

## 設定

### 同期先ディレクトリ

優先順位:
1. `$AN_DB_DIR` 環境変数
2. `~/.config/an/apps/`
3. `./apps/`

### GitHubリポジトリ

```
https://github.com/clearclown/AN.git
```

Raw URL（curlフォールバック用）:
```
https://raw.githubusercontent.com/clearclown/AN/main/apps/
```

## 内部API

### run 関数

```rust
pub fn run() -> Result<()>
```

### sync_via_curl 関数

```rust
fn sync_via_curl(db_dir: &PathBuf) -> Result<()>
```

### count_apps 関数

```rust
fn count_apps(db_dir: &PathBuf) -> usize
```

## エラーケース

| コード | 説明 | 対処 |
|--------|------|------|
| - | gitコマンドなし | curlフォールバック |
| - | git clone失敗 | curlフォールバック |
| - | ネットワークエラー | エラーメッセージ表示 |
| - | 書き込み権限なし | エラー終了 |

## テストケース

### ユニットテスト

| ID | テスト内容 | 入力 | 期待結果 |
|----|-----------|------|----------|
| TC001 | DBディレクトリパス | デフォルト | "apps"を含む |
| TC002 | アプリ数カウント | 存在しないパス | 0 |

### 統合テスト

| ID | テスト内容 | 前提条件 | 期待結果 |
|----|-----------|----------|----------|
| IT001 | 初回同期 | DBなし | ファイル作成 |
| IT002 | 更新同期 | 既存DBあり | 差分更新 |
| IT003 | gitなし | git未インストール | curlで動作 |

## セキュリティ考慮事項

1. **HTTPS通信**: 全ての通信はHTTPS経由
2. **一時ファイル**: `/tmp/`に作成、処理後削除
3. **上書き確認**: 既存ファイルは内容比較後に更新

## 関連ドキュメント

- [ステートマシン図](../design/state-machines/sync-flow.md)
- [update コマンド仕様](./update.md)
- [アプリDBスキーマ](./app-db-schema.md)
