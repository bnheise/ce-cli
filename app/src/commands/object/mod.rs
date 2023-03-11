use self::{export::handle_export, import::handle_import};
use crate::{cli::ObjectOption, error::CliError};
pub use headless_admin_list_type::apis::{
    configuration::Configuration as ListTypeConfig,
    list_type_definition_api::get_list_type_definitions_page,
};
pub use object_admin::apis::{
    configuration::Configuration as ObjectAdminConfig,
    object_definition_api::get_object_definitions_page,
};
use reqwest::Url;
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

fn prepare_url(
    url: Option<Url>,
    port: Option<u16>,
    is_workspace: bool,
) -> Result<Option<Url>, CliError> {
    if let Some(mut url) = url {
        let port = port.unwrap_or(8080);
        url.set_port(Some(port)).unwrap();
        Ok(Some(url))
    } else if is_workspace {
        let host = dotenv::var("LIFERAY_HOST").unwrap_or("http://localhost".to_owned());
        let port = dotenv::var("LIFERAY_PORT").unwrap_or("8080".to_owned());
        let mut url = Url::from_str(&host)
            .map_err(|_| CliError::InvalidInput("Received invalid Liferay host"))?;
        let parsed_port = u16::from_str(&port)
            .map_err(|_| CliError::InvalidInput("Could not parse provided port to u16"))?;
        println!("{url:?}");
        url.set_port(Some(parsed_port))
            .map_err(|_| CliError::InvalidInput("Failed to add port to Liferay host"))?;
        println!("{url:?}");
        url.set_path("");
        println!("{url:?}");
        Ok(Some(url))
    } else {
        Ok(None)
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
