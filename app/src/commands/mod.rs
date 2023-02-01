use std::{
    fs,
    io::Result,
    path::{Path, PathBuf},
};

use serde::Serialize;
use yaml_rust::{YamlEmitter, YamlLoader};

use crate::{
    config::Config,
    structs::client_extension_yaml::ClientExtensionYaml,
    templates::{BUILD_DIR, CLIENT_EXT_YAML_FILENAME, WORKSPACE_CONFIG_FILENAME},
};

pub mod add;
pub mod dev_deploy;
pub mod init;

fn get_config_path() -> PathBuf {
    Path::new("./").join(WORKSPACE_CONFIG_FILENAME)
}

fn get_config(path: &PathBuf) -> Config {
    let raw = fs::read_to_string(path).expect("Unable to locate client-extension.yaml file");

    serde_json::from_str::<Config>(&raw).expect("Could not deserialize config")
}

fn update_workspace_config(cb: impl Fn(&mut Config)) -> Result<()> {
    let path = get_config_path();
    let mut config = get_config(&path);

    cb(&mut config);

    fs::write(
        path,
        serde_json::to_string_pretty(&config).expect("Failed to write workspace-config.json"),
    )?;

    Ok(())
}

fn get_client_extension_yaml_path() -> PathBuf {
    Path::new(CLIENT_EXT_YAML_FILENAME).to_path_buf()
}

fn get_client_ext_yaml(path: &PathBuf) -> ClientExtensionYaml {
    let client_ext_yaml =
        fs::read_to_string(path).expect("Unable to locate client-extension.yaml file");

    serde_yaml::from_str::<ClientExtensionYaml>(&client_ext_yaml)
        .expect("Could not parse client-ext.yaml")
}

fn write_file_to_build_dir(filename: &str, subfolder: &str, content: String) -> Result<()> {
    let path = Path::new("./").join(BUILD_DIR).join(subfolder);
    fs::create_dir_all(&path)?;
    fs::write(path.join(filename), content)?;

    Ok(())
}

fn format_yaml<T: Serialize>(serializable: T) -> String {
    let string = serde_yaml::to_string(&serializable)
        .expect("It was not possible to stringify the client-extension.yaml data");

    let indent_hack = YamlLoader::load_from_str(&string).unwrap();

    let mut out_str = String::new();
    {
        let mut emitter = YamlEmitter::new(&mut out_str);
        indent_hack
            .iter()
            .for_each(|item| emitter.dump(item).unwrap()); // dump the YAML object to a String
    }

    out_str
}
