---
layout: default
title: AN - Unified Package Manager for Linux
---

<div align="center">

# AN (安装)

**Unified Package Manager for Linux**

[![GitHub release](https://img.shields.io/github/v/release/clearclown/AN?style=for-the-badge)](https://github.com/clearclown/AN/releases)
[![License](https://img.shields.io/badge/license-MIT-blue?style=for-the-badge)](https://github.com/clearclown/AN/blob/main/LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.70+-orange?style=for-the-badge&logo=rust)](https://www.rust-lang.org/)

</div>

---

## What is AN?

**AN** is a unified package manager for Debian/Ubuntu-based Linux distributions. It manages `.deb`, `AppImage`, and `Flatpak` packages through a single, intuitive interface.

## Features

- **Unified Interface** - Install and remove any package format with `an install` / `an remove`
- **Complete Removal** - Purge config files, dependencies, and user data
- **Transparency** - Shows download URLs and requires user confirmation
- **Flatpak Aliases** - Run `gimp` instead of `flatpak run org.gimp.GIMP`

## Quick Install

```bash
curl -fsSL https://raw.githubusercontent.com/clearclown/AN/main/install.sh | bash
```

## Usage

```bash
# Install an app
an install firefox

# Remove an app completely
an remove firefox

# Create Flatpak aliases
an link

# Search available apps
an search browser

# Update AN and app database
an update
```

## Installation Methods

| Method | Command |
|--------|---------|
| **One-liner** | `curl -fsSL https://raw.githubusercontent.com/clearclown/AN/main/install.sh \| bash` |
| **Cargo** | `cargo install an-installer` |
| **AUR** | `yay -S an` |
| **Nix** | `nix run github:clearclown/AN` |

## Supported Package Formats

| Format | Install | Remove | Update |
|--------|---------|--------|--------|
| AppImage | ✅ | ✅ | ✅ |
| Flatpak | ✅ | ✅ | ✅ |
| Deb | ✅ | ✅ | via apt |

## Commands

| Command | Alias | Description |
|---------|-------|-------------|
| `an install <target>` | `i` | Install an app |
| `an remove <target>` | `rm` | Remove an app completely |
| `an link` | `l` | Create Flatpak aliases |
| `an update` | - | Update AN and app database |
| `an list` | `ls` | List installed apps |
| `an search [query]` | `s` | Search app database |
| `an info <name>` | - | Show app details |
| `an sync` | - | Sync app database from GitHub |

## Links

- [GitHub Repository](https://github.com/clearclown/AN)
- [Releases](https://github.com/clearclown/AN/releases)
- [Documentation](https://github.com/clearclown/AN/tree/main/docs)
- [Report Issues](https://github.com/clearclown/AN/issues)

---

<div align="center">

Made with ❤️ in Rust

MIT License © 2025

</div>
