use headless_common::reqwest;
use std::io;
use thiserror::Error;
use zip::result::ZipError;

#[derive(Error, Debug)]
pub enum CliError {
    #[error("Network request failed: {0}")]
    NetworkError(String), // CurrentDirectory(Option<std::io::Error>),
    #[error("Failed to read or write to file or directory: {0}")]
    FileSystemError(String), // Write(String, std::io::Error),
    #[error("Failed to serialize/deserialize yaml: {0}")]
    YamlError(String),
    #[error("Failed to serialize/deserialize json: {0}")]
    JsonError(String),
    #[error("The extension name you entered is invalid. The name must start with an alphabet character and may not contain special symbols other than -")]
    InvalidExtensionName,
    #[error("The extension you are trying to create already exists!")]
    ExtensionExists,
    #[error("Failed to parse npm package name: {0}")]
    ParsePackageName(String),
    #[error("Required parameter not provided: {0}")]
    MissingParameter(String),
    #[error("Invalid input provided: {0}")]
    InvalidInput(String),
}

impl From<reqwest::Error> for CliError {
    fn from(value: reqwest::Error) -> Self {
        Self::NetworkError(value.to_string())
    }
}

impl From<io::Error> for CliError {
    fn from(value: io::Error) -> Self {
        Self::FileSystemError(value.to_string())
    }
}

impl From<ZipError> for CliError {
    fn from(value: ZipError) -> Self {
        Self::FileSystemError(value.to_string())
    }
}

impl From<serde_yaml::Error> for CliError {
    fn from(value: serde_yaml::Error) -> Self {
        Self::YamlError(value.to_string())
    }
}

impl From<serde_json::Error> for CliError {
    fn from(value: serde_json::Error) -> Self {
        Self::JsonError(value.to_string())
    }
}
