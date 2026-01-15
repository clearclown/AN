# Changelog

AN (安装) の変更履歴です。

このプロジェクトは [Semantic Versioning](https://semver.org/lang/ja/) に従います。

## [Unreleased]

## [0.1.1] - 2025-01-14

### Added

- **Flatpakref サポート**: `.flatpakref` ファイルからのインストールに対応
  - `an install app.flatpakref` で Flatpak アプリをインストール可能に
  - インストール後、自動でコマンドエイリアスを作成

### Fixed

- Neovim AppImage URLを更新 (v0.11+ 対応)
- `.flatpakref` インストール時にエイリアスが作成されない問題を修正

### Changed

- crates.io パッケージ名を `an` から `an-installer` に変更（バイナリ名は `an` のまま）
- TLS実装をOpenSSLからrustlsに変更（クロスコンパイル対応）

## [0.1.0] - 2025-01-13

### Added

- **コマンド**
  - `an install <target>` - ローカルファイル(.deb/.AppImage)またはリモートアプリをインストール
  - `an remove <name>` - アプリを削除（AppImage/Flatpak/Deb対応）
  - `an link` - Flatpakアプリのエイリアス（シンボリックリンク）を作成
  - `an update` - AN本体とアプリDBを更新
  - `an list` - インストール済みアプリの一覧を表示
  - `an search [query]` - アプリDBを検索
  - `an info <name>` - アプリの詳細情報を表示
  - `an sync` - アプリDBをGitHubから同期

- **ハンドラ**
  - AppImageハンドラ: ファイル配置、シンボリックリンク作成、デスクトップエントリ生成
  - Flatpakハンドラ: アプリスキャン、ラッパースクリプト生成、アンインストール
  - Debハンドラ: dpkgによるインストール/削除
  - リモートダウンロードハンドラ: URLからファイルを取得

- **アプリDB**
  - TOML形式のアプリ定義ファイル（30アプリ収録）
  - カテゴリ: ブラウザ、開発ツール、メディア、セキュリティ、ゲーム、オフィス、ユーティリティ

- **その他**
  - カラー出力対応（colored）
  - エラーハンドリング（thiserror/anyhow）
  - ユニットテスト（63テスト）
  - 統合テスト（41テスト）
  - GitHub Actions CI/CD（テスト、lint、ビルド、リリース）
  - インストールスクリプト（install.sh）

### Architecture

```
src/
├── cli.rs           # CLIパーサー (clap)
├── commands/        # コマンド実装
│   ├── install.rs
│   ├── remove.rs
│   ├── link.rs
│   ├── update.rs
│   ├── list.rs
│   ├── search.rs
│   ├── info.rs
│   └── sync.rs
├── handlers/        # パッケージ形式別ハンドラ
│   ├── appimage.rs
│   ├── flatpak.rs
│   ├── deb.rs
│   └── remote.rs
├── db/              # アプリDB層
│   └── app.rs
├── errors.rs        # エラー型定義
└── utils/           # ユーティリティ
    ├── fs.rs
    └── ui.rs
```

### Known Issues

- `an update`でのバイナリ置き換えは、実行中のバイナリを上書きするため、環境によっては失敗する可能性があります
- Flatpakアプリの自動検出は、アプリIDの末尾部分で照合するため、同名のアプリがある場合は誤検出の可能性があります

### Dependencies

- clap 4.x - CLIフレームワーク
- reqwest 0.12.x - HTTPクライアント
- serde 1.x - シリアライゼーション
- toml 0.8.x - TOML パーサー
- colored 2.x - カラー出力
- semver 1.x - バージョン比較
- anyhow 1.x - エラーハンドリング
- thiserror 1.x - エラー型定義
- dirs 5.x - ディレクトリパス取得
- which 6.x - コマンド検索

---

[Unreleased]: https://github.com/clearclown/AN/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/clearclown/AN/releases/tag/v0.1.0
