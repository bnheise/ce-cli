use crate::error::CliError;
use colored::Colorize;
use reqwest::Url;
use serde::Deserialize;
use std::collections::HashMap;

pub const NPM_PACKAGE_REGISTRY: &str = "https://registry.npmjs.org";
pub const PACKAGE_NAME: &str = env!("CARGO_PKG_NAME");

pub fn run_version_check() -> Result<(), CliError> {
    let metadata = fetch_package_metadata()?;
    let latest_version = metadata.version;
    let current_version = env!("CARGO_PKG_VERSION");
    let new_version_available_message =
        format!("A new version of {PACKAGE_NAME} is available.").bold();
    if current_version != latest_version {
        println!("\n ------------------------------------------------------ \n");
        println!("{new_version_available_message}");
        println!("Currently installed version: {}", current_version.red());
        println!("Latest version: {}", latest_version.green());
        println!("Run 'npm install --global {PACKAGE_NAME}' to upgrade.");
        println!("If you have Rust installed, you can also use 'cargo install {PACKAGE_NAME}'\n");
        println!(" ------------------------------------------------------ ");
    }

    Ok(())
}

fn fetch_package_metadata() -> Result<PackageMetadata, CliError> {
    let fetch_url = Url::parse(&format!("{NPM_PACKAGE_REGISTRY}/{PACKAGE_NAME}/latest"))
        .expect("Could not parse the npm package registry url");

    let resp = reqwest::blocking::get(fetch_url)
        .map_err(|e| {
            return CliError::Http(format!("Failed to get latest {PACKAGE_NAME} version"), e);
        })?
        .json::<PackageMetadata>()
        .map_err(|e| CliError::Http("Failed to deserialized ce-cli npm metadata".into(), e))?;
    Ok(resp)
}

#[derive(Debug, Deserialize)]
struct PackageMetadata {
    version: String,
    #[serde(flatten)]
    _extra: HashMap<String, serde_json::Value>,
}
