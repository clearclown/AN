# AN AUR Packages

このディレクトリには、Arch Linux向けのPKGBUILDファイルが含まれています。

## パッケージ

### an-installer
crates.ioからビルドする安定版パッケージ

```bash
# AUR helperを使用
yay -S an-installer
# または
paru -S an-installer

# 手動インストール
git clone https://aur.archlinux.org/an-installer.git
cd an-installer
makepkg -si
```

### an-installer-git
GitHubから最新のコードをビルドする開発版パッケージ

```bash
# AUR helperを使用
yay -S an-installer-git
# または
paru -S an-installer-git

# 手動インストール
git clone https://aur.archlinux.org/an-installer-git.git
cd an-installer-git
makepkg -si
```

## AURへの公開

これらのPKGBUILDファイルは、AUR (Arch User Repository) に公開する準備ができています。

AURへの公開手順：

1. AURアカウントを作成: https://aur.archlinux.org/register
2. SSH鍵を設定: https://wiki.archlinux.org/title/AUR_submission_guidelines
3. リポジトリを作成してプッシュ:

```bash
# 安定版
git clone ssh://aur@aur.archlinux.org/an-installer.git
cd an-installer
cp ../packaging/aur/PKGBUILD .
makepkg --printsrcinfo > .SRCINFO
git add PKGBUILD .SRCINFO
git commit -m "Initial commit: an-installer 0.1.1"
git push

# 開発版
git clone ssh://aur@aur.archlinux.org/an-installer-git.git
cd an-installer-git
cp ../packaging/aur/PKGBUILD-git PKGBUILD
makepkg --printsrcinfo > .SRCINFO
git add PKGBUILD .SRCINFO
git commit -m "Initial commit: an-installer-git"
git push
```

## メンテナンス

バージョンアップ時：

```bash
# PKGBUILDのpkgverを更新
vim PKGBUILD

# チェックサムを計算
makepkg -g >> PKGBUILD

# .SRCINFOを更新
makepkg --printsrcinfo > .SRCINFO

# コミット＆プッシュ
git add PKGBUILD .SRCINFO
git commit -m "Update to version X.Y.Z"
git push
```
