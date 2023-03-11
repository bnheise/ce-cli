use super::{
    get_list_type_definitions_page, get_object_definitions_page, prepare_data_path, ApiConfig,
    ListTypeConfig, ObjectAdminConfig,
};
use crate::{
    cli::ImportObjectArgs,
    config_generators::{config::Config, ConfigFile},
    error::CliError,
};
use reqwest::{blocking, Url};
use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};

use super::{initialize_param, prepare_url};

pub fn handle_import(args: ImportObjectArgs) -> Result<(), CliError> {
    let ImportObjectArgs { all, .. } = &args;

    if *all {
        handle_import_all(args)?;
    } else {
        todo!("Handle cases where -a wasn't passed")
        // let erc = erc.as_ref().expect("Erc should have been provided.");
        // let source = &source.as_ref().expect("Source should have been provided");
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

    let username = initialize_param(username, "LIFERAY_USERNAME", "username")?;
    let password = initialize_param(password, "LIFERAY_PASSWORD", "password")?;

    let config = Config::try_open();
    let url = prepare_url(url, port, config.is_ok())?;
    let output_base = prepare_data_path(output, config.is_ok())?;

    let mut api_config = ObjectAdminConfig::new();
    api_config.basic_auth = Some((username.clone(), Some(password.clone())));

    if let Some(url) = &url {
        api_config.update_base_path(url);
    }

    let context_paths = import_object_definitions(&api_config, &output_base);

    import_object_data(
        context_paths,
        &url,
        &api_config,
        &output_base,
        &username,
        &password,
    )?;

    import_picklists(&username, &password, &url, &output_base)?;

    Ok(())
}

fn import_picklists(
    username: &str,
    password: &str,
    url: &Option<Url>,
    output_base: &str,
) -> Result<(), CliError> {
    println!("Importing picklist data...");
    let mut api_config = ListTypeConfig::new();
    api_config.basic_auth = Some((username.to_owned(), Some(password.to_owned())));

    if let Some(url) = url {
        api_config.update_base_path(url);
    }

    let result =
        get_list_type_definitions_page(&api_config, None, None, Some("1"), Some("200"), None, None)
            .map_err(|_| CliError::GetPicklist("failed to retrieve picklists"))?;

    if let Some(mut items) = result.items {
        for picklist in items.iter_mut() {
            picklist.actions = None;
            let path = Path::new(&output_base).join("picklists");
            let name = picklist.name.as_ref().unwrap();
            fs::create_dir_all(&path).unwrap();

            let filepath = path.join(format!("{name}.json"));
            let mut file = File::create(filepath).unwrap();
            file.write_all(serde_json::to_string_pretty(picklist).unwrap().as_bytes())
                .unwrap();
        }
        println!("Imported {} picklist(s) ", items.len())
    }

    Ok(())
}

fn get_object_client() -> Result<blocking::Client, CliError> {
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

fn prepare_base_url(api_config: &ObjectAdminConfig, url: &Option<Url>) -> String {
    let url = if let Some(url) = url.clone() {
        url
    } else {
        Url::parse(&api_config.base_path).unwrap()
    };

    match url.origin() {
        url::Origin::Opaque(_) => unreachable!(),
        url::Origin::Tuple(scheme, host, port) => format!("{scheme}://{host}:{port}"),
    }
}

fn import_object_definitions(
    api_config: &ObjectAdminConfig,
    output_base: &str,
) -> Vec<(String, String)> {
    println!("Importing object definitions...");
    let result =
        get_object_definitions_page(api_config, None, None, Some("1"), Some("200"), None, None)
            .unwrap();

    if let Some(mut items) = result.items {
        let mut context_paths = Vec::with_capacity(items.len());
        for object_def in items.iter_mut() {
            if let Some(system) = object_def.system {
                if !system {
                    let path = Path::new(&output_base).join("definitions");
                    let name = object_def.name.as_ref().unwrap();
                    object_def.actions = None;
                    object_def.id = None;
                    if let (Some(context_path), Some(name)) = (
                        object_def.rest_context_path.to_owned(),
                        object_def.name.to_owned(),
                    ) {
                        context_paths.push((context_path, name));
                    }

                    fs::create_dir_all(&path).unwrap();

                    let filepath = path.join(format!("{name}.json"));
                    let mut file = File::create(filepath).unwrap();
                    file.write_all(serde_json::to_string_pretty(object_def).unwrap().as_bytes())
                        .unwrap();
                }
            }
        }
        println!(
            "Successfully imported {} object definitions(s)",
            context_paths.len()
        );
        context_paths
    } else {
        Vec::new()
    }
}

fn import_object_data(
    context_paths: Vec<(String, String)>,
    url: &Option<Url>,
    api_config: &ObjectAdminConfig,
    output_base: &str,
    username: &str,
    password: &str,
) -> Result<(), CliError> {
    println!("Attemping to import object data...");
    let mut record_count = 0;
    let client = get_object_client()?;
    println!("{client:?}");
    let base_url = prepare_base_url(api_config, url);

    for (path, name) in context_paths.iter() {
        let data_path = Path::new(&output_base).join("data");
        let url = format!("{base_url}{path}");

        let resp = client
            .get(url)
            .basic_auth(username, Some(password))
            .send()
            .map_err(|e| {
                CliError::NetworkError("Failed to retrieve object data from Liferay", e)
            })?;

        let data = resp.json().map_err(|e| {
            CliError::NetworkError("Could not parse response from Liferay as json", e)
        })?;

        match data {
            serde_json::Value::Object(mut object_page) => match object_page.get_mut("items") {
                Some(items) => match items {
                    serde_json::Value::Array(items) => {
                        for item in items.iter_mut() {
                            record_count += 1;
                            if let serde_json::Value::Object(obj) = item {
                                obj.remove("actions");
                            }
                        }

                        fs::create_dir_all(&data_path).unwrap();

                        let filepath = data_path.join(format!("{name}.json"));
                        let mut file = File::create(filepath).unwrap();
                        file.write_all(serde_json::to_string_pretty(&items).unwrap().as_bytes())
                            .unwrap();
                    }
                    _ => {
                        return Err(CliError::InvalidJson(
                            "Object items: should be an array but was not",
                        ))
                    }
                },
                None => {
                    println!("An error occured when retrieving object '{name}'. Skipping.")
                }
            },
            _ => return Err(CliError::InvalidJson("object page")),
        };
    }
    println!("Successfully imported {record_count} record(s)");
    Ok(())
}
