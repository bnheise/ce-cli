use super::{
    get_client_ext_yaml, get_client_extension_yaml_path, get_config, get_config_path,
    write_file_to_build_dir,
};
use crate::{
    error::CliError,
    structs::cet_configuration::CetConfiguration,
    templates::{BUILD_DIR, CET_CONFIG_FILENAME_BASE, CLIENT_EXTENSION, CLIENT_EXTENSIONS, OSGI},
    util::zip::zip_directory,
};
use std::path::Path;

pub fn handle_dev_deploy() -> Result<(), CliError> {
    let path = get_config_path();
    let config = get_config(&path);
    let port = config.dev_server_port;
    let project_name = config.project_name;
    let bundle_path = config.bundle_path;
    let path = get_client_extension_yaml_path();
    let client_ext_yaml = get_client_ext_yaml(&path).set_dev_urls(port);
    let cet_config = CetConfiguration::from(client_ext_yaml);
    let raw =
        serde_json::to_string_pretty(&cet_config).expect("Could not serialize CET Configuration");

    let filename = format!("{project_name}.{CET_CONFIG_FILENAME_BASE}");

    let output_dir = Path::new("./").join(CLIENT_EXTENSION);

    write_file_to_build_dir(&filename, CLIENT_EXTENSION, raw)?;

    let zip_dest = bundle_path
        .join(OSGI)
        .join(CLIENT_EXTENSIONS)
        .join(format!("{project_name}.zip"));

    let src_dir = Path::new("./").join(BUILD_DIR).join(output_dir);
    zip_directory(src_dir, zip_dest)?;
    Ok(())
}
