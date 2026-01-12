//! AN (安装) - Unified Package Manager for Linux
//!
//! ANは.deb、AppImage、Flatpakを統一インターフェースで管理する
//! パッケージマネージャーです。

mod cli;
mod commands;
mod db;
mod errors;
mod handlers;
mod utils;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands};

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Install { target, name, desktop, move_file } => {
            let options = commands::install::InstallOptions {
                name,
                desktop,
                move_file,
            };
            commands::install::run_with_options(&target, options)?;
        }
        Commands::Remove { target } => {
            commands::remove::run(&target)?;
        }
        Commands::Link => {
            commands::link::run()?;
        }
        Commands::Update => {
            commands::update::run()?;
        }
        Commands::List => {
            commands::list::run()?;
        }
        Commands::Search { query } => {
            match query {
                Some(q) => commands::search::run(&q)?,
                None => commands::search::list_all()?,
            }
        }
        Commands::Info { name } => {
            commands::search::show_details(&name)?;
        }
    }

    Ok(())
}
