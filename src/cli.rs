use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Init {
        #[arg(short, long)]
        project_name: Option<String>,

        #[arg(short, long)]
        bundle_path: Option<PathBuf>,

        #[arg(short, long, value_name = "CONFIG")]
        config_path: Option<PathBuf>,
    },

    Add {
        #[arg(short, long)]
        list: bool,
    },
}
