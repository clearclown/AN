# AN (安装) - مدیریت بسته یکپارچه برای لینوکس

[![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/License-MIT-blue.svg)](../../LICENSE)

[English](../../README.md) | [日本語](README_ja.md) | [简体中文](README_zh-CN.md) | [繁體中文](README_zh-TW.md) | [Русский](README_ru.md) | [Українська](README_uk.md) | **فارسی** | [العربية](README_ar.md)

**AN** یک مدیریت بسته یکپارچه برای توزیع‌های لینوکس مبتنی بر Debian/Ubuntu است.

بسته‌های `.deb`، `AppImage` و `Flatpak` را از طریق یک رابط واحد مدیریت کنید و سیستم خود را تمیز نگه دارید.

## ویژگی‌ها

- **رابط یکپارچه**: مدیریت تمام فرمت‌های بسته با `an install` / `an remove`
- **حذف کامل**: پاکسازی کامل فایل‌های پیکربندی، وابستگی‌ها و داده‌های کاربر
- **شفافیت**: نمایش URL برای نصب از راه دور و درخواست تأیید کاربر
- **نام‌های مستعار Flatpak**: اجرای `flatpak run org.gimp.GIMP` به سادگی با `gimp`

## نصب

### نصب با یک دستور (توصیه شده)

```bash
curl -fsSL https://raw.githubusercontent.com/clearclown/AN/main/install.sh | bash
```

### Cargo (برای کاربران Rust)

```bash
# از crates.io
cargo install an-installer

# مستقیماً از GitHub
cargo install --git https://github.com/clearclown/AN
```

### Arch Linux (AUR)

```bash
# yay (نسخه پایدار)
yay -S an-installer

# paru (نسخه پایدار)
paru -S an-installer

# نسخه توسعه
yay -S an-installer-git
paru -S an-installer-git
```

### Nix

```bash
# استفاده از Flakes
nix profile install github:clearclown/AN

# استفاده موقت
nix run github:clearclown/AN -- --help
```

### Homebrew (Linux)

```bash
brew tap clearclown/an
brew install an
```

### apt (Debian/Ubuntu PPA)

```bash
# اضافه کردن PPA (به زودی)
sudo add-apt-repository ppa:clearclown/an
sudo apt update
sudo apt install an-installer
```

### Snap

```bash
# از Snap Store (به زودی)
sudo snap install an-installer --classic
```

### دانلود دستی

فایل‌های باینری را از [Releases](https://github.com/clearclown/AN/releases) دانلود کنید:

| معماری | نام فایل |
|---------|----------|
| x86_64 | `an-linux-x86_64` |
| aarch64 (ARM64) | `an-linux-aarch64` |

```bash
# مثال: x86_64
wget https://github.com/clearclown/AN/releases/latest/download/an-linux-x86_64
chmod +x an-linux-x86_64
sudo mv an-linux-x86_64 /usr/local/bin/an
```

### ساخت از سورس

```bash
git clone https://github.com/clearclown/AN
cd AN
cargo build --release
sudo cp target/release/an /usr/local/bin/
```

## استفاده

### نصب برنامه‌ها

```bash
# نصب برنامه از راه دور
an install firefox

# نصب فایل محلی .deb
an install ~/Downloads/vscode.deb

# نصب AppImage محلی
an install ~/Downloads/Obsidian.AppImage
```

### حذف برنامه‌ها

```bash
# حذف کامل (شامل فایل‌های پیکربندی)
an remove firefox
an rm vscode
an uninstall obsidian
```

### ایجاد نام‌های مستعار Flatpak

```bash
# ایجاد نام‌های مستعار برای همه برنامه‌های Flatpak
an link
```

### به‌روزرسانی

```bash
# به‌روزرسانی AN و پایگاه داده برنامه‌ها
an update
```

### جستجو و نمایش اطلاعات

```bash
# لیست برنامه‌های نصب شده
an list

# جستجوی برنامه‌های موجود
an search browser
an search          # لیست همه برنامه‌ها

# نمایش جزئیات برنامه
an info firefox

# همگام‌سازی پایگاه داده برنامه‌ها از GitHub
an sync
```

## راهنمای دستورات

| دستور | نام‌های مستعار | توضیحات |
|-------|----------------|----------|
| `an install <target>` | `i` | نصب برنامه |
| `an remove <target>` | `rm`, `uninstall` | حذف کامل برنامه |
| `an link` | `l` | ایجاد نام‌های مستعار Flatpak |
| `an update` | - | به‌روزرسانی AN و پایگاه داده |
| `an list` | `ls` | لیست برنامه‌های نصب شده |
| `an search [query]` | `s` | جستجو در پایگاه داده برنامه‌ها |
| `an info <name>` | - | نمایش جزئیات برنامه |
| `an sync` | - | همگام‌سازی پایگاه داده از GitHub |

## معماری

```
┌───────────────────────────────────────────────────────────────┐
│                         لایه CLI                               │
│                    (ماکروهای clap derive)                     │
├───────────────────────────────────────────────────────────────┤
│                       لایه دستورات                            │
│  install │ remove │ link │ update │ list │ search │ info │ sync │
├───────────────────────────────────────────────────────────────┤
│                       لایه هندلرها                            │
│            deb │ appimage │ flatpak │ remote                   │
├───────────────────────────────────────────────────────────────┤
│                       لایه پایگاه داده                        │
│                   پایگاه داده برنامه‌های TOML                │
└───────────────────────────────────────────────────────────────┘
```

## ساختار دایرکتوری

| هدف | مسیر |
|-----|------|
| فایل اجرایی | `/usr/local/bin/an` یا `~/.local/bin/an` |
| ذخیره‌سازی AppImage | `~/Applications/` |
| لینک‌های نمادین | `~/.local/bin/` |
| پیکربندی/حافظه نهان | `~/.config/an/` |

## توسعه

### نیازمندی‌ها

- Rust 1.70+
- Linux (مبتنی بر Debian/Ubuntu)

### ساخت

```bash
# ساخت توسعه
cargo build

# اجرای تست‌ها
cargo test

# ساخت نهایی
cargo build --release
```

### ساختار پروژه

```
AN/
├── src/                 # کد منبع Rust
│   ├── commands/        # پیاده‌سازی زیردستورات
│   ├── handlers/        # هندلرهای فرمت بسته
│   ├── db/              # لایه پایگاه داده برنامه‌ها
│   └── utils/           # ابزارها
├── apps/                # پایگاه داده برنامه‌ها (TOML)
├── docs/                # مستندات
│   ├── spec/            # مشخصات ویژگی‌ها
│   ├── design/          # طراحی و نمودارهای حالت
│   └── guides/          # راهنماهای کاربر
└── tests/               # تست‌ها
```

## مستندات

- [مشخصات ویژگی‌ها](../spec/README.md)
- [اسناد طراحی](../design/README.md)
- [راهنماهای کاربر](../guides/README.md)
- [CLAUDE.md](../../CLAUDE.md) - راهنمای توسعه‌دهنده

## مجوز

مجوز MIT - برای جزئیات به [LICENSE](../../LICENSE) مراجعه کنید

## قدردانی

این پروژه از [AM (App Manager)](https://github.com/ivan-hc/AM) الهام گرفته شده است.
