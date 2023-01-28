use clap::{Error, Parser};
use cli::Cli;
use config::{get_project_name_from_user, Config};

use crate::config::{get_bundle_path_from_environment, get_bundle_path_from_user};

mod cli;
mod config;

fn main() -> Result<(), Error> {
    let cli = Cli::parse();
    let mut config = Config::new();

    if let Some(project_name) = cli.project_name {
        config.set_project_name(project_name);
    } else {
        let project_name = get_project_name_from_user()?;
        config.set_project_name(project_name);
    }

    if let Some(bundle_path) = cli.bundle_path {
        config.set_bundle_path(bundle_path);
    } else {
        let bundle_path = get_bundle_path_from_environment();
        let bundle_path = get_bundle_path_from_user(bundle_path);
        config.set_bundle_path(bundle_path);
    }
    println!("{:?}", config);
    let config = config.build();
    println!("{:?}", config);
    Ok(())
}
