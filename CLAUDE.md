# CLAUDE.md - AN Project Guide

このファイルはClaude Codeがプロジェクトを理解するためのガイドです。

## プロジェクト概要

**AN (安装)** は、Debian/Ubuntu系Linux向けの統合型パッケージマネージャーです。

- **言語**: Rust
- **ターゲット**: Debian/Ubuntu系Linux (Linux Mint, Pop!_OS等)
- **コアコンセプト**: "Unified & Clean"

### 解決する課題

1. パッケージ管理の断片化（.deb, AppImage, Flatpakがバラバラ）
2. AppImageの手動管理の手間
3. 不完全な削除（設定ファイル、依存関係の残存）

## ディレクトリ構造

```
AN/
├── Cargo.toml           # Rust依存関係
├── README.md            # プロジェクト要件・仕様
├── CLAUDE.md            # このファイル
├── LICENSE
│
├── src/                 # Rustソースコード
│   ├── main.rs          # エントリーポイント
│   ├── lib.rs           # ライブラリエントリ
│   ├── cli.rs           # CLIパーサー (clap)
│   ├── errors.rs        # エラー型定義
│   ├── commands/        # サブコマンド実装
│   │   ├── install.rs   # an install
│   │   ├── remove.rs    # an remove
│   │   ├── link.rs      # an link
│   │   └── update.rs    # an update
│   ├── handlers/        # パッケージ形式ハンドラ
│   │   ├── deb.rs       # .deb処理
│   │   ├── appimage.rs  # AppImage処理
│   │   ├── flatpak.rs   # Flatpak処理
│   │   └── remote.rs    # ダウンロード処理
│   ├── db/              # アプリDB層
│   │   └── app.rs       # TOML設定構造体
│   └── utils/           # ユーティリティ
│       ├── fs.rs        # ファイル操作
│       └── ui.rs        # カラー出力
│
├── apps/                # アプリDB (TOML)
│   └── *.toml
│
├── docs/                # ドキュメント
│   ├── spec/            # 機能仕様書
│   ├── design/          # 設計ドキュメント
│   │   └── state-machines/  # ステートマシン図
│   ├── api/             # 内部API仕様
│   └── guides/          # ユーザーガイド
│
├── tests/               # テスト
│   └── fixtures/
│
└── _legacy/             # AM参考コード（参照のみ）
    ├── APP-MANAGER
    ├── modules/
    └── programs/
```

## 開発ガイドライン

### 開発方針

- **Spec-First**: 仕様書を先に作成・更新してから実装
- **TDD**: テストを先に書いてから実装（Red→Green→Refactor）
- **ステートマシン駆動**: 状態遷移を`docs/design/state-machines/`に文書化

### ビルド・テスト

```bash
# ビルドチェック
cargo check

# テスト実行
cargo test

# リリースビルド
cargo build --release

# Lint
cargo clippy

# フォーマット
cargo fmt
```

### コードスタイル

- `rustfmt` デフォルト設定に従う
- エラーは `thiserror` + `anyhow` で処理
- UI出力は `src/utils/ui.rs` の関数を使用

### エラーコード体系

| 範囲 | カテゴリ |
|------|----------|
| E001-E099 | インストール関連 |
| E101-E199 | 削除関連 |
| E201-E299 | リンク関連 |
| E301-E399 | アップデート関連 |
| E901-E999 | 一般/システム |

## 主要コマンド

| コマンド | エイリアス | 説明 |
|----------|----------|------|
| `an install <target>` | `i` | アプリインストール |
| `an remove <target>` | `rm`, `uninstall` | アプリ削除（パージ） |
| `an link` | `l` | Flatpakエイリアス生成 |
| `an update` | - | AN・DB更新 |

## アプリDB形式 (TOML)

```toml
[app]
name = "firefox"
description = "Mozilla Firefox Web Browser"
homepage = "https://www.mozilla.org/firefox/"

[source]
type = "appimage"  # appimage | deb | flatpak | script
url = "https://..."
architecture = ["x86_64"]

[metadata]
categories = ["Network", "WebBrowser"]
desktop_entry = true
version = "1.0.0"
```

## 注意事項

### _legacy/ ディレクトリ

`_legacy/` はAMプロジェクトの参考コードです。
- **読み取り専用**として扱う
- 新機能実装時のロジック参照用
- 直接編集・実行しない

### 重要なパス

| 用途 | パス |
|------|------|
| AppImage格納 | `~/Applications/` |
| シンボリックリンク | `~/.local/bin/` |
| 設定 | `~/.config/an/` |
| アプリDB | `./apps/` または `~/.config/an/apps/` |

## 参照ドキュメント

- **仕様書**: `docs/spec/` - 各コマンドの詳細仕様
- **設計**: `docs/design/` - アーキテクチャ、ステートマシン図
- **API**: `docs/api/` - 内部モジュールのAPI
- **ガイド**: `docs/guides/` - ユーザー向けガイド

## 貢献時のチェックリスト

- [ ] `cargo test` が全パス
- [ ] `cargo clippy` で警告なし
- [ ] `cargo fmt` 適用済み
- [ ] 仕様書/ステートマシン図を更新（機能変更時）
- [ ] テストケースを追加（新機能時）
