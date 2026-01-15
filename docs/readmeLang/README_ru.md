# AN (安装) - Унифицированный менеджер пакетов для Linux

[![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/License-MIT-blue.svg)](../../LICENSE)

[English](../../README.md) | [日本語](README_ja.md) | [简体中文](README_zh-CN.md) | [繁體中文](README_zh-TW.md) | **Русский** | [Українська](README_uk.md) | [فارسی](README_fa.md) | [العربية](README_ar.md)

**AN** — это унифицированный менеджер пакетов для дистрибутивов Linux на базе Debian/Ubuntu.

Управляйте пакетами `.deb`, `AppImage` и `Flatpak` через единый интерфейс, поддерживая систему в чистоте.

## Особенности

- **Единый интерфейс**: Управление всеми форматами пакетов с помощью `an install` / `an remove`
- **Полное удаление**: Полная очистка конфигурационных файлов, зависимостей и пользовательских данных
- **Прозрачность**: Отображение URL для удалённой установки и запрос подтверждения пользователя
- **Алиасы Flatpak**: Запуск `flatpak run org.gimp.GIMP` просто как `gimp`

## Установка

### Одной командой (рекомендуется)

```bash
curl -fsSL https://raw.githubusercontent.com/clearclown/AN/main/install.sh | bash
```

### Cargo (для пользователей Rust)

```bash
# Из crates.io
cargo install an-installer

# Напрямую из GitHub
cargo install --git https://github.com/clearclown/AN
```

### Arch Linux (AUR)

```bash
# yay (стабильная версия)
yay -S an-installer

# paru (стабильная версия)
paru -S an-installer

# Версия для разработчиков
yay -S an-installer-git
paru -S an-installer-git
```

### Nix

```bash
# Используя Flakes
nix profile install github:clearclown/AN

# Временное использование
nix run github:clearclown/AN -- --help
```

### Homebrew (Linux)

```bash
brew tap clearclown/an
brew install an
```

### apt (Debian/Ubuntu PPA)

```bash
# Добавить PPA (скоро)
sudo add-apt-repository ppa:clearclown/an
sudo apt update
sudo apt install an-installer
```

### Snap

```bash
# Из Snap Store (скоро)
sudo snap install an-installer --classic
```

### Ручная загрузка

Скачайте бинарные файлы из [Releases](https://github.com/clearclown/AN/releases):

| Архитектура | Имя файла |
|-------------|-----------|
| x86_64 | `an-linux-x86_64` |
| aarch64 (ARM64) | `an-linux-aarch64` |

```bash
# Пример: x86_64
wget https://github.com/clearclown/AN/releases/latest/download/an-linux-x86_64
chmod +x an-linux-x86_64
sudo mv an-linux-x86_64 /usr/local/bin/an
```

### Сборка из исходников

```bash
git clone https://github.com/clearclown/AN
cd AN
cargo build --release
sudo cp target/release/an /usr/local/bin/
```

## Использование

### Установка приложений

```bash
# Установить удалённое приложение
an install firefox

# Установить локальный .deb файл
an install ~/Downloads/vscode.deb

# Установить локальный AppImage
an install ~/Downloads/Obsidian.AppImage
```

### Удаление приложений

```bash
# Полное удаление (включая конфигурационные файлы)
an remove firefox
an rm vscode
an uninstall obsidian
```

### Создание алиасов Flatpak

```bash
# Создать алиасы для всех приложений Flatpak
an link
```

### Обновление

```bash
# Обновить AN и базу данных приложений
an update
```

### Поиск и отображение информации

```bash
# Список установленных приложений
an list

# Поиск доступных приложений
an search browser
an search          # Список всех приложений

# Показать информацию о приложении
an info firefox

# Синхронизировать базу данных приложений с GitHub
an sync
```

## Справка по командам

| Команда | Алиасы | Описание |
|---------|--------|----------|
| `an install <target>` | `i` | Установить приложение |
| `an remove <target>` | `rm`, `uninstall` | Полностью удалить приложение |
| `an link` | `l` | Создать алиасы Flatpak |
| `an update` | - | Обновить AN и базу данных |
| `an list` | `ls` | Список установленных приложений |
| `an search [query]` | `s` | Поиск в базе данных приложений |
| `an info <name>` | - | Показать информацию о приложении |
| `an sync` | - | Синхронизировать базу данных с GitHub |

## Архитектура

```
┌───────────────────────────────────────────────────────────────┐
│                         Слой CLI                               │
│                    (макросы clap derive)                       │
├───────────────────────────────────────────────────────────────┤
│                       Командный слой                           │
│  install │ remove │ link │ update │ list │ search │ info │ sync │
├───────────────────────────────────────────────────────────────┤
│                       Слой обработчиков                        │
│            deb │ appimage │ flatpak │ remote                   │
├───────────────────────────────────────────────────────────────┤
│                       Слой базы данных                         │
│                   База данных приложений TOML                  │
└───────────────────────────────────────────────────────────────┘
```

## Структура каталогов

| Назначение | Путь |
|------------|------|
| Исполняемый файл | `/usr/local/bin/an` или `~/.local/bin/an` |
| Хранилище AppImage | `~/Applications/` |
| Символические ссылки | `~/.local/bin/` |
| Конфигурация/Кэш | `~/.config/an/` |

## Разработка

### Требования

- Rust 1.70+
- Linux (на базе Debian/Ubuntu)

### Сборка

```bash
# Сборка для разработки
cargo build

# Запуск тестов
cargo test

# Release сборка
cargo build --release
```

### Структура проекта

```
AN/
├── src/                 # Исходный код Rust
│   ├── commands/        # Реализация подкоманд
│   ├── handlers/        # Обработчики форматов пакетов
│   ├── db/              # Слой базы данных приложений
│   └── utils/           # Утилиты
├── apps/                # База данных приложений (TOML)
├── docs/                # Документация
│   ├── spec/            # Спецификации функций
│   ├── design/          # Дизайн и диаграммы состояний
│   └── guides/          # Руководства пользователя
└── tests/               # Тесты
```

## Документация

- [Спецификации функций](../spec/README.md)
- [Документы по дизайну](../design/README.md)
- [Руководства пользователя](../guides/README.md)
- [CLAUDE.md](../../CLAUDE.md) - Руководство разработчика

## Лицензия

Лицензия MIT - подробности см. в [LICENSE](../../LICENSE)

## Благодарности

Этот проект вдохновлён [AM (App Manager)](https://github.com/ivan-hc/AM).
