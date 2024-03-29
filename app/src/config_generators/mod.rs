use self::config::Config;
use crate::{cli::FrameworkOption, error::CliError};
use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::{
    fmt::Display,
    fs,
    path::{Path, PathBuf},
};
use yaml_rust::{YamlEmitter, YamlLoader};

pub mod cet_configuration;
pub mod client_extension_yaml;
pub mod config;
pub mod eslintrc;
pub mod package_json;
pub mod shared_component;
pub mod typescript_config_json;
pub mod typescript_config_prod_json;
pub mod webpack_common;

#[derive(Debug, PartialEq)]
pub enum ConfigFormat {
    Yaml,
    Json,
}

pub trait ConfigFile<'a>: Serialize + Deserialize<'a> {
    const FILENAME: &'static str;
    const FORMAT: ConfigFormat;

    fn get_filepath() -> PathBuf {
        PathBuf::from(Self::FILENAME)
    }

    fn try_open() -> Result<String, CliError> {
        let path = Self::get_filepath();
        let raw = fs::read_to_string(path)?;

        Ok(raw)
    }

    fn try_parse(raw: &'a str) -> Result<Self, CliError>
    where
        Self: Sized,
    {
        let parsed = match Self::FORMAT {
            ConfigFormat::Yaml => serde_yaml::from_str::<Self>(raw)?,
            ConfigFormat::Json => serde_json::from_str::<Self>(raw)?,
        };

        Ok(parsed)
    }

    fn try_serialize(config: Self) -> Result<String, CliError> {
        let raw = match Self::FORMAT {
            ConfigFormat::Yaml => {
                let raw = serde_yaml::to_string(&config)?;
                format_yaml(&raw)?
            }
            ConfigFormat::Json => serde_json::to_string_pretty(&config)?,
        };

        Ok(raw)
    }

    fn write(raw: String) -> Result<(), CliError> {
        let path = Self::get_filepath();

        fs::write(path, raw)?;
        Ok(())
    }

    fn add_project_settings<'b: 'a>(&mut self, config: &'b Config) -> Result<(), CliError>;
}

pub trait FrameworkConfigurable {
    fn set_framework_settings(&mut self, framework: FrameworkOption);
}

pub trait AppDir {
    const NAME: Option<&'static str> = None;

    fn get_path(parent_path: Option<PathBuf>) -> PathBuf {
        let base = if let Some(parent_path) = parent_path {
            parent_path
        } else {
            PathBuf::from("./")
        };

        if let Some(name) = Self::NAME {
            base.join(name)
        } else {
            base
        }
    }

    fn write_file(filename: &str, content: &str, subfolder: Option<&str>) -> Result<(), CliError> {
        let mut path = Self::get_path(None);

        if let Some(subfolder) = subfolder {
            path = path.join(subfolder);
        }

        fs::create_dir_all(&path)?;
        fs::write(path.join(filename), content)?;

        Ok(())
    }
}

pub struct ProjectRoot;

impl AppDir for ProjectRoot {
    const NAME: Option<&'static str> = Some("./");
}

pub struct BuildDir;

impl AppDir for BuildDir {
    const NAME: Option<&'static str> = Some("build");
}

pub struct SrcDir;

impl AppDir for SrcDir {
    const NAME: Option<&'static str> = Some("src");
}

pub struct UtilDir;

impl AppDir for UtilDir {
    const NAME: Option<&'static str> = Some("util");
}

pub struct DeployDir;

impl AppDir for DeployDir {}

fn format_yaml(raw: &str) -> Result<String, CliError> {
    let indent_hack = YamlLoader::load_from_str(raw).expect(
        "Should have been able to load yaml from string as it was already checked by serde_yaml",
    );

    let mut out_str = String::new();
    {
        let mut emitter = YamlEmitter::new(&mut out_str);
        indent_hack
            .iter()
            .for_each(|item| emitter.dump(item).unwrap()); // dump the YAML object to a String
    }

    Ok(out_str)
}

pub trait ClientExt {
    fn get_name(&self) -> &str;

    fn get_id(&self) -> String;

    fn get_ext_path(&self) -> PathBuf {
        Path::new("./src").join(self.get_id())
    }

    fn get_server_path(&self, config: &Config) -> String {
        let filename = [&self.get_id(), "js"].join(".");
        let parts = ["/o", &config.project_name, &filename];

        parts.join("/")
    }

    fn add_to_entrypoints(&self, config: &mut Config) {
        config
            .entrypoints
            .insert(self.get_id(), self.get_ext_path());
    }

    fn add_to_externals(&self, config: &mut Config) {
        config
            .externals
            .insert(self.get_id(), self.get_server_path(config));
    }

    fn initialize_directories(&self) -> Result<(), CliError> {
        let path = self.get_ext_path();
        if path.exists() {
            return Err(CliError::ExtensionExists);
        }
        fs::create_dir_all(path)?;

        Ok(())
    }

    fn get_context(&self) -> Vec<(String, String)>;

    fn get_camelcase_name(&self) -> String {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"[ -]").expect("Failed to parse regex");
        }
        let name = self.get_name();
        RE.split(name)
            .map(|part| part[0..1].to_uppercase() + &part[1..])
            .collect::<String>()
    }

    fn get_type_name(&self) -> &'static str;
}

pub struct TemplateContext;

impl TemplateContext {
    pub const NAME_CAMELCASE: &'static str = "name-camelcase";
    const OPENING_DELIM: &'static str = "{{";
    const CLOSING_DELIM: &'static str = "}}";
    pub const EXT_NAME: &'static str = "ext-name";
    pub const ELEMENT_NAME: &'static str = "element-name";

    pub fn format_key<S: Into<String> + Display>(key: &S) -> String {
        format!("{} {} {}", Self::OPENING_DELIM, key, Self::CLOSING_DELIM)
    }
}

pub trait External {
    fn get_filename(&self) -> String;
    fn get_extension(&self) -> String {
        "js".into()
    }

    fn add_to_externals(&self, config: &mut Config) {
        config
            .externals
            .insert(self.get_filename(), self.get_server_path(config));
    }

    fn get_server_path(&self, config: &Config) -> String {
        let filename = [self.get_filename(), self.get_extension()].join(".");
        let parts = ["/o", &config.project_name, &filename];

        parts.join("/")
    }
}
