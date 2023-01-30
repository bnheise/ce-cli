use crate::structs::client_extension_yaml::PortletCategoryNames;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    author,
    version,
    about,
    long_about = "
ce-cli reduces the amount of technical frontend knowldge required to develop Liferay Client Extensions.
Currently, quite a lot of technical knowledge is required to get started, and beginners are apt to put together 
a suboptimal solution. This CLI automatically generates Client Extension projects so that frontend devs can focus
on developing their solution without having to worry about whether or not they're getting the setup right."
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    #[command(
        about = "Initializes a new workspace. This should be carried out inside a Liferay workspace for deployment to work"
    )]
    Init {
        #[arg(
            short,
            long,
            help = "The name of the project. The default value is the name of the current folder"
        )]
        project_name: Option<String>,

        #[arg(
            short,
            long,
            help = "The path to your local Liferay bundle. Currently this value does nothing."
        )]
        bundle_path: Option<PathBuf>,

        #[arg(
            short,
            long,
            value_name = "CONFIG",
            help = "Specify a path to an existing workspace-config.json file."
        )]
        config_path: Option<PathBuf>,
    },

    #[command(
        about = "Add a Client Extension to the project. Currently only Custom Element apps are supported."
    )]
    Add {
        #[command(subcommand)]
        extension_type: ClientExtType,
    },
}

#[derive(Debug, Subcommand)]
pub enum ClientExtType {
    #[command(
        about = "Add a Custom Element Client Extension to the project. Other types will be added in the future."
    )]
    CustomElement {
        #[arg(
            help = "The human readable name of your app. This is what users will see in the Page Editor"
        )]
        name: String,
        #[arg(
            short = 'n',
            long,
            help = "The name of the custom html element that will host your app. If not present,
it will be generated from the 'name' parameter"
        )]
        html_element_name: Option<String>,
        #[arg(
            short,
            long,
            help = "Liferay uses this for directing route params to your app. Prefer to avoid
using this and instead use query params. If not present, it will be generated from the 'name' parameter"
        )]
        friendly_url_mapping: Option<String>,
        #[arg(
            short,
            long,
            help = "Indicated whether or not your app can be used multiple times on the same page.
Defaults to false."
        )]
        instanceable: Option<bool>,
        #[arg(
            short,
            long,
            help = "Indicates under what menu in the page edtior your app will be found. Currently
only Remote Apps is supported."
        )]
        portlet_category_name: Option<PortletCategoryNames>,
        #[arg(
            short,
            long,
            help = "Human readable description of your app. Will be displayed in the admin panel for reference."
        )]
        description: Option<String>,
        #[arg(
            short,
            long,
            help = "Determines whether ECMAScript modules will be used or not. Defaults to true."
        )]
        use_esm: Option<bool>,
    },
}
