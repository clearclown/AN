# コマンドリファレンス

ANの全コマンドとオプションの一覧です。

## コマンド一覧

| コマンド | エイリアス | 説明 |
|----------|----------|------|
| `install` | `i` | アプリをインストール |
| `remove` | `rm`, `uninstall` | アプリを削除 |
| `link` | `l` | Flatpakエイリアスを生成 |
| `update` | - | ANとDBを更新 |
| `list` | `ls` | インストール済みアプリ一覧 |
| `search` | `s` | アプリDBを検索 |
| `info` | - | アプリ詳細を表示 |
| `sync` | - | アプリDBをGitHubから同期 |

## グローバルオプション

```bash
an [OPTIONS] <COMMAND>
```

| オプション | 説明 |
|-----------|------|
| `-h, --help` | ヘルプを表示 |
| `-V, --version` | バージョンを表示 |

---

## install

アプリをインストールします。

### 構文

```bash
an install [OPTIONS] <TARGET>
an i [OPTIONS] <TARGET>
```

### 引数

| 引数 | 説明 |
|------|------|
| `TARGET` | ファイルパスまたはアプリ名 |

### オプション（AppImage用）

| オプション | 説明 |
|-----------|------|
| `-n, --name <NAME>` | カスタムコマンド名を指定 |
| `-d, --desktop` | デスクトップエントリを作成 |
| `-m, --move` | 元ファイルを削除（移動モード） |

### 動作

| TARGET | 動作 |
|--------|------|
| `*.deb` | dpkgでインストール、依存関係解決 |
| `*.AppImage` | ~/Applications/に配置、リンク作成 |
| アプリ名（AppImage/Deb） | DBから検索、URL確認後ダウンロード |
| アプリ名（Flatpak） | DBから検索、flatpak installで直接インストール |

### 例

```bash
# ローカル.debをインストール
an install ~/Downloads/vscode.deb

# ローカルAppImageをインストール
an install ~/Downloads/Obsidian.AppImage

# カスタム名でインストール
an install ~/Downloads/Firefox.AppImage -n ff

# デスクトップエントリ付きでインストール
an install ~/Downloads/app.AppImage -d

# 移動モードでインストール（元ファイル削除）
an install ~/Downloads/app.AppImage -m

# リモートアプリをインストール（AppImage/Deb）
an install firefox

# リモートFlatpakアプリをインストール
an install telegram
```

### 出力例

```
アプリを検索中: firefox
ソース: https://github.com/.../Firefox.AppImage
続行しますか? [y/N]: y

Downloading: https://...
Downloaded to: /tmp/an-downloads/firefox.AppImage
AppImageをインストール中: firefox
実行権限を付与
シンボリックリンク作成: ~/.local/bin/firefox
✓ AppImage 'firefox' をインストールしました
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

AppImageを削除中: firefox
シンボリックリンク削除: ~/.local/bin/firefox
ファイル削除: ~/Applications/Firefox.AppImage
✓ AppImageを削除しました
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

---

## list

インストール済みアプリの一覧を表示します。

### 構文

```bash
an list
an ls
```

### 動作

- AppImage: `~/.local/bin/` 内のシンボリックリンクをスキャン
- Flatpak: `flatpak list` で取得
- Deb: 案内メッセージを表示

### 例

```bash
an list
```

### 出力例

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

---

## search

アプリデータベースを検索します。

### 構文

```bash
an search [QUERY]
an s [QUERY]
```

### 引数

| 引数 | 説明 |
|------|------|
| `QUERY` | 検索クエリ（省略時は全件表示） |

### 動作

- クエリ指定: アプリ名・説明で部分一致検索
- クエリなし: 全アプリをタイプ別に一覧表示

### 例

```bash
# 全アプリ一覧
an search

# キーワード検索
an search browser
an s edit
```

### 出力例

```
利用可能なアプリ (14 件):

=== AppImage (9) ===
  firefox - Mozilla Firefox Web Browser
  brave - Brave Browser
  ...

=== Deb (2) ===
  code - Visual Studio Code
  discord - Discord

=== Flatpak (3) ===
  telegram - Telegram Desktop
  libreoffice - LibreOffice
  ...
```

---

## info

アプリの詳細情報を表示します。

### 構文

```bash
an info <NAME>
```

### 引数

| 引数 | 説明 |
|------|------|
| `NAME` | アプリ名 |

### 例

```bash
an info firefox
an info telegram
```

### 出力例

```
=== firefox ===
説明: Mozilla Firefox Web Browser
タイプ: AppImage
URL: https://github.com/.../Firefox_x86_64.AppImage
アーキテクチャ: x86_64
ホームページ: https://www.mozilla.org/firefox/
カテゴリ: Network, WebBrowser
```

```
=== telegram ===
説明: Telegram Desktop - Fast and secure messaging
タイプ: Flatpak
Flatpak ID: org.telegram.desktop
アーキテクチャ: x86_64, aarch64
ホームページ: https://telegram.org/
カテゴリ: Network, InstantMessaging
```

---

## sync

アプリデータベースをGitHubから同期します。

### 構文

```bash
an sync
```

### 動作

1. GitHubリポジトリからappsディレクトリを取得
2. `~/.config/an/apps/`にTOMLファイルをコピー
3. 新規・更新ファイル数を表示

### 例

```bash
an sync
```

### 出力例

```
アプリDBを同期中...

GitHubからデータを取得中...
✓ 同期完了: 16 件追加, 0 件更新

現在のアプリDB: 30 件
```

### 注意

- gitがインストールされていない場合、curlでフォールバック
- 同期先: `~/.config/an/apps/`

---

## 終了コード

| コード | 説明 |
|--------|------|
| 0 | 成功 |
| 1 | 一般エラー |

---

## エラーコード

| コード | 説明 |
|--------|------|
| E001 | ファイルが見つかりません |
| E002 | 不明なファイル形式 |
| E003 | アプリDBにエントリが見つかりません |
| E004 | ダウンロード失敗 |
| E005 | dpkg/権限エラー |
| E006 | flatpak installエラー |
| E101 | アプリが見つかりません |
| E102 | apt purgeエラー |
| E103 | flatpak uninstallエラー |
| E201 | Flatpakがインストールされていません |

---

## 関連ドキュメント

- [クイックスタート](./getting-started.md)
- [機能仕様書](../spec/README.md)
