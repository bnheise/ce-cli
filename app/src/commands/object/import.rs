use crate::{
    cli::ImportObjectArgs,
    config_generators::{config::Config, ConfigFile},
    error::CliError,
};
use headless_admin_list_type::apis::{
    configuration::Configuration as ListTypeConfig,
    list_type_definition_api::get_list_type_definitions_page,
};
use reqwest::Url;
use std::{
    fs::{self, File},
    io::Write,
    path::Path,
    str::FromStr,
};
use ObjectAdmin::apis::{
    configuration::Configuration as ObjectAdminConfig,
    object_definition_api::get_object_definitions_page,
};

pub fn handle_import(args: ImportObjectArgs) -> Result<(), CliError> {
    let ImportObjectArgs {
        all, erc, source, ..
    } = args.clone();

    if all {
        handle_import_all(args)?;
    } else {
        let erc = &erc.expect("Erc should have been provided.");
        let source = &source.expect("Source should have been provided");
    }

    Ok(())
}

fn handle_import_all(args: ImportObjectArgs) -> Result<(), CliError> {
    dotenv::dotenv().ok();
    let ImportObjectArgs {
        username,
        password,
        output,
        url,
        port,
        ..
    } = args;

    let username = if let Some(username) = username {
        username
    } else if let Ok(username) = dotenv::var("LIFERAY_USERNAME") {
        username
    } else {
        return Err(CliError::MissingParameter("username must be provided as an argument or via the environment variable LIFERAY_USERNAME"));
    };

    let password = if let Some(password) = password {
        password
    } else if let Ok(password) = dotenv::var("LIFERAY_PASSWORD") {
        password
    } else {
        return Err(CliError::MissingParameter("password must be provided as an argument or via the environment variable LIFERAY_PASSWORD"));
    };

    let config = Config::try_open();
    let url = prepare_url(url, port, config.is_ok());

    let output_base = if let Some(output) = output {
        output
    } else if config.is_ok() {
        "./objects".into()
    } else {
        todo!("MAKE AN ERROR")
    };

    let mut api_config = ObjectAdminConfig::new();
    api_config.basic_auth = Some((username.clone(), Some(password.clone())));

    if let Some(url) = &url {
        api_config.update_base_path(url);
    }

    let result =
        get_object_definitions_page(&api_config, None, None, Some("1"), Some("200"), None, None)
            .unwrap();

    if let Some(items) = result.items {
        for object_def in items.iter() {
            if let Some(system) = object_def.system {
                if !system {
                    let path = Path::new(&output_base).join("definitions");
                    let name = object_def.name.as_ref().unwrap();
                    fs::create_dir_all(&path).unwrap();

                    let filepath = path.join(format!("{name}.json"));
                    let mut file = File::create(filepath).unwrap();
                    file.write_all(serde_json::to_string_pretty(object_def).unwrap().as_bytes())
                        .unwrap();
                }
            }
        }
    }

    let mut api_config = ListTypeConfig::new();
    api_config.basic_auth = Some((username, Some(password)));

    if let Some(url) = &url {
        api_config.update_base_path(url);
    }

    let result =
        get_list_type_definitions_page(&api_config, None, None, Some("1"), Some("200"), None, None)
            .unwrap();

    if let Some(items) = result.items {
        for object_def in items.iter() {
            let path = Path::new(&output_base).join("picklists");
            let name = object_def.name.as_ref().unwrap();
            fs::create_dir_all(&path).unwrap();

            let filepath = path.join(format!("{name}.json"));
            let mut file = File::create(filepath).unwrap();
            file.write_all(serde_json::to_string_pretty(object_def).unwrap().as_bytes())
                .unwrap();
        }
    }

    Ok(())
}

fn handle_import_item() -> Result<(), CliError> {
    Ok(())
}

fn prepare_url(url: Option<Url>, port: Option<u16>, is_workspace: bool) -> Option<Url> {
    if let Some(mut url) = url {
        let port = port.unwrap_or(8080);
        url.set_port(Some(port)).unwrap();
        Some(url)
    } else if is_workspace {
        let host = dotenv::var("LIFERAY_HOST").unwrap_or("http://localhost".to_owned());
        let port = dotenv::var("LIFERAY_PORT").unwrap_or("8080".to_owned());
        let mut url = Url::from_str(&host).unwrap();
        url.set_port(Some(u16::from_str(&port).unwrap())).unwrap();
        Some(url)
    } else {
        None
    }
}

trait ApiConfig {
    fn update_base_path(&mut self, replacement: &Url);
}

impl ApiConfig for ObjectAdminConfig {
    fn update_base_path(&mut self, replacement: &Url) {
        self.base_path = self
            .base_path
            .replace("http://localhost:8080", replacement.as_str());
    }
}

impl ApiConfig for ListTypeConfig {
    fn update_base_path(&mut self, replacement: &Url) {
        self.base_path = self
            .base_path
            .replace("http://localhost:8080", replacement.as_str());
    }
}
