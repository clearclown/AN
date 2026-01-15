# AN (安装) - Unified Package Manager for Linux

[![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

**AN** (アン) は、Debian/Ubuntu系Linux向けの統合型パッケージマネージャーです。

`.deb`、`AppImage`、`Flatpak` を単一のインターフェースで管理し、システムをクリーンに保ちます。

## 特徴

- **統一インターフェース**: あらゆる形式を `an install` / `an remove` で管理
- **完全削除**: 設定ファイル、依存関係、ユーザーデータまで完全にパージ
- **透明性**: リモートインストール時はURLを表示し、ユーザー確認を要求
- **Flatpakエイリアス**: `flatpak run org.gimp.GIMP` → `gimp` で起動可能に

## インストール

### ワンライナー（推奨）

```bash
curl -fsSL https://raw.githubusercontent.com/clearclown/AN/main/install.sh | bash
```

### Cargo (Rustユーザー向け)

```bash
# crates.io から
cargo install an-installer

# GitHubから直接
cargo install --git https://github.com/clearclown/AN
```

### Arch Linux (AUR)

```bash
# yay（安定版）
yay -S an-installer

# paru（安定版）
paru -S an-installer

# 開発版
yay -S an-installer-git
paru -S an-installer-git
```

### Nix

```bash
# Flakeを使用
nix profile install github:clearclown/AN

# 一時的に使用
nix run github:clearclown/AN -- --help
```

### Homebrew (Linux)

```bash
brew tap clearclown/an
brew install an
```

### apt (Debian/Ubuntu PPA)

```bash
# PPA追加（将来実装予定）
sudo add-apt-repository ppa:clearclown/an
sudo apt update
sudo apt install an-installer
```

### Snap

```bash
# Snap Store から（将来実装予定）
sudo snap install an-installer --classic
```

### 手動ダウンロード

[Releases](https://github.com/clearclown/AN/releases) からバイナリをダウンロード:

| アーキテクチャ | ファイル名 |
|---------------|-----------|
| x86_64 | `an-linux-x86_64` |
| aarch64 (ARM64) | `an-linux-aarch64` |

```bash
# 例: x86_64
wget https://github.com/clearclown/AN/releases/latest/download/an-linux-x86_64
chmod +x an-linux-x86_64
sudo mv an-linux-x86_64 /usr/local/bin/an
```

### ソースからビルド

```bash
git clone https://github.com/clearclown/AN
cd AN
cargo build --release
sudo cp target/release/an /usr/local/bin/
```

## 使い方

### アプリのインストール

```bash
# リモートアプリをインストール
an install firefox

# ローカル.debファイルをインストール
an install ~/Downloads/vscode.deb

# ローカルAppImageをインストール
an install ~/Downloads/Obsidian.AppImage
```

### アプリの削除

```bash
# 完全削除（設定ファイルも削除）
an remove firefox
an rm vscode
an uninstall obsidian
```

### Flatpakエイリアス生成

```bash
# 全Flatpakアプリにエイリアスを作成
an link
```

### 更新

```bash
# AN本体とアプリDBを更新
an update
```

### アプリの検索・情報表示

```bash
# インストール済みアプリ一覧
an list

# 利用可能なアプリを検索
an search browser
an search          # 全アプリ一覧

# アプリ詳細を表示
an info firefox

# アプリDBを最新に同期
an sync
```

## コマンド一覧

| コマンド | エイリアス | 説明 |
|----------|----------|------|
| `an install <target>` | `i` | アプリをインストール |
| `an remove <target>` | `rm`, `uninstall` | アプリを完全削除 |
| `an link` | `l` | Flatpakエイリアスを生成 |
| `an update` | - | AN本体・DBを更新 |
| `an list` | `ls` | インストール済みアプリ一覧 |
| `an search [query]` | `s` | アプリDBを検索 |
| `an info <name>` | - | アプリ詳細を表示 |
| `an sync` | - | アプリDBをGitHubから同期 |

## アーキテクチャ

```
┌───────────────────────────────────────────────────────────────┐
│                         CLI Layer                              │
│                    (clap derive macros)                        │
├───────────────────────────────────────────────────────────────┤
│                       Command Layer                            │
│  install │ remove │ link │ update │ list │ search │ info │ sync │
├───────────────────────────────────────────────────────────────┤
│                       Handler Layer                            │
│            deb │ appimage │ flatpak │ remote                   │
├───────────────────────────────────────────────────────────────┤
│                         DB Layer                               │
│                   TOML App Database                            │
└───────────────────────────────────────────────────────────────┘
```

## ディレクトリ構成

| 用途 | パス |
|------|------|
| 実行ファイル | `/usr/local/bin/an` または `~/.local/bin/an` |
| AppImage格納 | `~/Applications/` |
| シンボリックリンク | `~/.local/bin/` |
| 設定/キャッシュ | `~/.config/an/` |

## 開発

### 必要環境

- Rust 1.70+
- Linux (Debian/Ubuntu系)

### ビルド

```bash
# 開発ビルド
cargo build

# テスト
cargo test

# リリースビルド
cargo build --release
```

### プロジェクト構造

```
AN/
├── src/                 # Rustソースコード
│   ├── commands/        # サブコマンド実装
│   ├── handlers/        # パッケージ形式ハンドラ
│   ├── db/              # アプリDB層
│   └── utils/           # ユーティリティ
├── apps/                # アプリDB (TOML)
├── docs/                # ドキュメント
│   ├── spec/            # 機能仕様書
│   ├── design/          # 設計・ステートマシン図
│   └── guides/          # ユーザーガイド
└── tests/               # テスト
```

## ドキュメント

- [機能仕様書](docs/spec/README.md)
- [設計ドキュメント](docs/design/README.md)
- [ユーザーガイド](docs/guides/README.md)
- [CLAUDE.md](CLAUDE.md) - 開発者向けガイド

## ライセンス

MIT License - 詳細は [LICENSE](LICENSE) を参照

## 謝辞

このプロジェクトは [AM (App Manager)](https://github.com/ivan-hc/AM) にインスパイアされています。
