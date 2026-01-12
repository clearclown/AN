# クイックスタート

ANを使い始めるためのガイドです。

## 前提条件

- Debian/Ubuntu系Linux (Linux Mint, Pop!_OS など)
- curl または wget
- sudo権限（.debインストール時）

## ANのインストール

```bash
# 方法1: ワンライナーインストール
curl -fsSL https://raw.githubusercontent.com/clearclown/AN/main/install.sh | bash

# 方法2: 手動インストール
wget https://github.com/clearclown/AN/releases/latest/download/an-linux-x86_64
chmod +x an-linux-x86_64
sudo mv an-linux-x86_64 /usr/local/bin/an
```

## インストール確認

```bash
an --version
# AN 0.1.0

an --help
# 使用可能なコマンド一覧が表示される
```

## 基本的な使い方

### アプリのインストール

```bash
# リモートアプリをインストール
an install firefox

# ローカルの.debファイルをインストール
an install ~/Downloads/vscode.deb

# ローカルのAppImageをインストール
an install ~/Downloads/Obsidian.AppImage
```

### アプリの削除

```bash
# アプリを完全に削除（設定ファイル含む）
an remove firefox

# または
an rm firefox
an uninstall firefox
```

### Flatpakエイリアスの生成

```bash
# 全Flatpakアプリにエイリアスを生成
an link

# これにより、以下のように実行可能に:
# flatpak run org.gimp.GIMP → gimp
```

### ANの更新

```bash
# AN本体とアプリDBを更新
an update
```

## PATHの設定

ANはシンボリックリンクを `~/.local/bin/` に作成します。
このディレクトリがPATHに含まれていることを確認してください。

### 確認方法

```bash
echo $PATH | grep -q "$HOME/.local/bin" && echo "OK" || echo "要設定"
```

### 設定方法

```bash
# ~/.bashrc または ~/.zshrc に追加
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

## 次のステップ

- [コマンドリファレンス](./commands.md) で全コマンドを確認
- [ANのインストール詳細](./installation.md) で詳細なインストール方法を確認
