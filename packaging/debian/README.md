# AN Debian/PPA Packaging

このディレクトリには、Debian/Ubuntu向けのパッケージングファイルが含まれています。

## 必要なツール

```bash
sudo apt install debhelper devscripts build-essential cargo rustc
```

## ローカルで.debパッケージをビルド

```bash
# プロジェクトルートで実行
dpkg-buildpackage -us -uc -b

# 生成された.debファイルをインストール
sudo dpkg -i ../an-installer_0.1.1-1_amd64.deb
```

## PPAへの公開

### 前提条件

1. Launchpadアカウントを作成
2. GPG鍵を生成・登録
3. PPAを作成（例: `ppa:clearclown/an`）

### 手順

#### 1. GPG鍵の準備

```bash
# GPG鍵を生成（既にある場合はスキップ）
gpg --full-generate-key

# 鍵IDを確認
gpg --list-keys

# Launchpadに公開鍵を送信
gpg --keyserver keyserver.ubuntu.com --send-keys YOUR_KEY_ID

# Launchpadで鍵を認証
# https://launchpad.net/~/+editpgpkeys
```

#### 2. ソースパッケージのビルド

```bash
# プロジェクトルートで実行

# Ubuntu 22.04 (Jammy) 用
debuild -S -sa

# 複数のUbuntuバージョン用にビルドする場合
# debian/changelogのディストリビューション名を変更
dch -D jammy -v 0.1.1-1~jammy1
debuild -S -sa

dch -D focal -v 0.1.1-1~focal1
debuild -S -sa
```

#### 3. PPAへアップロード

```bash
# Jammy用
dput ppa:clearclown/an ../an-installer_0.1.1-1~jammy1_source.changes

# Focal用
dput ppa:clearclown/an ../an-installer_0.1.1-1~focal1_source.changes
```

#### 4. ビルド確認

Launchpadでビルドが完了するのを待ちます（通常5〜30分）：
https://launchpad.net/~clearclown/+archive/ubuntu/an

#### 5. ユーザー向けインストール手順

```bash
sudo add-apt-repository ppa:clearclown/an
sudo apt update
sudo apt install an-installer
```

## GitHub Actionsによる自動化

`.github/workflows/ppa.yml` を作成して自動化することも可能です：

```yaml
name: Publish to PPA

on:
  release:
    types: [published]

jobs:
  ppa:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3

      - name: Import GPG key
        run: |
          echo "${{ secrets.PPA_GPG_KEY }}" | gpg --import

      - name: Build and upload to PPA
        run: |
          sudo apt-get update
          sudo apt-get install -y debhelper devscripts

          # Jammy
          dch -D jammy -v ${{ github.ref_name }}-1~jammy1
          debuild -S -sa
          dput ppa:clearclown/an ../*.changes
```

## トラブルシューティング

### ビルドエラー

```bash
# 依存関係の確認
dpkg-checkbuilddeps

# クリーンビルド
debclean
dpkg-buildpackage -us -uc -b
```

### GPG署名エラー

```bash
# 正しい鍵を使用
debuild -S -k YOUR_KEY_ID
```

### PPA アップロードエラー

```bash
# dputの設定を確認
cat ~/.dput.cf

# 手動でアップロード
dput ppa:clearclown/an ../an-installer_*.changes
```
