//! CLI定義モジュール
//!
//! clapを使用したコマンドライン引数のパースを定義します。

use clap::{Parser, Subcommand};

/// AN (安装) - Unified Package Manager for Linux
#[derive(Parser)]
#[command(name = "an")]
#[command(author = "clearclown")]
#[command(version)]
#[command(about = "AN - Unified Package Manager for Linux", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// アプリをインストール
    #[command(visible_alias = "i")]
    Install {
        /// ファイルパスまたはアプリ名
        target: String,
    },

    /// アプリを削除（パージ）
    #[command(visible_aliases = ["rm", "uninstall"])]
    Remove {
        /// 削除するアプリ名
        target: String,
    },

    /// Flatpakエイリアスを生成
    #[command(visible_alias = "l")]
    Link,

    /// ANとアプリDBを更新
    Update,
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;

    #[test]
    fn test_cli_parsing() {
        // CLIが正しく構築されることを確認
        Cli::command().debug_assert();
    }

    #[test]
    fn test_install_command() {
        let cli = Cli::parse_from(["an", "install", "firefox"]);
        match cli.command {
            Commands::Install { target } => assert_eq!(target, "firefox"),
            _ => panic!("Expected Install command"),
        }
    }

    #[test]
    fn test_install_alias() {
        let cli = Cli::parse_from(["an", "i", "firefox"]);
        match cli.command {
            Commands::Install { target } => assert_eq!(target, "firefox"),
            _ => panic!("Expected Install command"),
        }
    }

    #[test]
    fn test_remove_command() {
        let cli = Cli::parse_from(["an", "remove", "firefox"]);
        match cli.command {
            Commands::Remove { target } => assert_eq!(target, "firefox"),
            _ => panic!("Expected Remove command"),
        }
    }

    #[test]
    fn test_remove_alias_rm() {
        let cli = Cli::parse_from(["an", "rm", "firefox"]);
        match cli.command {
            Commands::Remove { target } => assert_eq!(target, "firefox"),
            _ => panic!("Expected Remove command"),
        }
    }

    #[test]
    fn test_remove_alias_uninstall() {
        let cli = Cli::parse_from(["an", "uninstall", "firefox"]);
        match cli.command {
            Commands::Remove { target } => assert_eq!(target, "firefox"),
            _ => panic!("Expected Remove command"),
        }
    }

    #[test]
    fn test_link_command() {
        let cli = Cli::parse_from(["an", "link"]);
        assert!(matches!(cli.command, Commands::Link));
    }

    #[test]
    fn test_update_command() {
        let cli = Cli::parse_from(["an", "update"]);
        assert!(matches!(cli.command, Commands::Update));
    }
}
