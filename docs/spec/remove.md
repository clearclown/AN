# remove コマンド仕様

## 概要

インストールされたアプリを完全に削除します（パージ処理）。
アプリの形式を自動判別し、適切な削除方法を実行します。

## シグネチャ

```bash
an remove <target>
an rm <target>
an uninstall <target>
```

## ユースケース

### UC1: AppImageの削除

```bash
an remove obsidian
# ~/.local/bin/obsidian (リンク) と ~/Applications/Obsidian.AppImage を削除
```

### UC2: Debパッケージの削除

```bash
an remove vscode
# apt purge + autoremove で完全削除
```

### UC3: Flatpakアプリの削除

```bash
an remove gimp
# flatpak uninstall --delete-data で設定データも削除
```

## 入力

| パラメータ | 型 | 必須 | 説明 |
|-----------|-----|------|------|
| target | String | Yes | アプリ名 |

## 出力

### 成功時

```
Detecting installation type for 'firefox'...
Found: Flatpak (org.mozilla.firefox)

Removing firefox...
  Uninstalling Flatpak app...
  Removing user data...

✓ firefox removed successfully
```

### 失敗時

```
Error: E101: アプリが見つかりません: unknownapp

検索対象:
  - AppImage links (~/.local/bin/)
  - Deb packages (dpkg)
  - Flatpak apps (flatpak list)
```

## 処理フロー

→ [ステートマシン図](../design/state-machines/remove-flow.md) 参照

### 検出優先順位

1. **AppImageリンク**: `~/.local/bin/<target>` のシンボリックリンク
2. **Debパッケージ**: `dpkg -l | grep <target>`
3. **Flatpakアプリ**: `flatpak list | grep <target>`
4. **AM管理下**: レガシー対応

### AppImage削除処理

1. シンボリックリンク削除 (`~/.local/bin/<name>`)
2. AppImage実体削除 (`~/Applications/<name>.AppImage`)
3. デスクトップエントリ削除（存在すれば）

### Deb削除処理

1. `sudo apt purge -y <package>` (設定ファイルも削除)
2. `sudo apt autoremove -y` (不要な依存関係削除)

### Flatpak削除処理

1. `flatpak uninstall --delete-data -y <app_id>`
   - `--delete-data`: ユーザーデータも削除

## エラーケース

| コード | 説明 | 対処 |
|--------|------|------|
| E101 | アプリが見つからない | アプリ名を確認、`an list` で一覧表示 |
| E102 | apt purgeに失敗 | パッケージ名確認、依存関係確認 |
| E103 | flatpak uninstallに失敗 | アプリID確認、Flatpakの状態確認 |

## テストケース

### ユニットテスト

| ID | テスト内容 | 入力 | 期待結果 |
|----|-----------|------|----------|
| TC101 | AppImageリンク検出 | 存在するリンク | DetectionResult::AppImage |
| TC102 | Debパッケージ検出 | インストール済みパッケージ | DetectionResult::Deb |
| TC103 | Flatpak検出 | インストール済みアプリ | DetectionResult::Flatpak |
| TC104 | 検出失敗 | 未インストールアプリ | DetectionResult::NotFound |
| TC105 | リンク先パス取得 | シンボリックリンク | 正しいパス |

### 統合テスト

| ID | テスト内容 | 前提条件 | 期待結果 |
|----|-----------|----------|----------|
| IT101 | AppImage削除 | テストAppImageインストール済み | リンクと実体が削除される |
| IT102 | Deb削除 | テストDebインストール済み | パッケージと設定が削除される |
| IT103 | Flatpak削除 | テストFlatpakインストール済み | アプリとデータが削除される |
| IT104 | 未インストールアプリ | アプリ未インストール | E101エラー |

## 削除の完全性

ANの削除は「パージ」を基本とし、以下を全て削除します：

| 形式 | 削除対象 |
|------|----------|
| AppImage | 実体ファイル、シンボリックリンク、デスクトップエントリ |
| Deb | パッケージ、設定ファイル、不要な依存関係 |
| Flatpak | アプリ、ユーザーデータ |

## 関連ドキュメント

- [ステートマシン図](../design/state-machines/remove-flow.md)
- [エラーハンドリング](../design/error-handling.md)
