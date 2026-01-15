# AN (安装) - Linux 统一包管理器

[![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/License-MIT-blue.svg)](../../LICENSE)

[English](../../README.md) | [日本語](README_ja.md) | **简体中文** | [繁體中文](README_zh-TW.md) | [Русский](README_ru.md) | [Українська](README_uk.md) | [فارسی](README_fa.md) | [العربية](README_ar.md)

**AN** 是一个面向 Debian/Ubuntu 系 Linux 发行版的统一包管理器。

通过单一界面管理 `.deb`、`AppImage` 和 `Flatpak` 软件包，保持系统整洁。

## 特性

- **统一界面**：使用 `an install` / `an remove` 管理所有包格式
- **完全删除**：彻底清除配置文件、依赖项和用户数据
- **透明性**：显示远程安装的 URL 并要求用户确认
- **Flatpak 别名**：将 `flatpak run org.gimp.GIMP` 简化为 `gimp`

## 安装

### 一键安装（推荐）

```bash
curl -fsSL https://raw.githubusercontent.com/clearclown/AN/main/install.sh | bash
```

### Cargo（适用于 Rust 用户）

```bash
# 从 crates.io 安装
cargo install an-installer

# 直接从 GitHub 安装
cargo install --git https://github.com/clearclown/AN
```

### Arch Linux (AUR)

```bash
# yay（稳定版）
yay -S an-installer

# paru（稳定版）
paru -S an-installer

# 开发版
yay -S an-installer-git
paru -S an-installer-git
```

### Nix

```bash
# 使用 Flakes
nix profile install github:clearclown/AN

# 临时使用
nix run github:clearclown/AN -- --help
```

### Homebrew (Linux)

```bash
brew tap clearclown/an
brew install an
```

### apt (Debian/Ubuntu PPA)

```bash
# 添加 PPA（即将推出）
sudo add-apt-repository ppa:clearclown/an
sudo apt update
sudo apt install an-installer
```

### Snap

```bash
# 从 Snap Store 安装（即将推出）
sudo snap install an-installer --classic
```

### 手动下载

从 [Releases](https://github.com/clearclown/AN/releases) 下载二进制文件：

| 架构 | 文件名 |
|------|--------|
| x86_64 | `an-linux-x86_64` |
| aarch64 (ARM64) | `an-linux-aarch64` |

```bash
# 示例：x86_64
wget https://github.com/clearclown/AN/releases/latest/download/an-linux-x86_64
chmod +x an-linux-x86_64
sudo mv an-linux-x86_64 /usr/local/bin/an
```

### 从源码构建

```bash
git clone https://github.com/clearclown/AN
cd AN
cargo build --release
sudo cp target/release/an /usr/local/bin/
```

## 使用方法

### 安装应用程序

```bash
# 安装远程应用
an install firefox

# 安装本地 .deb 文件
an install ~/Downloads/vscode.deb

# 安装本地 AppImage
an install ~/Downloads/Obsidian.AppImage
```

### 删除应用程序

```bash
# 完全删除（包括配置文件）
an remove firefox
an rm vscode
an uninstall obsidian
```

### 生成 Flatpak 别名

```bash
# 为所有 Flatpak 应用创建别名
an link
```

### 更新

```bash
# 更新 AN 本身和应用数据库
an update
```

### 搜索和显示信息

```bash
# 列出已安装的应用
an list

# 搜索可用应用
an search browser
an search          # 列出所有应用

# 显示应用详情
an info firefox

# 从 GitHub 同步应用数据库
an sync
```

## 命令参考

| 命令 | 别名 | 说明 |
|------|------|------|
| `an install <target>` | `i` | 安装应用程序 |
| `an remove <target>` | `rm`, `uninstall` | 完全删除应用程序 |
| `an link` | `l` | 生成 Flatpak 别名 |
| `an update` | - | 更新 AN 和数据库 |
| `an list` | `ls` | 列出已安装的应用 |
| `an search [query]` | `s` | 搜索应用数据库 |
| `an info <name>` | - | 显示应用详情 |
| `an sync` | - | 从 GitHub 同步应用数据库 |

## 架构

```
┌───────────────────────────────────────────────────────────────┐
│                         CLI 层                                 │
│                    (clap 派生宏)                               │
├───────────────────────────────────────────────────────────────┤
│                       命令层                                   │
│  install │ remove │ link │ update │ list │ search │ info │ sync │
├───────────────────────────────────────────────────────────────┤
│                       处理器层                                 │
│            deb │ appimage │ flatpak │ remote                   │
├───────────────────────────────────────────────────────────────┤
│                       数据库层                                 │
│                   TOML 应用数据库                              │
└───────────────────────────────────────────────────────────────┘
```

## 目录结构

| 用途 | 路径 |
|------|------|
| 可执行文件 | `/usr/local/bin/an` 或 `~/.local/bin/an` |
| AppImage 存储 | `~/Applications/` |
| 符号链接 | `~/.local/bin/` |
| 配置/缓存 | `~/.config/an/` |

## 开发

### 要求

- Rust 1.70+
- Linux（基于 Debian/Ubuntu）

### 构建

```bash
# 开发构建
cargo build

# 运行测试
cargo test

# 发布构建
cargo build --release
```

### 项目结构

```
AN/
├── src/                 # Rust 源代码
│   ├── commands/        # 子命令实现
│   ├── handlers/        # 包格式处理器
│   ├── db/              # 应用数据库层
│   └── utils/           # 实用工具
├── apps/                # 应用数据库 (TOML)
├── docs/                # 文档
│   ├── spec/            # 功能规范
│   ├── design/          # 设计和状态机图
│   └── guides/          # 用户指南
└── tests/               # 测试
```

## 文档

- [功能规范](../spec/README.md)
- [设计文档](../design/README.md)
- [用户指南](../guides/README.md)
- [CLAUDE.md](../../CLAUDE.md) - 开发者指南

## 许可证

MIT 许可证 - 详见 [LICENSE](../../LICENSE)

## 致谢

本项目受 [AM (App Manager)](https://github.com/ivan-hc/AM) 启发。
