use crate::{commands::init::handle_init, structs::config::ConfigBuilder};
use clap::Parser;
use cli::{Cli, InitArgs};
use commands::{add::handle_add, dev_deploy::handle_dev_deploy};
use std::{error::Error, io::Result};

mod assets_dir;
mod cli;
mod commands;
mod error;
mod structs;
mod util;
mod version_check;

fn main() -> Result<()> {
    let cli = Cli::parse();

    let result = match cli.command {
        cli::Commands::Init(init_args) => handle_init(init_args),
        cli::Commands::Add { component } => handle_add(component),
        cli::Commands::DevDeploy => handle_dev_deploy(),
    };

    if let Err(error) = result {
        println!("{error}");

        #[cfg(debug_assertions)]
        if let Some(source) = error.source() {
            println!("{source}")
        }
    }

    Ok(())
}
