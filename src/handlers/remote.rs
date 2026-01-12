//! リモートダウンロードハンドラ
//!
//! URLからファイルをダウンロードする処理を提供します。

use crate::errors::AnError;
use crate::utils::ui;
use anyhow::Result;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

/// ダウンロード先一時ディレクトリを取得
fn temp_dir() -> PathBuf {
    std::env::temp_dir().join("an-downloads")
}

/// URLからファイルをダウンロード
pub fn download(url: &str, filename: &str) -> Result<PathBuf> {
    ui::info(&format!("Downloading: {}", url));

    // 一時ディレクトリ作成
    let temp_directory = temp_dir();
    if !temp_directory.exists() {
        std::fs::create_dir_all(&temp_directory)?;
    }

    let dest_path = temp_directory.join(filename);

    // ダウンロード
    let response = reqwest::blocking::get(url).map_err(|_| AnError::DownloadFailed {
        url: url.to_string(),
    })?;

    if !response.status().is_success() {
        return Err(AnError::DownloadFailed {
            url: url.to_string(),
        }
        .into());
    }

    let bytes = response.bytes().map_err(|_| AnError::DownloadFailed {
        url: url.to_string(),
    })?;

    // ファイルに書き込み
    let mut file = File::create(&dest_path)?;
    file.write_all(&bytes)?;

    ui::success(&format!("Downloaded to: {:?}", dest_path));
    Ok(dest_path)
}

/// プログレスバー付きダウンロード
#[allow(dead_code)]
pub fn download_with_progress<F>(url: &str, filename: &str, _callback: F) -> Result<PathBuf>
where
    F: Fn(u64, u64),
{
    // TODO: プログレス表示の実装
    // 現在は通常のダウンロードにフォールバック
    download(url, filename)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_temp_dir() {
        let dir = temp_dir();
        assert!(dir.ends_with("an-downloads"));
    }
}
