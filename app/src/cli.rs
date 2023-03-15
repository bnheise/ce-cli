use crate::config_generators::client_extension_yaml::PortletCategoryNames;
use batch_api::reqwest::Url;
use clap::{ArgGroup, Args, Parser, Subcommand, ValueEnum};
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

    #[command()]
    Import(ImportArgs),

    #[command()]
    Export(ExportArgs),
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

    /// Username of the admin user of your Liferay instance. Used to      {n}
    /// import/export operations and the like.
    #[arg(short, long, value_enum)]
    pub username: Option<String>,

    /// Password for your liferay admin account.                          {n}
    #[arg(short, long, value_enum)]
    pub password: Option<String>,
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

    /// A shared dependency is an npm package that two or more of your    {n}
    /// custom element remote apps share. Using this option allos you to  {n}
    /// bundle this dependency separately so that the code is not         {n}
    /// duplicated in each bundle. This is best to use if the dependency  {n}
    /// is large.
    #[command()]
    SharedDependency {
        /// The name of the package you wish to bundle separately. It should  {n}
        /// the same as you would type when running `npm install`
        #[arg()]
        package: String,
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

#[derive(Debug, Clone, ValueEnum, Serialize, Deserialize, Copy, Default, PartialEq)]
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

impl From<FrameworkOption> for &str {
    fn from(value: FrameworkOption) -> Self {
        match value {
            FrameworkOption::React => "react",
            FrameworkOption::Angular => "angular",
            FrameworkOption::Vue => "vue",
        }
    }
}

#[derive(Debug, Args, Clone)]
#[command(group(
    ArgGroup::new("target")
        .required(true)
        .args(["objects", "picklists", "data"]).conflicts_with("all"),
))]
pub struct ImportArgs {
    /// The url for your Liferay instance. It can be local or remote. If {n}
    /// you don't provide this value, ce-cli will attempt to load it from{n}
    /// the LIFERAY_HOST environment variable.
    #[arg(long)]
    pub url: Option<Url>,

    /// The port for your Liferay instance. If not provided, ce-cli will {n}
    /// attempt to load it from the LIFERAY_PORT environment variable.
    #[arg(long)]
    pub port: Option<u16>,

    /// Setting this flag will import all Object definitions, picklists,  {n}
    /// and object data.
    #[arg(short, long)]
    pub all: bool,

    /// Setting this flag will import all Object definitions
    #[arg(short, long)]
    pub objects: bool,

    /// Setting this flag will import all picklists
    #[arg(short, long)]
    pub picklists: bool,

    /// Setting this flag will import object data.
    #[arg(short, long)]
    pub data: bool,

    /// Liferay user's email address who has access rights to requested   {n}
    /// data. If not provided, ce-cli will attempt to load it from the    {n}
    /// LIFERAY_USERNAME environment variable.
    #[arg(short, long, value_enum)]
    pub username: Option<String>,

    /// The password associated with the username parameter. if not       {n}
    /// provided, ce-cli will attempt to load this from the               {n}
    /// LIFERAY_PASSWORD environment variable.
    #[arg(short, long, value_enum)]
    pub password: Option<String>,

    /// Folder to store the output. If you are operating in a ce-cli     {n}
    /// generated workspace, this will default to                        {n}
    /// {root}/objects/definitions for Liferay Object defiitions,        {n}
    /// {root}/objects/picklists for Picklist definitions, and           {n}
    /// {root}/objects/data for object instance data.
    #[arg(short, long, value_enum)]
    pub output: Option<String>,
}

#[derive(Debug, Args)]
#[command(group(
    ArgGroup::new("target")
        .required(true)
        .args(["objects", "picklists"]).conflicts_with("data"),
))]
pub struct ExportArgs {
    /// The url for your Liferay instance. It can be local or remote. If {n}
    /// you don't provide this value, ce-cli will attempt to load it from{n}
    /// the LIFERAY_HOST environment variable.
    #[arg(long)]
    pub url: Option<Url>,

    /// The port for your Liferay instance. If not provided, ce-cli will {n}
    /// attempt to load it from the LIFERAY_PORT environment variable.
    #[arg(long)]
    pub port: Option<u16>,

    /// Setting this flag will export all Object definitions
    #[arg(short, long)]
    pub objects: bool,

    /// Setting this flag will export all picklists
    #[arg(short, long)]
    pub picklists: bool,

    /// Setting this flag will export object data. Note that objects and  {n}
    /// object data cannot be exported simultaneously -- you must export  {n}
    /// object data first, wait for the operation to complete, and then   {n}
    /// import the data.
    #[arg(short, long)]
    pub data: bool,

    /// Liferay user's email address who has permission to write the data.{n}
    /// If not provided, ce-cli will attempt to load it from the          {n}
    /// LIFERAY_USERNAME environment variable.
    #[arg(short, long, value_enum)]
    pub username: Option<String>,

    /// The password associated with the username parameter. if not       {n}
    /// provided, ce-cli will attempt to load this from the               {n}
    /// LIFERAY_PASSWORD environment variable.
    #[arg(short, long, value_enum)]
    pub password: Option<String>,

    /// Explicitly indicate the endpoint to send the data to. If empty,  {n}
    /// ce-cli will use the standard picklist or object definition       {n}
    /// endpoints for picklists and object definitions. In the case of   {n}
    /// object data, the endpoint is required.
    #[arg(short, long, value_enum)]
    pub endpoint: Option<String>,

    /// The directory where the item(s) to export are located. If caleld {n}
    /// from a ce-cli generated workspace, this will default to          {n}
    /// {root}/objects/definitions for Liferay Object defiitions,        {n}
    /// {root}/objects/picklists for Picklist definitions, and           {n}
    /// {root}/objects/data for object instance data.
    #[arg(short, long, value_enum)]
    pub directory: Option<String>,
    // TODO: add flag publish to automatically publish the objects after creation
}

#[derive(Debug, ValueEnum, Clone)]
pub enum ImportExportSource {
    Picklist,
    Definition,
    Data,
    DefAndPick,
}
