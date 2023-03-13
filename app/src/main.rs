use crate::commands::init::handle_init;
use clap::Parser;
use cli::Cli;
use commands::{add::handle_add, dev_deploy::handle_dev_deploy, object::handle_object};
use std::io::Result;

mod assets_dir;
mod cli;
mod commands;
mod config_generators;
mod data_dir;
mod error;
mod liferay_client;
mod util;
mod version_check;

// TODO: finish refactoring to use Liferay Client and DataDir

fn main() -> Result<()> {
    let cli = Cli::parse();

    let result = match cli.command {
        cli::Commands::Init(init_args) => handle_init(init_args),
        cli::Commands::Add { component } => handle_add(component),
        cli::Commands::DevDeploy => handle_dev_deploy(),
        cli::Commands::Object(args) => handle_object(args),
    };

    if let Err(error) = result {
        println!("{error}");

        #[cfg(debug_assertions)]
        {
            use std::error::Error;
            if let Some(source) = error.source() {
                println!("{source}")
            }
        }
    }

    Ok(())
}
