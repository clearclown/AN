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

        /// カスタムコマンド名を指定（AppImage用）
        #[arg(short, long)]
        name: Option<String>,

        /// デスクトップエントリを作成（AppImage用）
        #[arg(short, long)]
        desktop: bool,

        /// 元ファイルを削除（移動モード、AppImage用）
        #[arg(short = 'm', long = "move")]
        move_file: bool,
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

    /// インストール済みアプリ一覧
    #[command(visible_alias = "ls")]
    List,

    /// アプリDBを検索
    #[command(visible_alias = "s")]
    Search {
        /// 検索クエリ（アプリ名または説明）
        query: Option<String>,
    },

    /// アプリ詳細を表示
    Info {
        /// アプリ名
        name: String,
    },

    /// アプリDBをGitHubから同期
    Sync,
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
            Commands::Install { target, name, desktop, move_file } => {
                assert_eq!(target, "firefox");
                assert!(name.is_none());
                assert!(!desktop);
                assert!(!move_file);
            },
            _ => panic!("Expected Install command"),
        }
    }

    #[test]
    fn test_install_alias() {
        let cli = Cli::parse_from(["an", "i", "firefox"]);
        match cli.command {
            Commands::Install { target, .. } => assert_eq!(target, "firefox"),
            _ => panic!("Expected Install command"),
        }
    }

    #[test]
    fn test_install_with_options() {
        let cli = Cli::parse_from(["an", "install", "app.AppImage", "-n", "myapp", "-d", "-m"]);
        match cli.command {
            Commands::Install { target, name, desktop, move_file } => {
                assert_eq!(target, "app.AppImage");
                assert_eq!(name, Some("myapp".to_string()));
                assert!(desktop);
                assert!(move_file);
            },
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

    #[test]
    fn test_list_command() {
        let cli = Cli::parse_from(["an", "list"]);
        assert!(matches!(cli.command, Commands::List));
    }

    #[test]
    fn test_list_alias() {
        let cli = Cli::parse_from(["an", "ls"]);
        assert!(matches!(cli.command, Commands::List));
    }

    #[test]
    fn test_search_command() {
        let cli = Cli::parse_from(["an", "search", "firefox"]);
        match cli.command {
            Commands::Search { query } => assert_eq!(query, Some("firefox".to_string())),
            _ => panic!("Expected Search command"),
        }
    }

    #[test]
    fn test_search_without_query() {
        let cli = Cli::parse_from(["an", "search"]);
        match cli.command {
            Commands::Search { query } => assert!(query.is_none()),
            _ => panic!("Expected Search command"),
        }
    }

    #[test]
    fn test_info_command() {
        let cli = Cli::parse_from(["an", "info", "firefox"]);
        match cli.command {
            Commands::Info { name } => assert_eq!(name, "firefox"),
            _ => panic!("Expected Info command"),
        }
    }

    #[test]
    fn test_sync_command() {
        let cli = Cli::parse_from(["an", "sync"]);
        assert!(matches!(cli.command, Commands::Sync));
    }
}
