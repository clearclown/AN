# ANのインストール

AN本体のインストール方法を説明します。

## システム要件

### 対応OS

- Debian 11+
- Ubuntu 20.04+
- Linux Mint 20+
- Pop!_OS 20.04+
- その他Debian/Ubuntu派生ディストリビューション

### 対応アーキテクチャ

- x86_64 (Intel/AMD 64bit)
- aarch64 (ARM 64bit) - 将来対応予定

### 必要なパッケージ

```bash
# 通常は既にインストールされている
curl
wget
```

## インストール方法

### 方法1: ワンライナーインストール（推奨）

```bash
curl -fsSL https://raw.githubusercontent.com/clearclown/AN/main/install.sh | bash
```

このスクリプトは以下を実行します:
1. 最新リリースをダウンロード
2. `/usr/local/bin/an` にインストール
3. 実行権限を設定

### 方法2: 手動インストール

```bash
# 1. 最新リリースをダウンロード
wget https://github.com/clearclown/AN/releases/latest/download/an-linux-x86_64

# 2. 実行権限を付与
chmod +x an-linux-x86_64

# 3. PATHの通った場所に移動
sudo mv an-linux-x86_64 /usr/local/bin/an
```

### 方法3: ユーザーローカルインストール

sudo権限なしでインストールする場合:

```bash
# ~/.local/bin にインストール
mkdir -p ~/.local/bin
wget -O ~/.local/bin/an https://github.com/clearclown/AN/releases/latest/download/an-linux-x86_64
chmod +x ~/.local/bin/an

# PATHに追加（まだの場合）
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

### 方法4: Cargoからビルド（開発者向け）

```bash
# Rustがインストールされている場合
cargo install --git https://github.com/clearclown/AN

# または、リポジトリをクローンしてビルド
git clone https://github.com/clearclown/AN.git
cd AN
cargo build --release
sudo cp target/release/an /usr/local/bin/
```

## インストールの確認

```bash
# バージョン確認
an --version

# ヘルプ表示
an --help
```

## アンインストール

```bash
# /usr/local/bin にインストールした場合
sudo rm /usr/local/bin/an

# ~/.local/bin にインストールした場合
rm ~/.local/bin/an

# 設定ディレクトリも削除する場合
rm -rf ~/.config/an
```

## アップデート

ANがインストールされている場合、以下のコマンドで更新できます:

```bash
an update
```

## トラブルシューティング

### 「command not found」エラー

```bash
# PATHを確認
echo $PATH

# ~/.local/bin がPATHに含まれていない場合
export PATH="$HOME/.local/bin:$PATH"
```

### 権限エラー

```bash
# /usr/local/bin への書き込み権限がない場合
# ユーザーローカルインストールを使用
mkdir -p ~/.local/bin
# 上記の方法3を参照
```

### SSL証明書エラー

```bash
# ca-certificates が古い場合
sudo apt update && sudo apt install -y ca-certificates
```

## 次のステップ

- [クイックスタート](./getting-started.md) で基本操作を学ぶ
- [コマンドリファレンス](./commands.md) で全コマンドを確認
