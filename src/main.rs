use crate::{commands::init::handle_init, config::ConfigBuilder};
use clap::Parser;
use cli::Cli;
use std::io::Result;

mod cli;
mod commands;
mod config;
mod structs;
mod templates;

fn main() -> Result<()> {
    let cli = Cli::parse();
    let config = ConfigBuilder::new();

    match cli.command {
        cli::Commands::Init {
            project_name,
            bundle_path,
            config_path,
        } => handle_init(config, project_name, bundle_path, config_path)?,
        cli::Commands::Add { list } => todo!(),
    };
    let thing = "";
    Ok(())
}
