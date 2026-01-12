//! データベースモジュール
//!
//! TOMLアプリデータベースの管理を提供します。

pub mod app;

pub use app::{find_by_name, load, load_all, AppConfig, AppInfo, Metadata, SourceInfo, SourceType};
