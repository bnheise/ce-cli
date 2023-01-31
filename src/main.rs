use crate::{commands::init::handle_init, config::ConfigBuilder};
use clap::Parser;
use cli::Cli;
use commands::{add::handle_add, dev_deploy::handle_dev_deploy};
use std::io::Result;

mod cli;
mod commands;
mod config;
mod structs;
mod templates;
mod util;

fn main() -> Result<()> {
    let cli = Cli::parse();
    let config = ConfigBuilder::new();

    match cli.command {
        cli::Commands::Init {
            project_name,
            bundle_path,
            config_path,
        } => handle_init(config, project_name, bundle_path, config_path)?,
        cli::Commands::Add { extension_type } => handle_add(extension_type)?,
        cli::Commands::DevDeploy => handle_dev_deploy()?,
    };

    Ok(())
}
