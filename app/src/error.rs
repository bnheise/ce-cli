use std::error::{self, Error};
use std::fmt::Display;
use zip::result::ZipError;

#[derive(Debug)]
pub enum CliError {
    NotImplemented(UpcomingFeature),
    CurrentDirectoryError(Option<std::io::Error>),
    WriteError(String, std::io::Error),
    ParseJsonError(&'static str, serde_json::Error),
    SerializeJsonError(&'static str, serde_json::Error),
    NotADirectoryError(String),
    ZipError(ZipError),
    OpenFileError(String, std::io::Error),
    ReadFileError(String, std::io::Error),
    InputError(std::io::Error),
    SerializeYamlError(String, serde_yaml::Error),
    ParseYamlError(&'static str, serde_yaml::Error),
    HttpError(String, reqwest::Error),
}

impl Display for CliError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CliError::NotImplemented(feature_name) => {
                write!(
                    f,
                    "The feature you attemped to use ({feature_name}) is still under development."
                )
            }
            CliError::CurrentDirectoryError(..) => {
                write!(
                    f,
                    "An error occurred when attempting to read the current directory"
                )
            }
            CliError::WriteError(name, ..) => {
                write!(f, "Failed to write a file or directory: {name}")
            }
            CliError::ParseJsonError(filename, ..) => write!(f, "Failed to parse json: {filename}"),
            CliError::SerializeJsonError(filename, ..) => {
                write!(f, "Failed to serialize json: {filename}")
            }
            CliError::NotADirectoryError(path) => {
                write!(f, "Expected at path {path} but found a file instead")
            }
            CliError::ZipError(..) => write!(f, "Failed to write zip file"),
            CliError::OpenFileError(filename, ..) => write!(f, "Failed to open file {filename}"),
            CliError::ReadFileError(filename, ..) => {
                write!(f, "Failed to read content of file {filename}")
            }
            CliError::InputError(..) => write!(f, "To retrieve input"),
            CliError::SerializeYamlError(filename, ..) => {
                write!(f, "Failed to serialize yaml: {filename}")
            }
            CliError::ParseYamlError(filename, ..) => write!(f, "Failed to parse yaml: {filename}"),
            CliError::HttpError(message, _) => write!(f, "{message}"),
        }
    }
}

impl Error for CliError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            CliError::NotImplemented(_) => None,
            CliError::CurrentDirectoryError(ref e) => match e {
                Some(e) => Some(e),
                None => None,
            },
            CliError::WriteError(.., e) => Some(e),
            CliError::ParseJsonError(.., e) => Some(e),
            CliError::SerializeJsonError(.., e) => Some(e),
            CliError::NotADirectoryError(..) => None,
            CliError::ZipError(e) => Some(e),
            CliError::OpenFileError(.., e) => Some(e),
            CliError::ReadFileError(.., e) => Some(e),
            CliError::InputError(e) => Some(e),
            CliError::SerializeYamlError(_, e) => Some(e),
            CliError::ParseYamlError(.., e) => Some(e),
            CliError::HttpError(.., e) => Some(e),
        }
    }
}

#[derive(Debug)]
pub enum UpcomingFeature {
    SetConfigPathOnInit,
}

impl Display for UpcomingFeature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UpcomingFeature::SetConfigPathOnInit => write!(f, "SetConfigPathOnInit"),
        }
    }
}
