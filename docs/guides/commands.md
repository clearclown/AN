# コマンドリファレンス

ANの全コマンドとオプションの一覧です。

## コマンド一覧

| コマンド | エイリアス | 説明 |
|----------|----------|------|
| `install` | `i` | アプリをインストール |
| `remove` | `rm`, `uninstall` | アプリを削除 |
| `link` | `l` | Flatpakエイリアスを生成 |
| `update` | - | ANとDBを更新 |

## グローバルオプション

```bash
an [OPTIONS] <COMMAND>
```

| オプション | 説明 |
|-----------|------|
| `-h, --help` | ヘルプを表示 |
| `-V, --version` | バージョンを表示 |
| `-v, --verbose` | 詳細出力モード |
| `-q, --quiet` | 静音モード |

---

## install

アプリをインストールします。

### 構文

```bash
an install <TARGET>
an i <TARGET>
```

### 引数

| 引数 | 説明 |
|------|------|
| `TARGET` | ファイルパスまたはアプリ名 |

### 動作

| TARGET | 動作 |
|--------|------|
| `*.deb` | dpkgでインストール、依存関係解決 |
| `*.AppImage` | ~/Applications/に配置、リンク作成 |
| アプリ名 | DBから検索、URL確認後ダウンロード |

### 例

```bash
# ローカル.debをインストール
an install ~/Downloads/vscode.deb

# ローカルAppImageをインストール
an install ~/Downloads/Obsidian.AppImage

# リモートアプリをインストール
an install firefox
```

### 出力例

```
Installing firefox...
Source: https://github.com/.../Firefox.AppImage
Continue? [y/N]: y

Downloading... [████████████████████████████████] 100%
Moving to ~/Applications/...
Creating symlink in ~/.local/bin/...

✓ firefox installed successfully
```

---

## remove

アプリを完全に削除します（パージ）。

### 構文

```bash
an remove <TARGET>
an rm <TARGET>
an uninstall <TARGET>
```

### 引数

| 引数 | 説明 |
|------|------|
| `TARGET` | 削除するアプリ名 |

### 動作

ANは自動的にアプリの形式を検出し、適切な方法で削除します:

| 形式 | 削除内容 |
|------|----------|
| AppImage | シンボリックリンク、実体ファイル、デスクトップエントリ |
| Deb | パッケージ、設定ファイル、不要な依存関係 |
| Flatpak | アプリ、ユーザーデータ |

### 例

```bash
# アプリを削除
an remove firefox
an rm vscode
an uninstall obsidian
```

### 出力例

```
Detecting installation type for 'firefox'...
Found: AppImage

Removing firefox...
  Removing symlink ~/.local/bin/firefox...
  Removing file ~/Applications/Firefox.AppImage...

✓ firefox removed successfully
```

---

## link

Flatpakアプリにエイリアス（短いコマンド名）を生成します。

### 構文

```bash
an link
an l
```

### 動作

1. インストール済みFlatpakアプリをスキャン
2. アプリ名を正規化（小文字化、接尾辞除去）
3. `~/.local/bin/` にラッパースクリプトを生成

### 例

```bash
an link
```

### 出力例

```
Scanning Flatpak applications...

Created links:
  gimp         → org.gimp.GIMP
  firefox      → org.mozilla.firefox
  spotify      → com.spotify.Client

Skipped (already exists):
  code         → /usr/bin/code (system binary)

Summary: 3 links created, 1 skipped
```

### 生成されるラッパー

```bash
#!/bin/bash
# AN-generated wrapper for org.gimp.GIMP
exec flatpak run org.gimp.GIMP "$@"
```

---

## update

AN本体とアプリデータベースを更新します。

### 構文

```bash
an update
```

### 動作

1. GitHub Releasesから最新バージョンを確認
2. 新バージョンがあればダウンロード・インストール
3. アプリDBを最新に更新

### 例

```bash
an update
```

### 出力例

```
Checking for updates...

AN Update:
  Current version: 0.1.0
  Latest version:  0.2.0

Downloading AN v0.2.0...  [████████████████████████████████] 100%
Installing...

App Database:
  Updated: 15 new apps, 3 updated

✓ AN has been updated to v0.2.0
  Please restart your terminal to use the new version.
```

---

## 終了コード

| コード | 説明 |
|--------|------|
| 0 | 成功 |
| 1 | 一般エラー |
| 2 | 引数エラー |
| 3 | ファイルエラー |
| 4 | ネットワークエラー |
| 5 | 権限エラー |

---

## 関連ドキュメント

- [クイックスタート](./getting-started.md)
- [機能仕様書](../spec/README.md)
