use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long)]
    pub project_name: Option<String>,

    #[arg(short, long)]
    pub bundle_path: Option<PathBuf>,

    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,
}
