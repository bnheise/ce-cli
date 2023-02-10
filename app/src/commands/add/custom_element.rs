use crate::{
    error::CliError,
    structs::{
        client_extension_yaml::{
            ClientExtId, ClientExtType, ClientExtensionYaml, CustomElementDefinition,
            PortletCategoryNames,
        },
        config::Config,
        ConfigFile,
    },
    ASSETS,
};
use std::{
    fs,
    path::{Path, PathBuf},
};

use super::is_extension_name_valid;

#[allow(clippy::too_many_arguments)]
pub fn handle_custom_element(
    name: String,
    html_element_name: Option<String>,
    friendly_url_mapping: Option<String>,
    instanceable: Option<bool>,
    portlet_category_name: Option<PortletCategoryNames>,
    description: Option<String>,
    use_esm: Option<bool>,
    source_code_url: Option<String>,
) -> Result<(), CliError> {
    if !is_extension_name_valid(&name) {
        return Err(CliError::InvalidExtensionNameError);
    }

    let raw = Config::try_open()?;
    let mut config = Config::try_parse(&raw).map_err(|_e| CliError::InvalidDirectoryError("Could not load workspace-config.json. Are you in the root directory of a ce-cli workspace project?".into()))?;
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

    let app_path = Path::new("./src").join(definition.get_id());

    if app_path.exists() {
        return Err(CliError::ExtensionExistsError);
    }

    config
        .entrypoints
        .insert(definition.get_id(), app_path.to_owned());

    fs::create_dir_all(&app_path).map_err(|e| CliError::WriteError("./src".to_owned(), e))?;

    let custom_element_templates = ASSETS
        .get_dir(
            PathBuf::new()
                .join("app_templates")
                .join(config.framework.to_string())
                .join("custom_element"),
        )
        .expect("Failed to load the custom_element templates folder");

    let context = vec![
        ("{{ appNameCamelcase }}", definition.get_camelcase_name()),
        (
            "{{ customElementAppName }}",
            definition.get_name().to_owned(),
        ),
        (
            "{{ customElementName }}",
            definition.get_custom_element_name().to_owned(),
        ),
    ];

    for file in custom_element_templates.files() {
        let mut content = file
            .contents_utf8()
            .expect("Could not parse template file as utf-8")
            .to_owned();

        let name = match file.path().components().last().unwrap() {
            std::path::Component::Normal(filename) => filename.to_str().unwrap_or_default(),
            _ => unreachable!(),
        };

        for (key, val) in context.iter() {
            content = content.replace(key, val);
        }

        fs::write(app_path.join(name), content)
            .map_err(|e| CliError::WriteError(name.to_owned(), e))?;
    }

    let raw = ClientExtensionYaml::try_open()?;
    let mut client_ext_yaml = ClientExtensionYaml::try_parse(&raw)?;
    client_ext_yaml.add_app(ClientExtType::CustomElement(definition));
    let raw = ClientExtensionYaml::try_serialize(client_ext_yaml)?;
    ClientExtensionYaml::write(raw)?;

    let raw = Config::try_serialize(config)?;
    Config::write(raw)?;

    Ok(())
}
