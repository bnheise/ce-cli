use self::{export::handle_export, import::handle_import};
use crate::{cli::ObjectOption, error::CliError};
use batch_api::reqwest;
use batch_api::reqwest::Url;
pub use headless_admin_list_type::apis::{
    configuration::Configuration as ListTypeConfig,
    list_type_definition_api::get_list_type_definitions_page,
};
pub use object_admin::apis::{
    configuration::Configuration as ObjectAdminConfig,
    object_definition_api::get_object_definitions_page,
};
use std::str::FromStr;

pub mod export;
pub mod import;

pub fn handle_object(options: ObjectOption) -> Result<(), CliError> {
    match options {
        ObjectOption::Import(args) => handle_import(args)?,
        ObjectOption::Export(args) => handle_export(args)?,
    }
    Ok(())
}

fn initialize_param<T: From<String>>(
    param: Option<T>,
    env_key: &str,
    param_name: &'static str,
) -> Result<T, CliError> {
    if let Some(param) = param {
        Ok(param)
    } else if let Ok(username) = dotenv::var(env_key) {
        Ok(username.into())
    } else {
        Err(CliError::MissingParameter(param_name))
    }
}

fn prepare_url(url: Option<Url>, port: Option<u16>, is_workspace: bool) -> Result<Url, CliError> {
    if let Some(mut url) = url {
        let port = port.unwrap_or(8080);
        url.set_port(Some(port)).unwrap();
        Ok(url)
    } else if is_workspace {
        let host = dotenv::var("LIFERAY_HOST").unwrap_or("http://localhost".to_owned());
        let port = dotenv::var("LIFERAY_PORT").unwrap_or("8080".to_owned());
        let mut url = Url::from_str(&host)
            .map_err(|_| CliError::InvalidInput("Received invalid Liferay host".into()))?;
        let parsed_port = u16::from_str(&port)
            .map_err(|_| CliError::InvalidInput("Could not parse provided port to u16".into()))?;

        url.set_port(Some(parsed_port))
            .map_err(|_| CliError::InvalidInput("Failed to add port to Liferay host".into()))?;

        url.set_path("");

        Ok(url)
    } else {
        Err(CliError::MissingParameter("Url parameter is required when executing this command from outside a ce-cli generated workspace"))
    }
}

fn prepare_data_path(output: Option<String>, is_workspace: bool) -> Result<String, CliError> {
    if let Some(output) = output {
        Ok(output)
    } else if is_workspace {
        Ok("./objects".into())
    } else {
        Err(CliError::MissingParameter("output"))
    }
}

pub trait ApiConfig {
    fn update_base_path(&mut self, replacement: &Url);
}

impl ApiConfig for ObjectAdminConfig {
    fn update_base_path(&mut self, replacement: &Url) {
        let as_str = replacement.as_str();
        self.base_path = self
            .base_path
            .replace("http://localhost:8080", &as_str[0..(as_str.len() - 1)]);
    }
}

impl ApiConfig for ListTypeConfig {
    fn update_base_path(&mut self, replacement: &Url) {
        self.base_path = self
            .base_path
            .replace("http://localhost:8080", replacement.as_str());
    }
}

fn get_object_client() -> Result<reqwest::blocking::Client, CliError> {
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        reqwest::header::ACCEPT,
        reqwest::header::HeaderValue::from_str("application/json").unwrap(),
    );

    reqwest::blocking::ClientBuilder::new()
        .default_headers(headers)
        .build()
        .map_err(|e| CliError::NetworkError("Failed to build network client", e))
}

// fn prepare_base_url(base_path: &str, url: &Option<Url>) -> String {
//     let url = if let Some(url) = url.clone() {
//         url
//     } else {
//         Url::parse(base_path).unwrap()
//     };

//     match url.origin() {
//         url::Origin::Opaque(_) => unreachable!(),
//         url::Origin::Tuple(scheme, host, port) => format!("{scheme}://{host}:{port}"),
//     }
// }
