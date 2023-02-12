use crate::structs::client_extension_yaml::PortletCategoryNames;
use clap::{Args, Parser, Subcommand, ValueEnum};
use serde::{Deserialize, Serialize};
use std::{fmt::Display, path::PathBuf};

/// ce-cli helps you bootstrap new frontend Client Extension projects {n}
/// for Liferay DXP and Liferay Experience Cloud (LXC). Automatically {n}
/// generate the required configuration files with a few keystrokes.  {n}
/// New projets come pre-configured with a variety of tools including {n}
/// webpack, eslint, prettier, Jest, Cypress, and TypeScript so you   {n}
/// can focus on what matters most: writing your extension.           {n}
#[derive(Parser)]
#[command(author, version, about, long_about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Init(InitArgs),

    Add {
        #[command(subcommand)]
        component: AddOption,
    },

    /// Deploy with urls pointing to your dev server. No need to remember to  {n}
    /// switch the port back in client-extension.yaml.
    #[command()]
    DevDeploy,
}

/// Initializes a new workspace. This should be carried out inside a  {n}
/// Liferay workspace for deployment to work.
#[derive(Debug, Args)]
pub struct InitArgs {
    /// Initializes a new workspace. This should be carried out inside a  {n}
    /// The name of the project. The default value is the name of the     {n}
    /// current folder
    #[arg(short, long)]
    pub project_name: Option<String>,

    /// The path to the client extension deploy folder in your local      {n}
    /// Liferay bundle. ce-cli uses this to point Liferay to your dev     {n}
    /// server.
    #[arg(short, long)]
    pub deploy_path: Option<PathBuf>,

    /// The framework to use. Currently only React is supported  but Vue  {n}
    /// and Angular are on the roadmap.
    #[arg(short, long, value_enum)]
    pub framework: Option<FrameworkOption>,

    /// The instance id that your apps will deploy to by default. Using   {n}
    /// the default value will deploy to your root instance.              {n}
    #[arg(short, long, value_enum)]
    pub instance_id: Option<String>,
}

/// Add a new component to your workspace. Components are items that will {n}
/// be deployed to your Liferay instance.
#[derive(Debug, Subcommand)]
pub enum AddOption {
    /// Add a Custom Element Client Extension to the project.
    #[command()]
    CustomElement(CustomElementArgs),

    /// A shared component is a component that will be shared between two {n}
    /// or more of your Custom Element extensions. You can configure it to{n}
    /// be bundled within your extension or separately.
    #[command()]
    SharedComponent {
        /// The human readable name of your app. This is what users will see  {n}
        /// in the Page Editor.
        #[arg()]
        name: String,
    },
}

#[derive(Debug, Args)]
pub struct CustomElementArgs {
    /// The human readable name of your app. This is what users will see  {n}
    /// in the Page Editor. If you don't specificy html-element-name and  {n}
    /// friendly-url-mapping, they will be genrated from what you enter   {n}
    /// here.
    #[arg()]
    pub name: String,

    /// The name of the custom html element that will host your app. If   {n}
    /// not present it will be generated from the 'name' parameter"
    #[arg(short = 'n', long)]
    pub html_element_name: Option<String>,

    /// Liferay uses this for directing route params to your app. Prefer  {n}
    /// to avoid using this and instead use query params. If not present, {n}
    /// it will be generated from the 'name' parameter"
    #[arg(short, long)]
    pub friendly_url_mapping: Option<String>,

    /// Indicated whether or not your app can be used multiple times on   {n}
    /// the same page. Defaults to false.
    #[arg(short, long)]
    pub instanceable: Option<bool>,

    /// Indicates under what menu in the page edtior your app will be     {n}
    /// found. Currently only Remote Apps is supported.
    #[arg(short, long)]
    pub portlet_category_name: Option<PortletCategoryNames>,

    /// Human readable description of your app. Will be displayed in the  {n}
    /// admin panel for reference.
    #[arg(short, long)]
    pub description: Option<String>,

    /// Determines whether ECMAScript modules will be used or not. This   {n}
    /// workspace's bundling strategy relies on ECMAScript modules, so    {n}
    /// setting this to false will break your build.
    #[arg(short, long)]
    pub use_esm: Option<bool>,

    /// A link to the repository where the sourcecode resides. This is    {n}
    /// only for documentation purposes.
    #[arg(short, long)]
    pub source_code_url: Option<String>,
}

#[derive(Debug, Clone, ValueEnum, Serialize, Deserialize, Copy, Default)]
pub enum FrameworkOption {
    #[default]
    React,
    Angular,
    Vue,
}

impl Display for FrameworkOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FrameworkOption::React => write!(f, "react"),
            FrameworkOption::Angular => write!(f, "angular"),
            FrameworkOption::Vue => write!(f, "vue"),
        }
    }
}
