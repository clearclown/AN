# AN (安装) - Unified Package Manager for Linux

[![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

**English** | [日本語](docs/readmeLang/README_ja.md) | [简体中文](docs/readmeLang/README_zh-CN.md) | [繁體中文](docs/readmeLang/README_zh-TW.md) | [Русский](docs/readmeLang/README_ru.md) | [Українська](docs/readmeLang/README_uk.md) | [فارسی](docs/readmeLang/README_fa.md) | [العربية](docs/readmeLang/README_ar.md)

**AN** is a unified package manager for Debian/Ubuntu-based Linux distributions.

Manage `.deb`, `AppImage`, and `Flatpak` packages through a single interface while keeping your system clean.

## Features

- **Unified Interface**: Manage all package formats with `an install` / `an remove`
- **Complete Removal**: Purge config files, dependencies, and user data completely
- **Transparency**: Display URLs for remote installations and require user confirmation
- **Flatpak Aliases**: Run `flatpak run org.gimp.GIMP` as simply `gimp`

## Installation

### One-liner (Recommended)

```bash
curl -fsSL https://raw.githubusercontent.com/clearclown/AN/main/install.sh | bash
```

### Cargo (For Rust Users)

```bash
# From crates.io
cargo install an-installer

# Directly from GitHub
cargo install --git https://github.com/clearclown/AN
```

### Arch Linux (AUR)

```bash
# yay (stable version)
yay -S an-installer

# paru (stable version)
paru -S an-installer

# Development version
yay -S an-installer-git
paru -S an-installer-git
```

### Nix

```bash
# Using Flakes
nix profile install github:clearclown/AN

# Temporary usage
nix run github:clearclown/AN -- --help
```

### Homebrew (Linux)

```bash
brew tap clearclown/an
brew install an
```

### apt (Debian/Ubuntu PPA)

```bash
# Add PPA (Coming soon)
sudo add-apt-repository ppa:clearclown/an
sudo apt update
sudo apt install an-installer
```

### Snap

```bash
# From Snap Store (Coming soon)
sudo snap install an-installer --classic
```

### Manual Download

Download binaries from [Releases](https://github.com/clearclown/AN/releases):

| Architecture | Filename |
|--------------|----------|
| x86_64 | `an-linux-x86_64` |
| aarch64 (ARM64) | `an-linux-aarch64` |

```bash
# Example: x86_64
wget https://github.com/clearclown/AN/releases/latest/download/an-linux-x86_64
chmod +x an-linux-x86_64
sudo mv an-linux-x86_64 /usr/local/bin/an
```

### Build from Source

```bash
git clone https://github.com/clearclown/AN
cd AN
cargo build --release
sudo cp target/release/an /usr/local/bin/
```

## Usage

### Install Applications

```bash
# Install remote application
an install firefox

# Install local .deb file
an install ~/Downloads/vscode.deb

# Install local AppImage
an install ~/Downloads/Obsidian.AppImage
```

### Remove Applications

```bash
# Complete removal (including config files)
an remove firefox
an rm vscode
an uninstall obsidian
```

### Generate Flatpak Aliases

```bash
# Create aliases for all Flatpak apps
an link
```

### Update

```bash
# Update AN itself and app database
an update
```

### Search and Display Information

```bash
# List installed applications
an list

# Search available applications
an search browser
an search          # List all apps

# Display application details
an info firefox

# Sync app database from GitHub
an sync
```

## Command Reference

| Command | Aliases | Description |
|---------|---------|-------------|
| `an install <target>` | `i` | Install an application |
| `an remove <target>` | `rm`, `uninstall` | Completely remove an application |
| `an link` | `l` | Generate Flatpak aliases |
| `an update` | - | Update AN and database |
| `an list` | `ls` | List installed applications |
| `an search [query]` | `s` | Search app database |
| `an info <name>` | - | Display application details |
| `an sync` | - | Sync app database from GitHub |

## Architecture

```
┌───────────────────────────────────────────────────────────────┐
│                         CLI Layer                              │
│                    (clap derive macros)                        │
├───────────────────────────────────────────────────────────────┤
│                       Command Layer                            │
│  install │ remove │ link │ update │ list │ search │ info │ sync │
├───────────────────────────────────────────────────────────────┤
│                       Handler Layer                            │
│            deb │ appimage │ flatpak │ remote                   │
├───────────────────────────────────────────────────────────────┤
│                         DB Layer                               │
│                   TOML App Database                            │
└───────────────────────────────────────────────────────────────┘
```

## Directory Structure

| Purpose | Path |
|---------|------|
| Executable | `/usr/local/bin/an` or `~/.local/bin/an` |
| AppImage Storage | `~/Applications/` |
| Symbolic Links | `~/.local/bin/` |
| Config/Cache | `~/.config/an/` |

## Development

### Requirements

- Rust 1.70+
- Linux (Debian/Ubuntu-based)

### Building

```bash
# Development build
cargo build

# Run tests
cargo test

# Release build
cargo build --release
```

### Project Structure

```
AN/
├── src/                 # Rust source code
│   ├── commands/        # Subcommand implementations
│   ├── handlers/        # Package format handlers
│   ├── db/              # App database layer
│   └── utils/           # Utilities
├── apps/                # App database (TOML)
├── docs/                # Documentation
│   ├── spec/            # Feature specifications
│   ├── design/          # Design & state machine diagrams
│   └── guides/          # User guides
└── tests/               # Tests
```

## Documentation

- [Feature Specifications](docs/spec/README.md)
- [Design Documents](docs/design/README.md)
- [User Guides](docs/guides/README.md)
- [CLAUDE.md](CLAUDE.md) - Developer Guide

## License

MIT License - See [LICENSE](LICENSE) for details

## Acknowledgments

This project is inspired by [AM (App Manager)](https://github.com/ivan-hc/AM).
