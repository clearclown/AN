# AN Homebrew Formula

このディレクトリには、Homebrew (Linux) 向けのFormulaファイルが含まれています。

## インストール方法

### Tap経由（推奨）

```bash
brew tap clearclown/an
brew install an
```

### URL指定

```bash
brew install https://raw.githubusercontent.com/clearclown/AN/main/packaging/homebrew/an.rb
```

## Tapリポジトリの作成

Homebrew Tapを作成するには、`homebrew-an` という名前のリポジトリを作成し、Formulaを配置します。

### 手順

1. GitHubで `homebrew-an` リポジトリを作成

2. Formulaファイルを配置

```bash
git clone https://github.com/clearclown/homebrew-an.git
cd homebrew-an
mkdir Formula
cp /path/to/AN/packaging/homebrew/an.rb Formula/
git add Formula/an.rb
git commit -m "Add AN formula"
git push
```

3. ユーザーがTapを追加してインストール

```bash
brew tap clearclown/an
brew install an
```

## SHA256の計算

新しいバージョンをリリースする際は、SHA256ハッシュを更新する必要があります：

```bash
# GitHubリリースのtarballをダウンロード
wget https://github.com/clearclown/AN/archive/v0.1.1.tar.gz

# SHA256を計算
shasum -a 256 v0.1.1.tar.gz

# Formula内のsha256行を更新
vim Formula/an.rb
```

## テスト

Formulaが正しく動作するかテスト：

```bash
brew install --build-from-source an
brew test an
brew audit --strict an
```

## バージョンアップ

1. `an.rb` のバージョンとURLを更新
2. SHA256ハッシュを更新
3. コミット＆プッシュ

```bash
# Formulaを編集
vim Formula/an.rb

# テスト
brew reinstall --build-from-source an

# コミット
git add Formula/an.rb
git commit -m "Update to version 0.1.1"
git push
```
