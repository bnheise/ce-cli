use super::prepare_data_path;
use super::{initialize_param, prepare_url};
use crate::data_dir::DataDir;
use crate::liferay_client::client::LiferayClient;
use crate::liferay_client::client_options::LiferayClientOptions;
use crate::{
    cli::ExportObjectArgs,
    config_generators::{config::Config, ConfigFile},
    error::CliError,
};
use regex::Regex;
use std::fs;
use std::path::Path;

pub fn handle_export(args: ExportObjectArgs) -> Result<(), CliError> {
    let ExportObjectArgs {
        source,
        username,
        password,
        url,
        port,
        directory,
        ..
    } = args;
    let username = initialize_param(username, "LIFERAY_USERNAME", "username")?;
    let password = initialize_param(password, "LIFERAY_PASSWORD", "password")?;

    let config = Config::try_open();
    let url = prepare_url(url, port, config.is_ok())?;

    let base_data_dir = prepare_data_path(directory, config.is_ok())?;

    let options = LiferayClientOptions {
        base_url: url,
        username,
        password,
    };

    let client = LiferayClient::new(options);
    match source {
        crate::cli::ImportExportSource::Picklist => todo!(),
        crate::cli::ImportExportSource::ObjectDefinition => {
            export_object_definitions(&client, &base_data_dir)?;
        }
        crate::cli::ImportExportSource::ObjectData => {
            export_object_data(&client, &base_data_dir)?;
        }
        crate::cli::ImportExportSource::DefinitionsAndPicklists => {
            export_object_definitions(&client, &base_data_dir)?;
            todo!("Add picklists")
        }
    }
    Ok(())
}

fn export_object_definitions(client: &LiferayClient, base_data_dir: &str) -> Result<(), CliError> {
    println!("Loading object definitions...");
    let data_dir = DataDir::init(base_data_dir);

    let object_definitions = data_dir.load_object_definitions(true)?;

    println!("Sending data to Liferay as batch request...");
    let response = client
        .get_object_admin_api()
        .get_object_admin_endpoints()
        .post_object_definition_batch(object_definitions, None, None)
        .map_err(|e| {
            CliError::NetworkError(format!("Failed post object definitions batch: {e}",))
        })?;

    println!(
        "Post batch sent. Batch operation erc is {}",
        response.external_reference_code.unwrap_or_default()
    );
    Ok(())
}

fn export_object_data(client: &LiferayClient, base_data_dir: &str) -> Result<(), CliError> {
    println!("Loading object data...");
    let re = Regex::new(r"(/o/c/\w+)").expect("Invalid regex. Please fix.");

    let data_path = Path::new(base_data_dir).join("data");

    for entry in fs::read_dir(data_path)? {
        let entry = entry?;
        let path = entry.path();

        let content = fs::read_to_string(&path)?;
        let parsed = serde_json::from_str::<serde_json::Value>(&content)?;

        let get_href = get_get_href(&parsed);

        if let Some(get_href) = get_href {
            if let Some(captures) = re.captures(&get_href) {
                let context_path = captures
                    .iter()
                    .next()
                    .expect("Should have found a capture")
                    .unwrap()
                    .as_str();

                let batch_path = format!("{context_path}/batch");

                let resp = client
                    .get_custom_object_api()
                    .post_object_data_batch(&parsed, &batch_path, None, None)
                    .map_err(|e| {
                        CliError::NetworkError(format!(
                            "Custom objects data batch request post failed: {e}"
                        ))
                    })?;

                println!(
                    "Successfully sent batch request to Liferay. Erc is {}",
                    resp.external_reference_code.unwrap_or_default()
                )
            } else {
                println!("Invalid href found in post data. Skipping...")
            }
        } else {
            println!("Could not find href to build deploy path from. Skipping...")
        }
    }

    Ok(())
}

fn get_get_href(json_val: &serde_json::Value) -> Option<String> {
    match json_val {
        serde_json::Value::Array(arr) => {
            if let Some(obj) = arr.get(0) {
                match obj {
                    serde_json::Value::Object(obj) => {
                        if let Some(actions) = obj.get("actions") {
                            match actions {
                                serde_json::Value::Object(actions) => {
                                    if let Some(get) = actions.get("get") {
                                        match get {
                                            serde_json::Value::Object(get) => {
                                                if let Some(href) = get.get("href") {
                                                    match href {
                                                        serde_json::Value::String(href) => {
                                                            Some(href.to_owned())
                                                        }
                                                        _ => {
                                                            println!("Expected 'href'to be a string, but was not. Cannot determine REST context. Skipping file.");
                                                            None
                                                        }
                                                    }
                                                } else {
                                                    println!("Expected 'get' to have key 'href' but did not. Cannot determine REST context. Skipping file.");
                                                    None
                                                }
                                            }
                                            _ => {
                                                println!("Expected 'get' to exist but didn not. Cannot determine REST context. Skipping file.");
                                                None
                                            }
                                        }
                                    } else {
                                        println!("Expected key 'get' exist on 'actions' but didn't find it. Cannot determine REST context. Skipping file.");
                                        None
                                    }
                                }
                                _ => {
                                    println!("Expected 'actions' to be an object, but wasn't. Skipping file.");
                                    None
                                }
                            }
                        } else {
                            println!("Failed to find 'actions' key. Cannot determine REST context. Skipping file.");
                            None
                        }
                    }
                    _ => {
                        println!("Expected an object, but didn't find one. Skipping file");
                        None
                    }
                }
            } else {
                println!("Empty array found. Skipping file.");
                None
            }
        }
        _ => {
            println!("Expected file to contain a json array, but it wasn't. Skipping.");
            None
        }
    }
}
