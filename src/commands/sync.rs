//! sync コマンド実装
//!
//! GitHubからアプリデータベースを同期・更新します。

use crate::utils::ui;
use anyhow::Result;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

/// GitHubリポジトリURL
const REPO_URL: &str = "https://github.com/clearclown/AN.git";

/// アプリDBディレクトリを取得
fn db_dir() -> PathBuf {
    if let Ok(dir) = std::env::var("AN_DB_DIR") {
        return PathBuf::from(dir);
    }

    if let Some(config_dir) = dirs::config_dir() {
        return config_dir.join("an").join("apps");
    }

    PathBuf::from("apps")
}

/// syncコマンドのエントリーポイント
pub fn run() -> Result<()> {
    ui::info("アプリDBを同期中...\n");

    let db_directory = db_dir();

    // DBディレクトリが存在しない場合は作成
    if !db_directory.exists() {
        ui::info(&format!("DBディレクトリを作成: {:?}", db_directory));
        fs::create_dir_all(&db_directory)?;
    }

    // 一時ディレクトリにリポジトリをクローン
    let temp_dir = std::env::temp_dir().join("an-sync");
    if temp_dir.exists() {
        fs::remove_dir_all(&temp_dir)?;
    }

    ui::info("GitHubからデータを取得中...");

    // sparse checkoutでappsディレクトリのみ取得
    let clone_result = Command::new("git")
        .args([
            "clone",
            "--depth=1",
            "--filter=blob:none",
            "--sparse",
            REPO_URL,
            temp_dir.to_str().unwrap(),
        ])
        .output();

    match clone_result {
        Ok(output) if output.status.success() => {
            // sparse-checkoutでappsのみ展開
            let _ = Command::new("git")
                .args(["sparse-checkout", "set", "apps"])
                .current_dir(&temp_dir)
                .output();

            // appsディレクトリからファイルをコピー
            let source_apps = temp_dir.join("apps");
            if source_apps.exists() {
                let mut updated = 0;
                let mut added = 0;

                for entry in fs::read_dir(&source_apps)? {
                    let entry = entry?;
                    let path = entry.path();

                    if path.extension().map(|e| e == "toml").unwrap_or(false) {
                        let filename = path.file_name().unwrap();
                        let dest = db_directory.join(filename);

                        if dest.exists() {
                            // 既存ファイルの更新チェック
                            let src_content = fs::read_to_string(&path)?;
                            let dest_content = fs::read_to_string(&dest)?;

                            if src_content != dest_content {
                                fs::copy(&path, &dest)?;
                                updated += 1;
                            }
                        } else {
                            // 新規ファイル
                            fs::copy(&path, &dest)?;
                            added += 1;
                        }
                    }
                }

                ui::success(&format!(
                    "同期完了: {} 件追加, {} 件更新",
                    added, updated
                ));
            } else {
                ui::warn("appsディレクトリが見つかりません");
            }

            // 一時ディレクトリを削除
            let _ = fs::remove_dir_all(&temp_dir);
        }
        Ok(output) => {
            let stderr = String::from_utf8_lossy(&output.stderr);
            ui::warn(&format!("git clone失敗: {}", stderr));

            // フォールバック: curlで直接ダウンロード
            ui::info("代替方法でダウンロードを試行中...");
            sync_via_curl(&db_directory)?;
        }
        Err(e) => {
            ui::warn(&format!("gitが利用できません: {}", e));

            // フォールバック: curlで直接ダウンロード
            ui::info("代替方法でダウンロードを試行中...");
            sync_via_curl(&db_directory)?;
        }
    }

    // 同期後のアプリ数を表示
    let count = count_apps(&db_directory);
    ui::info(&format!("\n現在のアプリDB: {} 件", count));

    Ok(())
}

/// curlを使用してappsディレクトリをダウンロード
fn sync_via_curl(db_dir: &PathBuf) -> Result<()> {
    // 既知のアプリファイルをダウンロード
    let apps = [
        "firefox", "brave", "gimp", "vlc", "obsidian", "vscode",
        "discord", "keepassxc", "neovim", "audacity", "flameshot",
        "telegram", "thunderbird", "libreoffice",
    ];

    let base_url = "https://raw.githubusercontent.com/clearclown/AN/main/apps";

    for app in apps {
        let url = format!("{}/{}.toml", base_url, app);
        let dest = db_dir.join(format!("{}.toml", app));

        let output = Command::new("curl")
            .args(["-sf", "-o", dest.to_str().unwrap(), &url])
            .output();

        match output {
            Ok(o) if o.status.success() => {
                // 成功
            }
            _ => {
                // 個別のファイルが失敗しても続行
            }
        }
    }

    ui::success("ダウンロード完了");
    Ok(())
}

/// DBディレクトリ内のアプリ数をカウント
fn count_apps(db_dir: &PathBuf) -> usize {
    if !db_dir.exists() {
        return 0;
    }

    fs::read_dir(db_dir)
        .map(|entries| {
            entries
                .filter_map(|e| e.ok())
                .filter(|e| {
                    e.path()
                        .extension()
                        .map(|ext| ext == "toml")
                        .unwrap_or(false)
                })
                .count()
        })
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_db_dir() {
        let dir = db_dir();
        assert!(dir.to_string_lossy().contains("apps"));
    }

    #[test]
    fn test_count_apps_nonexistent() {
        let path = PathBuf::from("/nonexistent/path");
        assert_eq!(count_apps(&path), 0);
    }
}
