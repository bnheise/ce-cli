use super::is_extension_name_valid;
use crate::{
    assets_dir::AssetsDir,
    cli::CustomElementArgs,
    config_generators::{
        client_extension_yaml::{ClientExtType, ClientExtensionYaml, CustomElementDefinition},
        config::Config,
        ClientExt, ConfigFile,
    },
    error::CliError,
};

#[allow(clippy::too_many_arguments)]
pub fn handle_custom_element(args: CustomElementArgs) -> Result<(), CliError> {
    let CustomElementArgs {
        name,
        friendly_url_mapping,
        html_element_name,
        instanceable,
        portlet_category_name,
        description,
        use_esm,
        source_code_url,
        ..
    } = args;
    if !is_extension_name_valid(&name) {
        return Err(CliError::InvalidExtensionName);
    }

    let raw = Config::try_open()?;
    let mut config = Config::try_parse(&raw).map_err(|_e| CliError::InvalidDirectory("Could not load workspace-config.json. Are you in the root directory of a ce-cli workspace project?".into()))?;
    let mut definition = CustomElementDefinition::new(name);

    if let Some(friendly_url_mapping) = friendly_url_mapping {
        definition.set_friendly_url_mapping(friendly_url_mapping);
    }

    if let Some(html_element_name) = html_element_name {
        definition.set_html_element_name(html_element_name);
    }

    if let Some(instanceable) = instanceable {
        definition.set_instanceable(instanceable);
    }

    if let Some(portlet_category_name) = portlet_category_name {
        definition.set_portlet_category_name(portlet_category_name);
    }

    if let Some(description) = description {
        definition.set_description(description);
    } else {
        definition.set_description("".to_string()) // Hack to get around the differences between Liferay workspace yaml parser and the Rust yaml parser
    }

    if let Some(use_esm) = use_esm {
        definition.set_use_esm(use_esm);
    }

    if let Some(source_code_url) = source_code_url {
        definition.set_source_code_url(source_code_url);
    } else {
        definition.set_source_code_url("".to_string());
    }

    definition.set_instance_id(&config.default_instance_id);

    definition.add_to_entrypoints(&mut config);

    definition.initialize_directories()?;

    AssetsDir::initialize_templates(&config, &definition)?;

    let raw = ClientExtensionYaml::try_open()?;
    let mut client_ext_yaml = ClientExtensionYaml::try_parse(&raw)?;
    client_ext_yaml.add_app(ClientExtType::CustomElement(definition));
    let raw = ClientExtensionYaml::try_serialize(client_ext_yaml)?;
    ClientExtensionYaml::write(raw)?;

    let raw = Config::try_serialize(config)?;
    Config::write(raw)?;

    Ok(())
}
