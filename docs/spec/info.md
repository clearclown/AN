# info コマンド仕様

## 概要

指定したアプリの詳細情報を表示します。

## シグネチャ

```bash
an info <NAME>
```

## ユースケース

### UC1: アプリ詳細の確認

```bash
an info firefox
```

### UC2: Flatpakアプリの確認

```bash
an info telegram
```

## 入力

| パラメータ | 型 | 必須 | 説明 |
|-----------|-----|------|------|
| NAME | String | Yes | 調べたいアプリ名 |

## 出力

### 成功時（AppImage）

```
=== firefox ===
説明: Mozilla Firefox Web Browser
タイプ: AppImage
URL: https://github.com/.../Firefox_x86_64.AppImage
アーキテクチャ: x86_64
ホームページ: https://www.mozilla.org/firefox/
カテゴリ: Network, WebBrowser
```

### 成功時（Flatpak）

```
=== telegram ===
説明: Telegram Desktop - Fast and secure messaging
タイプ: Flatpak
Flatpak ID: org.telegram.desktop
アーキテクチャ: x86_64, aarch64
ホームページ: https://telegram.org/
カテゴリ: Network, InstantMessaging
```

### 成功時（Deb）

```
=== code ===
説明: Visual Studio Code - Code Editing. Redefined.
タイプ: Deb
URL: https://code.visualstudio.com/sha/download?build=stable&os=linux-deb-x64
アーキテクチャ: x86_64
ホームページ: https://code.visualstudio.com/
カテゴリ: Development, IDE
```

### アプリが見つからない場合

```
アプリ「unknownapp」が見つかりません
```

## 処理フロー

→ [ステートマシン図](../design/state-machines/info-flow.md) 参照

### 処理手順

1. アプリ名でDBを検索
2. 見つかった場合:
   - 基本情報（名前、説明、タイプ）を表示
   - タイプに応じてURL/FlatpakIDを表示
   - アーキテクチャ、ホームページ、カテゴリを表示
3. 見つからない場合:
   - 警告メッセージを表示

## 表示項目

| 項目 | 表示条件 | 説明 |
|------|----------|------|
| 名前 | 常に | アプリ名 |
| 説明 | 常に | アプリの説明 |
| タイプ | 常に | AppImage/Deb/Flatpak |
| URL | AppImage/Deb | ダウンロードURL |
| Flatpak ID | Flatpak | Flatpakアプリケーション識別子 |
| アーキテクチャ | 常に | 対応アーキテクチャ |
| ホームページ | 存在時 | 公式サイトURL |
| バージョン | 存在時 | 利用可能なバージョン |
| カテゴリ | 存在時 | アプリカテゴリ |

## 内部API

`search.rs` 内の `show_details` 関数を使用:

```rust
pub fn show_details(name: &str) -> Result<()>
```

## エラーケース

| コード | 説明 | 対処 |
|--------|------|------|
| E003 | アプリDBにエントリなし | `an search` で検索を提案 |

## テストケース

### ユニットテスト

| ID | テスト内容 | 入力 | 期待結果 |
|----|-----------|------|----------|
| TC001 | 存在するアプリ | "firefox" | 詳細表示 |
| TC002 | 存在しないアプリ | "unknown" | 警告メッセージ |

### 統合テスト

| ID | テスト内容 | 前提条件 | 期待結果 |
|----|-----------|----------|----------|
| IT001 | AppImage詳細 | firefoxがDB内 | URL表示 |
| IT002 | Flatpak詳細 | telegramがDB内 | FlatpakID表示 |
| IT003 | 存在しないアプリ | - | 警告表示 |

## 関連ドキュメント

- [ステートマシン図](../design/state-machines/info-flow.md)
- [search コマンド仕様](./search.md)
- [アプリDBスキーマ](./app-db-schema.md)
