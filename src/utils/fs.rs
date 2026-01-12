//! ファイルシステムユーティリティ
//!
//! ファイル操作のヘルパー関数を提供します。

use anyhow::Result;
use std::path::Path;

/// ファイルを削除
pub fn remove_file(path: &Path) -> Result<()> {
    if path.exists() {
        std::fs::remove_file(path)?;
    }
    Ok(())
}

/// ディレクトリを再帰的に削除
pub fn remove_dir_all(path: &Path) -> Result<()> {
    if path.exists() {
        std::fs::remove_dir_all(path)?;
    }
    Ok(())
}

/// ディレクトリを作成（存在しない場合）
pub fn ensure_dir(path: &Path) -> Result<()> {
    if !path.exists() {
        std::fs::create_dir_all(path)?;
    }
    Ok(())
}

/// ファイルが存在するか確認
pub fn file_exists(path: &Path) -> bool {
    path.exists() && path.is_file()
}

/// ディレクトリが存在するか確認
pub fn dir_exists(path: &Path) -> bool {
    path.exists() && path.is_dir()
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_ensure_dir() {
        let temp = tempdir().unwrap();
        let new_dir = temp.path().join("test_dir");

        assert!(!new_dir.exists());
        ensure_dir(&new_dir).unwrap();
        assert!(new_dir.exists());
    }

    #[test]
    fn test_file_exists() {
        let temp = tempdir().unwrap();
        let file_path = temp.path().join("test.txt");

        assert!(!file_exists(&file_path));
        std::fs::write(&file_path, "test").unwrap();
        assert!(file_exists(&file_path));
    }

    #[test]
    fn test_dir_exists() {
        let temp = tempdir().unwrap();
        assert!(dir_exists(temp.path()));
        assert!(!dir_exists(&temp.path().join("nonexistent")));
    }

    #[test]
    fn test_remove_file() {
        let temp = tempdir().unwrap();
        let file_path = temp.path().join("test.txt");

        std::fs::write(&file_path, "test").unwrap();
        assert!(file_path.exists());

        remove_file(&file_path).unwrap();
        assert!(!file_path.exists());
    }
}
