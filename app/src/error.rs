use std::error::{self, Error};
use std::fmt::Display;
use zip::result::ZipError;

#[derive(Debug)]
pub enum CliError {
    CurrentDirectory(Option<std::io::Error>),
    Write(String, std::io::Error),
    ParseJson(&'static str, serde_json::Error),
    SerializeJson(&'static str, serde_json::Error),
    NotADirectory(String),
    Zip(ZipError),
    OpenFile(String, std::io::Error),
    ReadFile(String, std::io::Error),
    Input(std::io::Error),
    SerializeYaml(String, serde_yaml::Error),
    ParseYaml(&'static str, serde_yaml::Error),
    Http(String, reqwest::Error),
    Init(String),
    InvalidDirectory(String),
    InvalidExtensionName,
    ExtensionExists
}

impl Display for CliError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CliError::CurrentDirectory(..) => {
                write!(
                    f,
                    "An error occurred when attempting to read the current directory"
                )
            }
            CliError::Write(name, ..) => {
                write!(f, "Failed to write a file or directory: {name}")
            }
            CliError::ParseJson(filename, ..) => write!(f, "Failed to parse json: {filename}"),
            CliError::SerializeJson(filename, ..) => {
                write!(f, "Failed to serialize json: {filename}")
            }
            CliError::NotADirectory(path) => {
                write!(f, "Expected at path {path} but found a file instead")
            }
            CliError::Zip(..) => write!(f, "Failed to write zip file"),
            CliError::OpenFile(filename, ..) => write!(f, "Failed to open file {filename}"),
            CliError::ReadFile(filename, ..) => {
                write!(f, "Failed to read content of file {filename}")
            }
            CliError::Input(..) => write!(f, "To retrieve input"),
            CliError::SerializeYaml(filename, ..) => {
                write!(f, "Failed to serialize yaml: {filename}")
            }
            CliError::ParseYaml(filename, ..) => write!(f, "Failed to parse yaml: {filename}"),
            CliError::Http(message, _) => {
                write!(f, "{message}")
            }
            CliError::Init(message) => write!(f, "Error initializing project: {message}"),
            
            CliError::InvalidDirectory(message) => {
                write!(f, "Action performed in invalid directory: {message}")
            }
            CliError::InvalidExtensionName =>  write!(f, "The extension name you entered is invalid. The name must start with an alphabet character and may not contain special symbols other than -"),
            CliError::ExtensionExists => write!(f, "The extension you are trying to create already exists!"),
        }
    }
}

impl Error for CliError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            CliError::CurrentDirectory(ref e) => match e {
                Some(e) => Some(e),
                None => None,
            },
            CliError::Write(.., e) => Some(e),
            CliError::ParseJson(.., e) => Some(e),
            CliError::SerializeJson(.., e) => Some(e),
            CliError::NotADirectory(..) => None,
            CliError::Zip(e) => Some(e),
            CliError::OpenFile(.., e) => Some(e),
            CliError::ReadFile(.., e) => Some(e),
            CliError::Input(e) => Some(e),
            CliError::SerializeYaml(_, e) => Some(e),
            CliError::ParseYaml(.., e) => Some(e),
            CliError::Http(.., e) => Some(e),
            CliError::Init(_) => None,
            CliError::InvalidDirectory(_) => None,
            CliError::InvalidExtensionName => None,
            CliError::ExtensionExists => None,
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
