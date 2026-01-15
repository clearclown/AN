# AN (安装) - Linux 統一套件管理器

[![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/License-MIT-blue.svg)](../../LICENSE)

[English](../../README.md) | [日本語](README_ja.md) | [简体中文](README_zh-CN.md) | **繁體中文** | [Русский](README_ru.md) | [Українська](README_uk.md) | [فارسی](README_fa.md) | [العربية](README_ar.md)

**AN** 是一個面向 Debian/Ubuntu 系 Linux 發行版的統一套件管理器。

透過單一介面管理 `.deb`、`AppImage` 和 `Flatpak` 軟體套件，保持系統整潔。

## 特性

- **統一介面**：使用 `an install` / `an remove` 管理所有套件格式
- **完全刪除**：徹底清除設定檔、相依性和使用者資料
- **透明性**：顯示遠端安裝的 URL 並要求使用者確認
- **Flatpak 別名**：將 `flatpak run org.gimp.GIMP` 簡化為 `gimp`

## 安裝

### 一鍵安裝（推薦）

```bash
curl -fsSL https://raw.githubusercontent.com/clearclown/AN/main/install.sh | bash
```

### Cargo（適用於 Rust 使用者）

```bash
# 從 crates.io 安裝
cargo install an-installer

# 直接從 GitHub 安裝
cargo install --git https://github.com/clearclown/AN
```

### Arch Linux (AUR)

```bash
# yay（穩定版）
yay -S an-installer

# paru（穩定版）
paru -S an-installer

# 開發版
yay -S an-installer-git
paru -S an-installer-git
```

### Nix

```bash
# 使用 Flakes
nix profile install github:clearclown/AN

# 臨時使用
nix run github:clearclown/AN -- --help
```

### Homebrew (Linux)

```bash
brew tap clearclown/an
brew install an
```

### apt (Debian/Ubuntu PPA)

```bash
# 新增 PPA（即將推出）
sudo add-apt-repository ppa:clearclown/an
sudo apt update
sudo apt install an-installer
```

### Snap

```bash
# 從 Snap Store 安裝（即將推出）
sudo snap install an-installer --classic
```

### 手動下載

從 [Releases](https://github.com/clearclown/AN/releases) 下載二進位檔案：

| 架構 | 檔案名稱 |
|------|----------|
| x86_64 | `an-linux-x86_64` |
| aarch64 (ARM64) | `an-linux-aarch64` |

```bash
# 範例：x86_64
wget https://github.com/clearclown/AN/releases/latest/download/an-linux-x86_64
chmod +x an-linux-x86_64
sudo mv an-linux-x86_64 /usr/local/bin/an
```

### 從原始碼建置

```bash
git clone https://github.com/clearclown/AN
cd AN
cargo build --release
sudo cp target/release/an /usr/local/bin/
```

## 使用方法

### 安裝應用程式

```bash
# 安裝遠端應用程式
an install firefox

# 安裝本機 .deb 檔案
an install ~/Downloads/vscode.deb

# 安裝本機 AppImage
an install ~/Downloads/Obsidian.AppImage
```

### 刪除應用程式

```bash
# 完全刪除（包括設定檔）
an remove firefox
an rm vscode
an uninstall obsidian
```

### 產生 Flatpak 別名

```bash
# 為所有 Flatpak 應用程式建立別名
an link
```

### 更新

```bash
# 更新 AN 本身和應用程式資料庫
an update
```

### 搜尋和顯示資訊

```bash
# 列出已安裝的應用程式
an list

# 搜尋可用應用程式
an search browser
an search          # 列出所有應用程式

# 顯示應用程式詳情
an info firefox

# 從 GitHub 同步應用程式資料庫
an sync
```

## 命令參考

| 命令 | 別名 | 說明 |
|------|------|------|
| `an install <target>` | `i` | 安裝應用程式 |
| `an remove <target>` | `rm`, `uninstall` | 完全刪除應用程式 |
| `an link` | `l` | 產生 Flatpak 別名 |
| `an update` | - | 更新 AN 和資料庫 |
| `an list` | `ls` | 列出已安裝的應用程式 |
| `an search [query]` | `s` | 搜尋應用程式資料庫 |
| `an info <name>` | - | 顯示應用程式詳情 |
| `an sync` | - | 從 GitHub 同步應用程式資料庫 |

## 架構

```
┌───────────────────────────────────────────────────────────────┐
│                         CLI 層                                 │
│                    (clap 衍生巨集)                             │
├───────────────────────────────────────────────────────────────┤
│                       命令層                                   │
│  install │ remove │ link │ update │ list │ search │ info │ sync │
├───────────────────────────────────────────────────────────────┤
│                       處理器層                                 │
│            deb │ appimage │ flatpak │ remote                   │
├───────────────────────────────────────────────────────────────┤
│                       資料庫層                                 │
│                   TOML 應用程式資料庫                          │
└───────────────────────────────────────────────────────────────┘
```

## 目錄結構

| 用途 | 路徑 |
|------|------|
| 可執行檔 | `/usr/local/bin/an` 或 `~/.local/bin/an` |
| AppImage 儲存 | `~/Applications/` |
| 符號連結 | `~/.local/bin/` |
| 設定/快取 | `~/.config/an/` |

## 開發

### 需求

- Rust 1.70+
- Linux（基於 Debian/Ubuntu）

### 建置

```bash
# 開發建置
cargo build

# 執行測試
cargo test

# 發行建置
cargo build --release
```

### 專案結構

```
AN/
├── src/                 # Rust 原始碼
│   ├── commands/        # 子命令實作
│   ├── handlers/        # 套件格式處理器
│   ├── db/              # 應用程式資料庫層
│   └── utils/           # 工具程式
├── apps/                # 應用程式資料庫 (TOML)
├── docs/                # 文件
│   ├── spec/            # 功能規格
│   ├── design/          # 設計和狀態機圖
│   └── guides/          # 使用者指南
└── tests/               # 測試
```

## 文件

- [功能規格](../spec/README.md)
- [設計文件](../design/README.md)
- [使用者指南](../guides/README.md)
- [CLAUDE.md](../../CLAUDE.md) - 開發者指南

## 授權

MIT 授權 - 詳見 [LICENSE](../../LICENSE)

## 致謝

本專案受 [AM (App Manager)](https://github.com/ivan-hc/AM) 啟發。
