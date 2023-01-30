use std::io::Result;

use crate::templates::CLIENT_EXT_YAML_FILENAME;

use super::{
    format_yaml, get_client_ext_yaml, get_client_extension_yaml_path, get_config, get_config_path,
    write_file_to_build_dir,
};

pub fn handle_dev_deploy() -> Result<()> {
    let path = get_config_path();
    let port = get_config(&path).dev_server_port;
    let path = get_client_extension_yaml_path();
    let client_ext_yaml = get_client_ext_yaml(&path).set_dev_urls(port);
    let yaml = format_yaml(client_ext_yaml);
    write_file_to_build_dir(CLIENT_EXT_YAML_FILENAME, yaml)?;
    Ok(())
}
