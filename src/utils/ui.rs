//! UIユーティリティ
//!
//! カラー出力、プロンプトなどのUI関連機能を提供します。

use colored::*;
use std::io::{self, Write};

/// 情報メッセージを出力
pub fn info(message: &str) {
    println!("{}", message);
}

/// 成功メッセージを出力
pub fn success(message: &str) {
    println!("{} {}", "✓".green(), message.green());
}

/// 警告メッセージを出力
pub fn warn(message: &str) {
    eprintln!("{} {}", "Warning:".yellow(), message);
}

/// エラーメッセージを出力
pub fn error(message: &str) {
    eprintln!("{} {}", "Error:".red(), message);
}

/// ユーザー確認プロンプト
pub fn confirm(message: &str) -> io::Result<bool> {
    print!("{} [y/N]: ", message);
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let input = input.trim().to_lowercase();
    Ok(input == "y" || input == "yes")
}

/// プログレスバーを表示（簡易版）
pub fn progress(current: u64, total: u64) {
    let percentage = if total > 0 {
        (current as f64 / total as f64 * 100.0) as u32
    } else {
        0
    };

    let bar_width = 30;
    let filled = (percentage as usize * bar_width / 100).min(bar_width);
    let empty = bar_width - filled;

    print!(
        "\r[{}{}] {}%",
        "█".repeat(filled),
        "░".repeat(empty),
        percentage
    );
    io::stdout().flush().unwrap();

    if current >= total {
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_progress_percentage() {
        // プログレス計算のテスト（出力は確認しない）
        let percentage = (50u64 as f64 / 100u64 as f64 * 100.0) as u32;
        assert_eq!(percentage, 50);
    }
}
