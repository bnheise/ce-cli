use crate::error::CliError;
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};
use yaml_rust::{YamlEmitter, YamlLoader};

pub mod cet_configuration;
pub mod client_extension_yaml;
pub mod config;
pub mod eslintrc;
pub mod package_json;

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
        let raw = fs::read_to_string(path)
            .map_err(|e| CliError::ReadFileError(Self::FILENAME.into(), e))?;

        Ok(raw)
    }

    fn try_parse(raw: &'a str) -> Result<Self, CliError>
    where
        Self: Sized,
    {
        let parsed = match Self::FORMAT {
            ConfigFormat::Yaml => serde_yaml::from_str::<Self>(raw)
                .map_err(|e| CliError::ParseYamlError(Self::FILENAME, e))?,
            ConfigFormat::Json => serde_json::from_str::<Self>(raw)
                .map_err(|e| CliError::ParseJsonError(Self::FILENAME, e))?,
        };

        Ok(parsed)
    }

    fn try_serialize(config: Self) -> Result<String, CliError> {
        let raw = match Self::FORMAT {
            ConfigFormat::Yaml => {
                let raw = serde_yaml::to_string(&config)
                    .map_err(|e| CliError::SerializeYamlError(Self::FILENAME.to_owned(), e))?;
                format_yaml(&raw)?
            }
            ConfigFormat::Json => serde_json::to_string_pretty(&config)
                .map_err(|e| CliError::SerializeJsonError(Self::FILENAME, e))?,
        };

        Ok(raw)
    }

    fn write(raw: String) -> Result<(), CliError> {
        let path = Self::get_filepath();

        fs::write(path, raw).map_err(|e| CliError::WriteError(Self::FILENAME.to_owned(), e))?;
        Ok(())
    }
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

        fs::create_dir_all(&path)
            .map_err(|e| CliError::WriteError(path.to_string_lossy().to_string(), e))?;
        fs::write(path.join(filename), content)
            .map_err(|e| CliError::WriteError(filename.to_owned(), e))?;

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
