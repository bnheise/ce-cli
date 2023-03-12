use super::{initialize_param, prepare_url};
use super::{prepare_data_path, ApiConfig, ObjectAdminConfig};
use crate::commands::object::get_object_client;
use crate::{
    cli::ExportObjectArgs,
    config_generators::{config::Config, ConfigFile},
    error::CliError,
};
use batch_api::models::import_task::ImportStrategy;
use batch_api::models::ImportTask;
use batch_api::reqwest::header::CONTENT_TYPE;
use batch_api::reqwest::Url;
use object_admin::apis::object_definition_api::post_object_definition_batch;
use object_admin::models::object_field::BusinessType;
use object_admin::models::{CreationStrategy, ObjectDefinition};
use regex::Regex;
use std::fs;
use std::path::Path;

pub fn handle_export(args: ExportObjectArgs) -> Result<(), CliError> {
    let ExportObjectArgs { all, .. } = &args;
    if *all {
        export_all(args)?;
    } else {
        todo!("Handle the not all case")
    }
    Ok(())
}

fn export_all(args: ExportObjectArgs) -> Result<(), CliError> {
    dotenv::dotenv().ok();
    let ExportObjectArgs {
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

    export_object_definitions(&base_data_dir, &username, &password, &url)?; // TODO: strip out relations, post batch, then post relations
    export_object_data(&url, &base_data_dir, &username, &password)?;

    Ok(())
}

fn export_object_definitions(
    base_data_dir: &str,
    username: &str,
    password: &str,
    url: &Url,
) -> Result<(), CliError> {
    println!("Loading object definitions...");

    let mut api_config = ObjectAdminConfig::new();
    api_config.basic_auth = Some((username.into(), Some(password.into())));
    api_config.update_base_path(url);

    let definitions_path = Path::new(base_data_dir).join("definitions");
    let mut object_definitions = Vec::new();
    let mut context_paths = Vec::new();

    for entry in fs::read_dir(definitions_path).map_err(|e| {
        CliError::ReadFile("Failed to open source directory for export data".into(), e)
    })? {
        let entry =
            entry.map_err(|e| CliError::ReadFile("Failed to read directory entry".into(), e))?;
        let path = entry.path();
        let file = fs::read_to_string(&path)
            .map_err(|e| CliError::ReadFile(format!("Failed to open file ${path:?}"), e))?;

        let mut object_def = serde_json::from_str::<ObjectDefinition>(&file)
            .map_err(|e| CliError::ParseJson(path.to_str().unwrap().to_string(), e))?;
        object_def.actions = None;
        object_def.id = None;
        object_def.active = None;

        if let (Some(context_path), Some(name)) = (&object_def.rest_context_path, &object_def.name)
        {
            context_paths.push((name.to_owned(), context_path.to_owned()))
        }

        if let Some(object_fields) = object_def.object_fields {
            object_def.object_fields = Some(
                object_fields
                    .into_iter()
                    .filter(|field| {
                        if let Some(business_type) = field.business_type {
                            business_type != BusinessType::Relationship
                        } else {
                            true
                        }
                    })
                    .collect::<Vec<_>>(),
            )
        }
        let erc = object_def
            .external_reference_code
            .clone()
            .unwrap_or_default();
        println!("Found definition: {erc}");
        object_definitions.push(object_def);
    }
    let body =
        serde_json::to_value(object_definitions).expect("Should be able to convert to json value");

    println!("Sending data to Liferay as batch request...");
    let response = post_object_definition_batch(
        &api_config,
        None,
        Some(&body),
        Some(CreationStrategy::Upsert),
        Some(batch_api::models::import_task::ImportStrategy::Continue),
    )
    .map_err(|e| match e {
        object_admin::apis::Error::Reqwest(e) => CliError::NetworkError("Batch request failed", e),
        object_admin::apis::Error::Serde(e) => {
            CliError::SerializeJson("Serialization error occurred during batch request", e)
        }
        object_admin::apis::Error::Io(_) => {
            todo!()
        }
        object_admin::apis::Error::ResponseError(e) => {
            println!("{e:?}");
            todo!()
        }
    })?;

    println!(
        "Post batch sent. Batch operation erc is {}",
        response.external_reference_code.unwrap_or_default()
    );
    Ok(())
}

fn export_object_data(
    url: &Url,
    base_data_dir: &str,
    username: &str,
    password: &str,
) -> Result<(), CliError> {
    println!("Loading object data...");
    let re = Regex::new(r"(/o/c/\w+)").expect("Invalid regex. Please fix.");

    let client = get_object_client()?;
    let data_path = Path::new(base_data_dir).join("data");

    for entry in fs::read_dir(data_path).map_err(|e| {
        CliError::ReadFile("Failed to open source directory for export data".into(), e)
    })? {
        let entry =
            entry.map_err(|e| CliError::ReadFile("Failed to read directory entry".into(), e))?;
        let path = entry.path();
        println!("{path:?}");
        let content = fs::read_to_string(&path)
            .map_err(|e| CliError::ReadFile(format!("Failed to open file ${path:?}"), e))?;
        let parsed = serde_json::from_str::<serde_json::Value>(&content)
            .map_err(|e| CliError::ParseJson(path.to_str().unwrap().to_owned(), e))?;

        let get_href = get_get_href(parsed);

        if let Some(get_href) = get_href {
            println!("href: {get_href}");
            if let Some(captures) = re.captures(&get_href) {
                println!("Found captures");
                let context_path = captures
                    .iter()
                    .next()
                    .expect("Should have found a capture")
                    .unwrap()
                    .as_str();

                let batch_path = format!("{context_path}/batch");

                let resp = client
                    .post(url.join(&batch_path).unwrap())
                    .header(CONTENT_TYPE, "application/json")
                    .query(&[
                        ("importStrategy", ImportStrategy::Continue.to_string()),
                        ("createStrategy", CreationStrategy::Upsert.to_string()),
                    ])
                    .body(content)
                    .basic_auth(username, Some(password))
                    .send()
                    .map_err(|e| {
                        CliError::NetworkError("Failed to retrieve object data from Liferay", e)
                    })?;
                println!("{resp:?}");
                let resp = resp
                    .json::<ImportTask>()
                    .expect("Failed to deserialize repsonse from Liferay");

                println!(
                    "Sent batch request to Liferay for {}. Erc for task is {}",
                    path.to_str().unwrap(),
                    resp.external_reference_code.unwrap_or_default()
                );
            }
        }
    }

    // for (name, path) in context_paths {
    //     let filename = format!("{name}.json");
    //     let path = format!("{path}/batch");

    //     if let Ok(content) = fs::read_to_string(definitions_path.join(&filename)) {
    //         let url = url
    //             .join(&path)
    //             .map_err(|_| CliError::InvalidInput(path.to_owned()))?;

    //         let resp = client
    //             .post(url)
    //             .query(&[
    //                 ("importStrategy", ImportStrategy::Continue.to_string()),
    //                 ("createStrategy", CreationStrategy::Upsert.to_string()),
    //             ])
    //             .body(content)
    //             .basic_auth(username, Some(password))
    //             .send()
    //             .map_err(|e| {
    //                 CliError::NetworkError("Failed to retrieve object data from Liferay", e)
    //             })?;

    //         let resp = resp
    //             .json::<ImportTask>()
    //             .expect("Failed to deserialize repsonse from Liferay");

    //         println!(
    //             "Sent batch request to Liferay for {name}. Erc for task is {}",
    //             resp.external_reference_code.unwrap_or_default()
    //         );
    //     } else {
    //         println!("Failed to read object data for ${name}");
    //     }
    // }
    Ok(())
}

fn get_get_href(json_val: serde_json::Value) -> Option<String> {
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
