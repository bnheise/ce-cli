use std::borrow::BorrowMut;
use std::collections::HashMap;

use super::prepare_data_path;
use super::{initialize_param, prepare_url};
use crate::data_dir::DataDir;
use crate::liferay_client::client::LiferayClient;
use crate::liferay_client::client_options::LiferayClientOptions;
use crate::{
    cli::ExportArgs,
    config_generators::{config::Config, ConfigFile},
    error::CliError,
};
use headless_admin_list_type::models::list_type_entry::ListTypeEntryField;
use headless_batch_engine::models::import_task::ImportStrategy;
use headless_common::api::field_collection::FieldCollection;
use headless_common::api::page_params::PageParams;
use regex::Regex;

pub fn handle_export(args: ExportArgs) -> Result<(), CliError> {
    let ExportArgs {
        username,
        password,
        url,
        port,
        directory,
        objects,
        picklists,
        data,
        ..
    } = args;
    let username = initialize_param(username, "LIFERAY_USERNAME", "username")?;
    let password = initialize_param(password, "LIFERAY_PASSWORD", "password")?;

    let config = Config::try_open();
    let url = prepare_url(url, port, config.is_ok())?;

    let base_data_dir = prepare_data_path(directory, config.is_ok())?;
    let data_dir = DataDir::init(&base_data_dir);

    let options = LiferayClientOptions {
        base_url: url,
        username,
        password,
    };

    let client = LiferayClient::new(options);
    match (objects, picklists, data) {
        (false, true, false) => {
            export_picklist_definitions(&client, &data_dir)?;
        }
        (true, false, false) => {
            export_object_definitions(&client, &data_dir)?;
        }
        (false, false, true) => {
            export_object_data(&client, &data_dir)?;
        }
        (true, true, false) => {
            export_object_definitions(&client, &data_dir)?;
            export_picklist_definitions(&client, &data_dir)?;
        }
        _ => unreachable!(),
    }
    Ok(())
}

fn export_picklist_definitions(client: &LiferayClient, data_dir: &DataDir) -> Result<(), CliError> {
    println!("Loading picklist definitions...");
    let picklists = data_dir.load_picklist_data()?;

    let mut entries_to_process = Vec::new();
    let mut existing_entry_ids = HashMap::new();

    for mut list_type_definition in picklists {
        let entries = list_type_definition.list_type_entries.take();
        let external_reference_code = list_type_definition.external_reference_code.take();

        if let Some(ref external_reference_code) = external_reference_code {
            println!("Sending put request for picklist ${external_reference_code}");
            let res = client
                .get_list_type_api()
                .get_list_type_api_endpoints()
                .put_list_type_definition(external_reference_code, Some(list_type_definition))
                .map_err(|e| {
                    CliError::NetworkError(format!(
                        "Failed to put picklist {}: {e}",
                        external_reference_code
                    ))
                })?;

            if let Some(definition_id) = res.id {
                println!("Put successful. Picklist id is ${definition_id}");

                println!("Getting existing entries for ${definition_id}");
                let mut options = PageParams::new();
                options.page = Some(1);
                options.page_size = Some(200);
                options.fields = Some(FieldCollection::from(vec![
                    ListTypeEntryField::Id,
                    ListTypeEntryField::Key,
                ]));
                let entries_res = client
                    .get_list_type_api()
                    .get_list_type_api_endpoints()
                    .get_list_type_definition_list_type_entries_page(
                        &definition_id.to_string(),
                        Some(options),
                    )
                    .map_err(|e| {
                        CliError::NetworkError(format!(
                            "Failed to get existing picklist entries for {}: {e}",
                            external_reference_code
                        ))
                    })?;

                if let Some(items) = entries_res.items {
                    for item in items {
                        if let (Some(entry_key), Some(entry_id)) = (item.key, item.id) {
                            existing_entry_ids
                                .insert(format!("{definition_id}-{entry_key}"), entry_id);
                        }
                    }
                }

                entries_to_process.push((definition_id, entries));
            } else {
                println!("Failed to retrieve picklist id. Entries will not be added")
            }
        } else {
            println!("Provided picklist has no external reference code. Skipping");
        }
    }

    // TODO: iterate over list type definitions to get list of entries
    // Partition entries to put into existing entries to update and entries to post
    // one by one send the requests

    let mut entries_to_put_batch = Vec::new();

    for (definition_id, entries) in entries_to_process {
        if let Some(entries) = entries {
            let (entries_to_post, mut entries_to_put): (Vec<_>, Vec<_>) = entries
                .into_iter()
                .map(|mut entry| {
                    let entry_key = entry.key.clone().unwrap_or_default();
                    let hash_key = format!("{definition_id}-{entry_key}");
                    entry.id = entry
                        .borrow_mut()
                        .id
                        .and_then(|_| existing_entry_ids.get(&hash_key).copied());
                    entry
                })
                .partition(|entry| entry.id.is_none());

            entries_to_put_batch.append(&mut entries_to_put);

            let res = client
                .get_list_type_api()
                .get_list_type_api_endpoints()
                .post_list_type_definition_list_type_entry_batch(
                    definition_id,
                    entries_to_post,
                    None,
                    Some(ImportStrategy::Continue),
                    None,
                )
                .map_err(|e| {
                    CliError::NetworkError(format!(
                        "Failed to send picklist entries batch request for picklist with id {definition_id}: {e}",
                    ))
                })?;

            println!(
                "Post batch for picklist {definition_id} sent. Batch operation erc is {}",
                res.external_reference_code
                    .unwrap_or("(erc not found)".into())
            );
        }
    }

    println!("Sending put batch request for existing picklists...");

    let res = client
        .get_list_type_api()
        .get_list_type_api_endpoints()
        .put_list_type_entry_batch(
            None,
            Some(entries_to_put_batch),
            None,
            Some(ImportStrategy::Continue),
        )
        .map_err(|e| {
            CliError::NetworkError(format!(
                "Failed to send picklist entries batch request for picklist existing entries: {e}",
            ))
        })?;

    println!(
        "Existing picklist batch sent. Batch operation erc is {}",
        res.external_reference_code.unwrap_or_default()
    );
    Ok(())
}

fn export_object_definitions(client: &LiferayClient, data_dir: &DataDir) -> Result<(), CliError> {
    println!("Loading object definitions...");

    let object_definitions = data_dir.load_object_definitions(true)?;

    println!("Sending data to Liferay as batch request...");
    let response = client
        .get_object_admin_api()
        .get_object_admin_endpoints()
        .post_object_definition_batch(object_definitions, None)
        .map_err(|e| {
            CliError::NetworkError(format!("Failed post object definitions batch: {e}",))
        })?;

    println!(
        "Post batch sent. Batch operation erc is {}",
        response.external_reference_code.unwrap_or_default()
    );
    Ok(())
}

fn export_object_data(client: &LiferayClient, data_dir: &DataDir) -> Result<(), CliError> {
    println!("Loading object data...");
    let re = Regex::new(r"(/o/c/\w+)").expect("Invalid regex. Please fix.");

    let object_data = data_dir.load_object_data()?;

    for entry in object_data {
        let get_href = get_get_href(&entry);

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
                    .post_object_data_batch(&entry, &batch_path, None, None)
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
