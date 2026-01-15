# AN Packaging

このディレクトリには、様々なパッケージマネージャー向けのパッケージングファイルが含まれています。

## ディレクトリ構成

```
packaging/
├── aur/           # Arch Linux (AUR)
│   ├── PKGBUILD          # 安定版
│   ├── PKGBUILD-git      # 開発版
│   └── README.md
├── debian/        # Debian/Ubuntu (apt/PPA)
│   ├── control
│   ├── changelog
│   ├── rules
│   ├── copyright
│   └── README.md
├── homebrew/      # Homebrew (Linux)
│   ├── an.rb
│   └── README.md
└── README.md      # このファイル
```

## サポート状況

| パッケージマネージャー | 状態 | ドキュメント |
|-------------------|------|--------------|
| **crates.io** | ✅ 公開済み | [Cargo.toml](../Cargo.toml) |
| **GitHub Releases** | ✅ 自動化済み | [release.yml](../.github/workflows/release.yml) |
| **AUR** | 📝 準備完了 | [aur/README.md](aur/README.md) |
| **Nix** | ✅ 対応済み | [flake.nix](../flake.nix) |
| **Homebrew** | 📝 準備完了 | [homebrew/README.md](homebrew/README.md) |
| **apt/PPA** | 📝 準備完了 | [debian/README.md](debian/README.md) |
| **Snap** | 📝 準備完了 | [snap/README.md](../snap/README.md) |

## クイックスタート

### crates.io（既に公開済み）

```bash
cargo install an-installer
```

### GitHub Releases（既に自動化済み）

タグをプッシュするだけで自動的にリリースが作成されます：

```bash
git tag v0.1.2
git push origin v0.1.2
```

### AUR

```bash
# 1. AURにリポジトリを作成
git clone ssh://aur@aur.archlinux.org/an-installer.git
cd an-installer

# 2. PKGBUILDをコピー
cp /path/to/AN/packaging/aur/PKGBUILD .

# 3. .SRCINFOを生成
makepkg --printsrcinfo > .SRCINFO

# 4. プッシュ
git add PKGBUILD .SRCINFO
git commit -m "Initial commit: an-installer 0.1.1"
git push
```

詳細: [aur/README.md](aur/README.md)

### Nix（既に対応済み）

```bash
# flake.nixが既にリポジトリに含まれているため、ユーザーは直接使用可能
nix profile install github:clearclown/AN
nix run github:clearclown/AN -- --help
```

### Homebrew

```bash
# 1. homebrew-an リポジトリを作成
git clone https://github.com/clearclown/homebrew-an.git
cd homebrew-an

# 2. Formulaをコピー
mkdir -p Formula
cp /path/to/AN/packaging/homebrew/an.rb Formula/

# 3. プッシュ
git add Formula/an.rb
git commit -m "Add AN formula"
git push
```

詳細: [homebrew/README.md](homebrew/README.md)

### apt/PPA

```bash
# 1. Launchpadでアカウント作成とPPA作成

# 2. GPG鍵を設定
gpg --full-generate-key
gpg --keyserver keyserver.ubuntu.com --send-keys YOUR_KEY_ID

# 3. ソースパッケージをビルド
debuild -S -sa

# 4. PPAにアップロード
dput ppa:clearclown/an ../an-installer_*.changes
```

詳細: [debian/README.md](debian/README.md)

### Snap

```bash
# 1. Snapcraft にログイン
snapcraft login

# 2. 名前を登録
snapcraft register an-installer

# 3. ビルド
snapcraft

# 4. アップロード
snapcraft upload --release=stable an-installer_*.snap
```

詳細: [snap/README.md](../snap/README.md)

## リリースチェックリスト

新しいバージョンをリリースする際のチェックリスト：

### 1. バージョン更新

- [ ] `Cargo.toml` のバージョンを更新
- [ ] `CHANGELOG.md` を更新
- [ ] `flake.nix` のバージョンを更新

### 2. 自動リリース

- [ ] Gitタグを作成してプッシュ
  ```bash
  git tag v0.1.2
  git push origin v0.1.2
  ```
- [ ] GitHub Actionsでビルドが完了するのを待つ
- [ ] crates.io に公開
  ```bash
  cargo publish
  ```

### 3. 手動リリース（必要に応じて）

- [ ] **AUR**: PKGBUILDのバージョンとSHA256を更新
- [ ] **Homebrew**: Formulaのバージョンとsha256を更新
- [ ] **PPA**: debian/changelogを更新してdput
- [ ] **Snap**: snapcraft.yamlのバージョンを更新してアップロード

### 4. ドキュメント更新

- [ ] README.mdのインストール手順を確認
- [ ] 各パッケージマネージャーのREADMEを更新

## CI/CD 自動化

将来の自動化案：

### GitHub Actions ワークフロー

```yaml
# .github/workflows/publish-all.yml
name: Publish to All Package Managers

on:
  release:
    types: [published]

jobs:
  crates-io:
    # 既に実装済み

  aur:
    # AUR SSH経由で自動プッシュ

  ppa:
    # Launchpad PPA に自動アップロード

  snap:
    # Snap Store に自動公開
```

## トラブルシューティング

各パッケージマネージャーのトラブルシューティングは、それぞれのREADMEを参照してください：

- [AUR](aur/README.md#トラブルシューティング)
- [Debian/PPA](debian/README.md#トラブルシューティング)
- [Homebrew](homebrew/README.md#テスト)
- [Snap](../snap/README.md#トラブルシューティング)

## コントリビューション

パッケージングファイルの改善提案は、PRまたはIssueでお願いします。
