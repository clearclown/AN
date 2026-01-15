# AN (安装) - Уніфікований менеджер пакетів для Linux

[![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/License-MIT-blue.svg)](../../LICENSE)

[English](../../README.md) | [日本語](README_ja.md) | [简体中文](README_zh-CN.md) | [繁體中文](README_zh-TW.md) | [Русский](README_ru.md) | **Українська** | [فارسی](README_fa.md) | [العربية](README_ar.md)

**AN** — це уніфікований менеджер пакетів для дистрибутивів Linux на базі Debian/Ubuntu.

Керуйте пакетами `.deb`, `AppImage` та `Flatpak` через єдиний інтерфейс, підтримуючи систему в чистоті.

## Особливості

- **Єдиний інтерфейс**: Керування всіма форматами пакетів за допомогою `an install` / `an remove`
- **Повне видалення**: Повне очищення конфігураційних файлів, залежностей та користувацьких даних
- **Прозорість**: Відображення URL для віддаленого встановлення та запит підтвердження користувача
- **Аліаси Flatpak**: Запуск `flatpak run org.gimp.GIMP` просто як `gimp`

## Встановлення

### Однією командою (рекомендовано)

```bash
curl -fsSL https://raw.githubusercontent.com/clearclown/AN/main/install.sh | bash
```

### Cargo (для користувачів Rust)

```bash
# З crates.io
cargo install an-installer

# Безпосередньо з GitHub
cargo install --git https://github.com/clearclown/AN
```

### Arch Linux (AUR)

```bash
# yay (стабільна версія)
yay -S an-installer

# paru (стабільна версія)
paru -S an-installer

# Версія для розробників
yay -S an-installer-git
paru -S an-installer-git
```

### Nix

```bash
# Використовуючи Flakes
nix profile install github:clearclown/AN

# Тимчасове використання
nix run github:clearclown/AN -- --help
```

### Homebrew (Linux)

```bash
brew tap clearclown/an
brew install an
```

### apt (Debian/Ubuntu PPA)

```bash
# Додати PPA (незабаром)
sudo add-apt-repository ppa:clearclown/an
sudo apt update
sudo apt install an-installer
```

### Snap

```bash
# Зі Snap Store (незабаром)
sudo snap install an-installer --classic
```

### Ручне завантаження

Завантажте бінарні файли з [Releases](https://github.com/clearclown/AN/releases):

| Архітектура | Ім'я файлу |
|-------------|------------|
| x86_64 | `an-linux-x86_64` |
| aarch64 (ARM64) | `an-linux-aarch64` |

```bash
# Приклад: x86_64
wget https://github.com/clearclown/AN/releases/latest/download/an-linux-x86_64
chmod +x an-linux-x86_64
sudo mv an-linux-x86_64 /usr/local/bin/an
```

### Збірка з вихідних кодів

```bash
git clone https://github.com/clearclown/AN
cd AN
cargo build --release
sudo cp target/release/an /usr/local/bin/
```

## Використання

### Встановлення додатків

```bash
# Встановити віддалений додаток
an install firefox

# Встановити локальний .deb файл
an install ~/Downloads/vscode.deb

# Встановити локальний AppImage
an install ~/Downloads/Obsidian.AppImage
```

### Видалення додатків

```bash
# Повне видалення (включаючи конфігураційні файли)
an remove firefox
an rm vscode
an uninstall obsidian
```

### Створення аліасів Flatpak

```bash
# Створити аліаси для всіх додатків Flatpak
an link
```

### Оновлення

```bash
# Оновити AN та базу даних додатків
an update
```

### Пошук та відображення інформації

```bash
# Список встановлених додатків
an list

# Пошук доступних додатків
an search browser
an search          # Список всіх додатків

# Показати інформацію про додаток
an info firefox

# Синхронізувати базу даних додатків з GitHub
an sync
```

## Довідка по командах

| Команда | Аліаси | Опис |
|---------|--------|------|
| `an install <target>` | `i` | Встановити додаток |
| `an remove <target>` | `rm`, `uninstall` | Повністю видалити додаток |
| `an link` | `l` | Створити аліаси Flatpak |
| `an update` | - | Оновити AN та базу даних |
| `an list` | `ls` | Список встановлених додатків |
| `an search [query]` | `s` | Пошук у базі даних додатків |
| `an info <name>` | - | Показати інформацію про додаток |
| `an sync` | - | Синхронізувати базу даних з GitHub |

## Архітектура

```
┌───────────────────────────────────────────────────────────────┐
│                         Шар CLI                                │
│                    (макроси clap derive)                       │
├───────────────────────────────────────────────────────────────┤
│                       Командний шар                            │
│  install │ remove │ link │ update │ list │ search │ info │ sync │
├───────────────────────────────────────────────────────────────┤
│                       Шар обробників                           │
│            deb │ appimage │ flatpak │ remote                   │
├───────────────────────────────────────────────────────────────┤
│                       Шар бази даних                           │
│                   База даних додатків TOML                     │
└───────────────────────────────────────────────────────────────┘
```

## Структура каталогів

| Призначення | Шлях |
|-------------|------|
| Виконуваний файл | `/usr/local/bin/an` або `~/.local/bin/an` |
| Сховище AppImage | `~/Applications/` |
| Символічні посилання | `~/.local/bin/` |
| Конфігурація/Кеш | `~/.config/an/` |

## Розробка

### Вимоги

- Rust 1.70+
- Linux (на базі Debian/Ubuntu)

### Збірка

```bash
# Збірка для розробки
cargo build

# Запуск тестів
cargo test

# Release збірка
cargo build --release
```

### Структура проєкту

```
AN/
├── src/                 # Вихідний код Rust
│   ├── commands/        # Реалізація підкоманд
│   ├── handlers/        # Обробники форматів пакетів
│   ├── db/              # Шар бази даних додатків
│   └── utils/           # Утиліти
├── apps/                # База даних додатків (TOML)
├── docs/                # Документація
│   ├── spec/            # Специфікації функцій
│   ├── design/          # Дизайн та діаграми станів
│   └── guides/          # Посібники користувача
└── tests/               # Тести
```

## Документація

- [Специфікації функцій](../spec/README.md)
- [Документи по дизайну](../design/README.md)
- [Посібники користувача](../guides/README.md)
- [CLAUDE.md](../../CLAUDE.md) - Посібник розробника

## Ліцензія

Ліцензія MIT - подробиці див. у [LICENSE](../../LICENSE)

## Подяки

Цей проєкт натхненний [AM (App Manager)](https://github.com/ivan-hc/AM).
