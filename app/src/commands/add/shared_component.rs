use super::is_extension_name_valid;
use crate::assets_dir::AssetsDir;
use crate::config_generators::shared_component::SharedComponentDefinition;
use crate::config_generators::typescript_config_json::TSConfigJson;
use crate::config_generators::{ClientExt, ConfigFile};
use crate::{config_generators::config::Config, error::CliError};

pub fn handle_shared_component(name: String) -> Result<(), CliError> {
    if !is_extension_name_valid(&name) {
        return Err(CliError::InvalidExtensionName);
    }

    let raw = Config::try_open()?;
    let mut config = Config::try_parse(&raw)?;
    let component_def = SharedComponentDefinition::new(name);

    component_def.add_to_entrypoints(&mut config);
    component_def.add_to_externals(&mut config);
    component_def.add_to_aliases(&mut config);
    component_def.initialize_directories()?;

    let raw_tsconfig = TSConfigJson::try_open()?;
    let mut ts_config = TSConfigJson::try_parse(&raw_tsconfig)?;
    component_def.add_to_typescript_paths(&mut ts_config);

    AssetsDir::initialize_templates(&config, &component_def)?;

    let raw = Config::try_serialize(config)?;
    Config::write(raw)?;
    let raw_tsconfig = TSConfigJson::try_serialize(ts_config)?;
    TSConfigJson::write(raw_tsconfig)?;

    Ok(())
}
