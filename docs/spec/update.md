# update コマンド仕様

## 概要

AN本体のアップデートとアプリデータベースの更新を行います。

## シグネチャ

```bash
an update
```

## ユースケース

### UC1: AN本体とDBの更新

```bash
an update
# AN本体の最新版確認・更新
# アプリDBの更新
```

## 入力

このコマンドは引数を取りません。

## 出力

### AN更新あり

```
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
Checking for updates...

AN: Already up to date (v0.2.0)

App Database:
  Fetching latest database...
  Updated: 2 new apps

✓ Database updated successfully
```

### ネットワークエラー

```
Error: E301: アップデート情報の取得に失敗しました

Hint: ネットワーク接続を確認してください
```

## 処理フロー

→ [ステートマシン図](../design/state-machines/update-flow.md) 参照

### AN本体の更新

1. GitHub Releases APIで最新バージョン確認
2. 現在のバージョンと比較
3. 新バージョンがあればダウンロード
4. 現在のバイナリをバックアップ
5. 新バージョンをインストール
6. 実行権限を設定

### アプリDBの更新

1. DBリポジトリから最新を取得
2. ローカルDBを更新
3. 追加・更新されたアプリ数を表示

## バージョン管理

### バージョン形式

Semantic Versioning (SemVer) を使用:

```
MAJOR.MINOR.PATCH
例: 0.1.0, 1.0.0, 1.2.3
```

### バージョン比較

```rust
// semver クレートを使用
use semver::Version;

let current = Version::parse("0.1.0")?;
let latest = Version::parse("0.2.0")?;

if latest > current {
    // 更新が必要
}
```

## エラーケース

| コード | 説明 | 対処 |
|--------|------|------|
| E301 | アップデート情報取得失敗 | ネットワーク接続確認 |
| E302 | ダウンロード失敗 | 再試行、URL確認 |
| E303 | インストール失敗 | 権限確認、ディスク容量確認 |

## テストケース

### ユニットテスト

| ID | テスト内容 | 入力 | 期待結果 |
|----|-----------|------|----------|
| TC301 | バージョン比較 (更新あり) | "0.1.0", "0.2.0" | true |
| TC302 | バージョン比較 (最新) | "0.2.0", "0.2.0" | false |
| TC303 | バージョン比較 (ダウングレード) | "0.3.0", "0.2.0" | false |
| TC304 | バージョンパース | "v0.1.0" | "0.1.0" |
| TC305 | API レスポンスパース | JSON | バージョン文字列 |

### 統合テスト

| ID | テスト内容 | 前提条件 | 期待結果 |
|----|-----------|----------|----------|
| IT301 | 更新確認 (更新あり) | 古いバージョン | 新バージョン情報表示 |
| IT302 | 更新確認 (最新) | 最新バージョン | "up to date" メッセージ |
| IT303 | DB更新 | ネットワーク接続あり | DB更新成功 |
| IT304 | ネットワークエラー | オフライン | E301エラー |

## セキュリティ考慮事項

### バイナリ検証 (将来実装)

1. **チェックサム検証**: ダウンロードしたバイナリのSHA256検証
2. **署名検証**: GPG署名の検証

### バックアップ

更新前に現在のバイナリをバックアップ:

```
/usr/local/bin/an → /usr/local/bin/an.bak
```

ロールバックが必要な場合:

```bash
sudo mv /usr/local/bin/an.bak /usr/local/bin/an
```

## 関連ドキュメント

- [ステートマシン図](../design/state-machines/update-flow.md)
- [エラーハンドリング](../design/error-handling.md)
