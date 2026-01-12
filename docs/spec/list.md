# list コマンド仕様

## 概要

インストール済みアプリの一覧を表示します。AppImage、Flatpak、Debの各形式に対応しています。

## シグネチャ

```bash
an list
an ls
```

## ユースケース

### UC1: インストール済みアプリの確認

```bash
# 全インストール済みアプリを表示
an list
```

### UC2: 短縮コマンド

```bash
# エイリアスで同じ結果
an ls
```

## 入力

なし（引数・オプションなし）

## 出力

### 成功時

```
インストール済みアプリ一覧:

=== AppImage ===
  firefox (/home/user/Applications/Firefox.AppImage)
  obsidian (/home/user/Applications/Obsidian.AppImage)

=== Flatpak ===
  GIMP (org.gimp.GIMP)
  Telegram (org.telegram.desktop)

=== Deb (apt経由で管理) ===
  `dpkg -l | grep <パッケージ名>` で確認してください

✓ 合計: 4 アプリ
```

### AppImageなし時

```
=== AppImage ===
  (なし)
```

### Flatpakなし時

```
=== Flatpak ===
  (Flatpakがインストールされていません)
```

## 処理フロー

→ [ステートマシン図](../design/state-machines/list-flow.md) 参照

### 処理手順

1. AppImage一覧取得
   - `~/.local/bin/` 内のシンボリックリンクをスキャン
   - `~/Applications/` 内の.AppImageファイルを対応付け
2. Flatpak一覧取得
   - `flatpak list --app` コマンドで取得
   - アプリIDと表示名を抽出
3. Deb案内表示
   - dpkgで管理されているためコマンド案内のみ
4. 合計数を表示

## 内部API

### InstalledApp 構造体

```rust
pub struct InstalledApp {
    pub name: String,
    pub source: AppSource,
    pub path: Option<String>,
}
```

### AppSource 列挙型

```rust
pub enum AppSource {
    AppImage,
    Flatpak,
    Deb,
}
```

### is_installed 関数

特定のアプリがインストールされているか確認します。

```rust
pub fn is_installed(name: &str) -> Option<InstalledApp>
```

## エラーケース

| コード | 説明 | 対処 |
|--------|------|------|
| - | AppImage一覧取得エラー | 警告表示のみ、処理続行 |
| - | Flatpak一覧取得エラー | 警告表示のみ、処理続行 |

※ このコマンドは常に成功終了します。

## テストケース

### ユニットテスト

| ID | テスト内容 | 入力 | 期待結果 |
|----|-----------|------|----------|
| TC001 | AppSource表示 | AppSource::AppImage | "AppImage" |
| TC002 | AppSource表示 | AppSource::Flatpak | "Flatpak" |
| TC003 | AppSource表示 | AppSource::Deb | "Deb" |

### 統合テスト

| ID | テスト内容 | 前提条件 | 期待結果 |
|----|-----------|----------|----------|
| IT001 | 空の状態で実行 | 何もインストールなし | "(なし)"表示 |
| IT002 | AppImageあり | 1件インストール済み | リスト表示 |
| IT003 | Flatpakなし | Flatpak未インストール | 案内表示 |

## 関連ドキュメント

- [ステートマシン図](../design/state-machines/list-flow.md)
- [install コマンド仕様](./install.md)
- [remove コマンド仕様](./remove.md)
