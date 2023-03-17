use headless_common::{api::page_params::PageParams, url::Url};

use super::{
    get_list_type_definitions_page, get_object_definitions_page, prepare_data_path, ApiConfig,
    ListTypeConfig, ObjectAdminConfig,
};
use crate::{
    cli::ImportArgs,
    commands::object::get_object_client,
    config_generators::{config::Config, ConfigFile},
    error::CliError,
};
use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};

use super::{initialize_param, prepare_url};

pub fn handle_import(args: ImportArgs) -> Result<(), CliError> {
    dotenv::dotenv().ok();
    let ImportArgs {
        all,
        username,
        password,
        output,
        url,
        port,
        objects,
        picklists,
        data,
        ..
    } = args;
    // TODO: refactor this part to match new pattern
    let username = initialize_param(username, "LIFERAY_USERNAME", "username")?;
    let password = initialize_param(password, "LIFERAY_PASSWORD", "password")?;

    let config = Config::try_open();
    let url = prepare_url(url, port, config.is_ok())?;
    let output_base = prepare_data_path(output, config.is_ok())?;

    if all {
        handle_import_all(&username, &password, &url, &output_base)?;
    } else {
        match (objects, picklists, data) {
            (false, true, false) => import_picklists(&username, &password, &url, &output_base)?,
            (true, false, false) => {
                import_object_definitions(&output_base, &username, &password, &url, true)?;
            }
            (false, false, true) => {
                let context_paths =
                    import_object_definitions(&output_base, &username, &password, &url, false)?;
                import_object_data(context_paths, &url, &output_base, &username, &password)?;
            }
            (true, true, false) => {
                import_object_definitions(&output_base, &username, &password, &url, true)?;
                import_picklists(&username, &password, &url, &output_base)?;
            }
            (true, false, true) => {
                let context_paths =
                    import_object_definitions(&output_base, &username, &password, &url, true)?;
                import_object_data(context_paths, &url, &output_base, &username, &password)?;
            }
            (false, true, true) => {
                let context_paths =
                    import_object_definitions(&output_base, &username, &password, &url, false)?;
                import_picklists(&username, &password, &url, &output_base)?;
                import_object_data(context_paths, &url, &output_base, &username, &password)?;
            }
            _ => unreachable!(),
        };
    };

    Ok(())
}

fn handle_import_all(
    username: &str,
    password: &str,
    url: &Url,
    output_base: &str,
) -> Result<(), CliError> {
    let context_paths = import_object_definitions(output_base, username, password, url, true)?;

    import_object_data(context_paths, url, output_base, username, password)?;

    import_picklists(username, password, url, output_base)?;

    Ok(())
}

fn import_picklists(
    username: &str,
    password: &str,
    url: &Url,
    output_base: &str,
) -> Result<(), CliError> {
    println!("Importing picklist data...");
    let mut api_config = ListTypeConfig::new();
    api_config.basic_auth = Some((username.to_owned(), Some(password.to_owned())));
    api_config.update_base_path(url);

    let mut options = PageParams::new();
    options.page = Some(1);
    options.page_size = Some(200);

    let result = get_list_type_definitions_page(&api_config, options)
        .map_err(|e| CliError::NetworkError(format!("failed to retrieve picklists: {e}")))?;

    if let Some(mut items) = result.items {
        for picklist in items.iter_mut() {
            picklist.actions = None;

            picklist.list_type_entries.as_mut().map(|entries| {
                let picklist_name = picklist
                    .external_reference_code
                    .clone()
                    .unwrap_or_default()
                    .to_uppercase();

                entries.iter_mut().for_each(|entry| {
                    let entry_name = entry.key.clone().unwrap_or_default().to_uppercase();
                    entry.external_reference_code = Some(format!("{picklist_name}_{entry_name}"));
                });
                Some(entries)
            });

            let path = Path::new(&output_base).join("picklists");
            let name = picklist.name.as_ref().unwrap();
            fs::create_dir_all(&path)?;

            let filepath = path.join(format!("{name}.json"));
            let mut file = File::create(filepath).unwrap();
            file.write_all(serde_json::to_string_pretty(picklist).unwrap().as_bytes())?;
        }
        println!("Imported {} picklist(s) ", items.len())
    }

    Ok(())
}

fn import_object_definitions(
    output_base: &str,
    username: &str,
    password: &str,
    url: &Url,
    write: bool,
) -> Result<Vec<(String, String)>, CliError> {
    println!("Importing object definitions...");

    let mut api_config = ObjectAdminConfig::new();
    api_config.basic_auth = Some((username.to_owned(), Some(password.to_owned())));
    api_config.update_base_path(url);

    let mut options = PageParams::new();
    options.page = Some(1);
    options.page_size = Some(200);

    let result = get_object_definitions_page(&api_config, options).map_err(|e| {
        CliError::NetworkError(format!(
            "failed to retrieve object definitions from Liferay: {e}"
        ))
    })?;

    if let Some(mut items) = result.items {
        let mut context_paths = Vec::with_capacity(items.len());
        for object_def in items.iter_mut() {
            if let Some(system) = object_def.system {
                if !system {
                    let path = Path::new(&output_base).join("definitions");
                    let name = object_def.name.as_ref().unwrap();

                    if let (Some(context_path), Some(name)) = (
                        object_def.rest_context_path.to_owned(),
                        object_def.name.to_owned(),
                    ) {
                        context_paths.push((context_path, name));
                    }

                    if write {
                        fs::create_dir_all(&path).unwrap();

                        let filepath = path.join(format!("{name}.json"));
                        let mut file = File::create(filepath).unwrap();
                        file.write_all(
                            serde_json::to_string_pretty(object_def).unwrap().as_bytes(),
                        )
                        .expect("Failed to write object definitions");
                    }
                }
            }
        }
        println!(
            "Successfully imported {} object definitions(s)",
            context_paths.len()
        );
        Ok(context_paths)
    } else {
        Ok(Vec::new())
    }
}

fn import_object_data(
    context_paths: Vec<(String, String)>,
    url: &Url,
    output_base: &str,
    username: &str,
    password: &str,
) -> Result<(), CliError> {
    println!("Attemping to import object data...");
    let mut record_count = 0;
    let client = get_object_client()?;

    let data_path = Path::new(&output_base).join("data");
    for (path, name) in context_paths.iter() {
        let url = url
            .join(path)
            .map_err(|_| CliError::InvalidInput(path.to_owned()))?;

        let resp = client
            .get(url)
            .basic_auth(username, Some(password))
            .send()
            .map_err(|e| {
                CliError::NetworkError(format!("failed to retrieve object data from Liferay: {e}"))
            })?;

        let data = resp.json()?;

        match data {
            serde_json::Value::Object(mut object_page) => match object_page.get_mut("items") {
                Some(items) => match items {
                    serde_json::Value::Array(items) => {
                        record_count += items.len();

                        fs::create_dir_all(&data_path).unwrap();

                        let filepath = data_path.join(format!("{name}.json"));
                        let mut file = File::create(filepath).unwrap();
                        file.write_all(serde_json::to_string_pretty(&items).unwrap().as_bytes())
                            .unwrap();
                    }
                    _ => {
                        return Err(CliError::JsonError(
                            "Object items: should be an array but was not".into(),
                        ))
                    }
                },
                None => {
                    println!("An error occured when retrieving object '{name}'. Skipping.")
                }
            },
            _ => {
                return Err(CliError::JsonError(
                    "Could not deserialize object page".into(),
                ))
            }
        };
    }
    println!("Successfully imported {record_count} record(s)");
    Ok(())
}
