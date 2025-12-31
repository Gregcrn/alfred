use colored::*;

pub fn info(msg: &str) {
    println!("{} {}", "ℹ️ INFO:".blue().bold(), msg);
}

pub fn success(msg: &str) {
    println!("{} {}", "✅ SUCCESS:".green().bold(), msg);
}

pub fn warn(msg: &str) {
    println!("{} {}", "⚠️ WARN:".yellow().bold(), msg);
}

pub fn error(msg: &str) {
    eprintln!("{} {}", "❌ ERROR:".red().bold(), msg);
}

pub fn highlight(msg: &str) -> String {
    msg.cyan().bold().to_string()
}
