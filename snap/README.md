# AN Snap Package

このディレクトリには、Snap向けのパッケージングファイルが含まれています。

## 必要なツール

```bash
sudo snap install snapcraft --classic
```

## ローカルでSnapパッケージをビルド

```bash
# プロジェクトルートで実行
snapcraft

# 生成されたSnapファイルをインストール
sudo snap install ./an-installer_0.1.1_amd64.snap --classic --dangerous
```

## Snap Storeへの公開

### 前提条件

1. Ubuntu One アカウントを作成
2. Snapcraft Developer アカウントを登録: https://dashboard.snapcraft.io/
3. 名前を予約（登録）

### 手順

#### 1. Snapcraftにログイン

```bash
snapcraft login
```

#### 2. 名前を登録（初回のみ）

```bash
snapcraft register an-installer
```

#### 3. ビルド

```bash
snapcraft
```

#### 4. アップロード

```bash
# edge チャンネルにアップロード（テスト用）
snapcraft upload --release=edge an-installer_0.1.1_amd64.snap

# beta チャンネルにアップロード
snapcraft upload --release=beta an-installer_0.1.1_amd64.snap

# stable チャンネルにアップロード（本番）
snapcraft upload --release=stable an-installer_0.1.1_amd64.snap
```

#### 5. ユーザー向けインストール手順

```bash
# stable チャンネルから
sudo snap install an-installer --classic

# edge チャンネルから（開発版）
sudo snap install an-installer --classic --edge
```

## GitHub Actionsによる自動化

`.github/workflows/snap.yml` を作成して自動化：

```yaml
name: Publish Snap

on:
  release:
    types: [published]

jobs:
  snap:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3

      - name: Build snap
        uses: snapcore/action-build@v1

      - name: Publish to Snap Store
        uses: snapcore/action-publish@v1
        env:
          SNAPCRAFT_STORE_CREDENTIALS: ${{ secrets.SNAP_STORE_TOKEN }}
        with:
          snap: an-installer_*.snap
          release: stable
```

### Snap Store トークンの取得

```bash
snapcraft export-login --snaps=an-installer --channels=stable,edge snap-store-token.txt

# トークンをGitHub Secretsに追加
# SNAP_STORE_TOKEN という名前で追加
```

## マルチアーキテクチャビルド

```bash
# amd64
snapcraft --target-arch=amd64

# arm64
snapcraft --target-arch=arm64

# armhf
snapcraft --target-arch=armhf
```

## Classic confinement について

ANは `classic` confinement を使用しています。これは：

- システム全体へのアクセスが必要（dpkg, flatpakとの統合）
- Snap Store審査が必要
- ユーザーは `--classic` フラグでインストール

Strict confinement への移行を検討する場合：
1. `confinement: strict` に変更
2. 必要な`interfaces`を追加（例: `network`, `home`）
3. セキュリティポリシーを確認

## トラブルシューティング

### ビルドエラー

```bash
# クリーンビルド
snapcraft clean
snapcraft

# 詳細ログ
snapcraft --debug
```

### インストール後の動作確認

```bash
# Snapのバージョン確認
snap list an-installer

# ログ確認
snap logs an-installer

# アンインストール
sudo snap remove an-installer
```
