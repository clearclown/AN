# AN (安装) - مدير الحزم الموحد لنظام Linux

[![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/License-MIT-blue.svg)](../../LICENSE)

[English](../../README.md) | [日本語](README_ja.md) | [简体中文](README_zh-CN.md) | [繁體中文](README_zh-TW.md) | [Русский](README_ru.md) | [Українська](README_uk.md) | [فارسی](README_fa.md) | **العربية**

**AN** هو مدير حزم موحد لتوزيعات Linux المبنية على Debian/Ubuntu.

قم بإدارة حزم `.deb` و `AppImage` و `Flatpak` من خلال واجهة واحدة مع الحفاظ على نظافة نظامك.

## المميزات

- **واجهة موحدة**: إدارة جميع صيغ الحزم باستخدام `an install` / `an remove`
- **الإزالة الكاملة**: مسح ملفات التكوين والتبعيات وبيانات المستخدم بالكامل
- **الشفافية**: عرض عناوين URL للتثبيت عن بُعد وطلب تأكيد المستخدم
- **الأسماء المستعارة لـ Flatpak**: تشغيل `flatpak run org.gimp.GIMP` ببساطة كـ `gimp`

## التثبيت

### أمر واحد (موصى به)

```bash
curl -fsSL https://raw.githubusercontent.com/clearclown/AN/main/install.sh | bash
```

### Cargo (لمستخدمي Rust)

```bash
# من crates.io
cargo install an-installer

# مباشرة من GitHub
cargo install --git https://github.com/clearclown/AN
```

### Arch Linux (AUR)

```bash
# yay (النسخة المستقرة)
yay -S an-installer

# paru (النسخة المستقرة)
paru -S an-installer

# نسخة التطوير
yay -S an-installer-git
paru -S an-installer-git
```

### Nix

```bash
# استخدام Flakes
nix profile install github:clearclown/AN

# الاستخدام المؤقت
nix run github:clearclown/AN -- --help
```

### Homebrew (Linux)

```bash
brew tap clearclown/an
brew install an
```

### apt (Debian/Ubuntu PPA)

```bash
# إضافة PPA (قريباً)
sudo add-apt-repository ppa:clearclown/an
sudo apt update
sudo apt install an-installer
```

### Snap

```bash
# من Snap Store (قريباً)
sudo snap install an-installer --classic
```

### التنزيل اليدوي

قم بتنزيل الملفات الثنائية من [Releases](https://github.com/clearclown/AN/releases):

| البنية | اسم الملف |
|--------|-----------|
| x86_64 | `an-linux-x86_64` |
| aarch64 (ARM64) | `an-linux-aarch64` |

```bash
# مثال: x86_64
wget https://github.com/clearclown/AN/releases/latest/download/an-linux-x86_64
chmod +x an-linux-x86_64
sudo mv an-linux-x86_64 /usr/local/bin/an
```

### البناء من المصدر

```bash
git clone https://github.com/clearclown/AN
cd AN
cargo build --release
sudo cp target/release/an /usr/local/bin/
```

## الاستخدام

### تثبيت التطبيقات

```bash
# تثبيت تطبيق عن بُعد
an install firefox

# تثبيت ملف .deb محلي
an install ~/Downloads/vscode.deb

# تثبيت AppImage محلي
an install ~/Downloads/Obsidian.AppImage
```

### إزالة التطبيقات

```bash
# الإزالة الكاملة (بما في ذلك ملفات التكوين)
an remove firefox
an rm vscode
an uninstall obsidian
```

### إنشاء أسماء مستعارة لـ Flatpak

```bash
# إنشاء أسماء مستعارة لجميع تطبيقات Flatpak
an link
```

### التحديث

```bash
# تحديث AN وقاعدة بيانات التطبيقات
an update
```

### البحث وعرض المعلومات

```bash
# قائمة التطبيقات المثبتة
an list

# البحث عن التطبيقات المتاحة
an search browser
an search          # قائمة جميع التطبيقات

# عرض تفاصيل التطبيق
an info firefox

# مزامنة قاعدة بيانات التطبيقات من GitHub
an sync
```

## مرجع الأوامر

| الأمر | الأسماء المستعارة | الوصف |
|-------|-------------------|-------|
| `an install <target>` | `i` | تثبيت تطبيق |
| `an remove <target>` | `rm`, `uninstall` | إزالة تطبيق بالكامل |
| `an link` | `l` | إنشاء أسماء مستعارة لـ Flatpak |
| `an update` | - | تحديث AN وقاعدة البيانات |
| `an list` | `ls` | قائمة التطبيقات المثبتة |
| `an search [query]` | `s` | البحث في قاعدة بيانات التطبيقات |
| `an info <name>` | - | عرض تفاصيل التطبيق |
| `an sync` | - | مزامنة قاعدة البيانات من GitHub |

## البنية المعمارية

```
┌───────────────────────────────────────────────────────────────┐
│                         طبقة CLI                               │
│                    (وحدات ماكرو clap derive)                  │
├───────────────────────────────────────────────────────────────┤
│                       طبقة الأوامر                            │
│  install │ remove │ link │ update │ list │ search │ info │ sync │
├───────────────────────────────────────────────────────────────┤
│                       طبقة المعالجات                          │
│            deb │ appimage │ flatpak │ remote                   │
├───────────────────────────────────────────────────────────────┤
│                       طبقة قاعدة البيانات                     │
│                   قاعدة بيانات التطبيقات TOML                │
└───────────────────────────────────────────────────────────────┘
```

## بنية الدليل

| الغرض | المسار |
|-------|--------|
| الملف التنفيذي | `/usr/local/bin/an` أو `~/.local/bin/an` |
| تخزين AppImage | `~/Applications/` |
| الروابط الرمزية | `~/.local/bin/` |
| التكوين/ذاكرة التخزين المؤقت | `~/.config/an/` |

## التطوير

### المتطلبات

- Rust 1.70+
- Linux (مبني على Debian/Ubuntu)

### البناء

```bash
# بناء التطوير
cargo build

# تشغيل الاختبارات
cargo test

# بناء الإصدار
cargo build --release
```

### بنية المشروع

```
AN/
├── src/                 # كود مصدر Rust
│   ├── commands/        # تنفيذ الأوامر الفرعية
│   ├── handlers/        # معالجات صيغ الحزم
│   ├── db/              # طبقة قاعدة بيانات التطبيقات
│   └── utils/           # الأدوات المساعدة
├── apps/                # قاعدة بيانات التطبيقات (TOML)
├── docs/                # الوثائق
│   ├── spec/            # مواصفات الميزات
│   ├── design/          # التصميم ومخططات الحالة
│   └── guides/          # أدلة المستخدم
└── tests/               # الاختبارات
```

## الوثائق

- [مواصفات الميزات](../spec/README.md)
- [وثائق التصميم](../design/README.md)
- [أدلة المستخدم](../guides/README.md)
- [CLAUDE.md](../../CLAUDE.md) - دليل المطور

## الترخيص

ترخيص MIT - راجع [LICENSE](../../LICENSE) للتفاصيل

## الشكر والتقدير

هذا المشروع مستوحى من [AM (App Manager)](https://github.com/ivan-hc/AM).
