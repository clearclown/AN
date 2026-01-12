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
        Commands::Install { target } => {
            commands::install::run(&target)?;
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
    }

    Ok(())
}
