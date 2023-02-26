use crate::{
    config_generators::{
        cet_configuration::CetConfiguration, client_extension_yaml::ClientExtensionYaml,
        config::Config, AppDir, BuildDir, ConfigFile,
    },
    error::CliError,
    util::zip::zip_directory,
};
use std::path::Path;

pub fn handle_dev_deploy() -> Result<(), CliError> {
    let raw = Config::try_open()?;
    let config = Config::try_parse(&raw)?;
    let port = config.dev_server_port;
    let project_name = config.project_name;
    let deploy_path = config.deploy_path;

    let raw = ClientExtensionYaml::try_open()?;
    let client_ext_yaml = ClientExtensionYaml::try_parse(&raw)?.set_dev_urls(port);
    let cet_config = CetConfiguration::from(client_ext_yaml);

    let content =
        serde_json::to_string_pretty(&cet_config).expect("Could not serialize CET Configuration");

    let filename = format!("{project_name}.client-extension-config.json");

    BuildDir::write_file(&filename, &content, Some("clientExtension"))?;

    let zip_dest = deploy_path.join(format!("{project_name}.zip"));

    let src_dir = Path::new("./").join("build").join("clientExtension");
    zip_directory(src_dir, zip_dest)?;
    Ok(())
}
