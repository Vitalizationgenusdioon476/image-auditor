mod scanner;
mod tui;

use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "image-auditor", version = "0.1.0", author, about)]
struct Cli {
    /// Path to scan
    #[arg(value_name = "PATH")]
    path: Option<PathBuf>,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    tui::run(cli.path)?;
    Ok(())
}
