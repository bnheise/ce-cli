use super::{initialize_param, prepare_url};
use super::{prepare_data_path, ApiConfig, ObjectAdminConfig};
use crate::{
    cli::ExportObjectArgs,
    config_generators::{config::Config, ConfigFile},
    error::CliError,
};
use object_admin::apis::object_definition_api::{
    post_object_definition_batch, put_object_definition_batch,
    put_object_definition_by_external_reference_code,
};
use object_admin::models::{CreationStrategy, ObjectDefinition};
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

    let mut api_config = ObjectAdminConfig::new();
    api_config.basic_auth = Some((username, Some(password)));

    if let Some(url) = &url {
        api_config.update_base_path(url);
    }

    let base_data_dir = prepare_data_path(directory, config.is_ok())?;

    let mut object_definitions = Vec::new();

    let definitions_path = Path::new(&base_data_dir).join("definitions");
    println!("Loading object definitions...");
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
        Some(body),
        Some(CreationStrategy::Upsert),
    )
    .map_err(|e| match e {
        object_admin::apis::Error::Reqwest(e) => CliError::NetworkError("Batch request failed", e),
        object_admin::apis::Error::Serde(e) => {
            CliError::SerializeJson("Serialization error occurred during batch request", e)
        }
        object_admin::apis::Error::Io(_) => {
            todo!()
        }
        object_admin::apis::Error::ResponseError(_) => {
            println!("{e:?}");
            todo!()
        }
    })?;

    println!(
        "Request sent. Batch operation erc is {}",
        response.external_reference_code.unwrap_or_default()
    );

    Ok(())
}
