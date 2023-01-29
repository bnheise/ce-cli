use crate::structs::client_extension_yaml::PortletCategoryNames;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
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
        #[command(subcommand)]
        extension_type: ClientExtType,
    },
}

#[derive(Debug, Subcommand)]
pub enum ClientExtType {
    RemoteApp {
        name: String,
        #[arg(short = 'n', long)]
        html_element_name: Option<String>,
        #[arg(short, long)]
        friendly_url_mapping: Option<String>,
        #[arg(short, long)]
        instanceable: Option<bool>,
        #[arg(short, long)]
        portlet_category_name: Option<PortletCategoryNames>,
        #[arg(short, long)]
        description: Option<String>,
        #[arg(short, long)]
        use_esm: Option<bool>,
    },
}
