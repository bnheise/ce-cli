use yaml_rust::{YamlEmitter, YamlLoader};

use crate::{
    config::Config,
    structs::client_extension_yaml::ClientExtType,
    structs::client_extension_yaml::{
        ClientExtId, ClientExtensionYaml, CustomElementDefinition, PortletCategoryNames,
    },
    templates::{
        CLIENT_EXT_YAML_FILENAME, CUSTOM_ELEMENT_APP, CUSTOM_ELEMENT_APP_FILENAME,
        CUSTOM_ELEMENT_APP_NAME, CUSTOM_ELEMENT_APP_NAME_CAMEL, CUSTOM_ELEMENT_CSS,
        CUSTOM_ELEMENT_CSS_FILENAME, CUSTOM_ELEMENT_INDEX, CUSTOM_ELEMENT_INDEX_FILENAME,
        CUSTOM_ELEMENT_NAME, WORKSPACE_CONFIG_FILENAME,
    },
};
use std::{
    fs,
    io::Result,
    path::{Path, PathBuf},
};

pub fn handle_custom_element(
    name: String,
    html_element_name: Option<String>,
    friendly_url_mapping: Option<String>,
    instanceable: Option<bool>,
    portlet_category_name: Option<PortletCategoryNames>,
    description: Option<String>,
    use_esm: Option<bool>,
) -> Result<()> {
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

    let app_path = Path::new("./src").join(definition.get_id());
    fs::create_dir(&app_path)?;

    let index_path = app_path.join(CUSTOM_ELEMENT_INDEX_FILENAME);
    create_app_file(&definition, &app_path)?;
    create_css_file(&definition, &app_path)?;
    create_index_file(&definition, &index_path)?;
    update_workspace_config(&definition, &index_path)?;
    update_client_ext_yaml(definition)?;

    Ok(())
}

fn update_client_ext_yaml(definition: CustomElementDefinition) -> Result<()> {
    let client_ext_yaml_path = Path::new(CLIENT_EXT_YAML_FILENAME);

    let client_ext_yaml = fs::read_to_string(client_ext_yaml_path)
        .expect("Unable to locate client-extension.yaml file");

    let mut deserialized = serde_yaml::from_str::<ClientExtensionYaml>(&client_ext_yaml)
        .expect("Could not parse client-ext.yaml");

    deserialized.add_app(ClientExtType::CustomElement(definition));

    let string = serde_yaml::to_string(&deserialized)
        .expect("It was not possible to stringify the client-extension.yaml data");

    let indent_hack = YamlLoader::load_from_str(&string).unwrap();

    let mut out_str = String::new();
    {
        let mut emitter = YamlEmitter::new(&mut out_str);
        indent_hack
            .iter()
            .for_each(|item| emitter.dump(item).unwrap()); // dump the YAML object to a String
    }

    fs::write(client_ext_yaml_path, out_str)?;

    Ok(())
}

fn update_workspace_config(definition: &CustomElementDefinition, index_path: &Path) -> Result<()> {
    let config_path = Path::new("./").join(WORKSPACE_CONFIG_FILENAME);

    let workspace_config =
        fs::read_to_string(&config_path).expect("Unable to locate client-extension.yaml file");

    let mut deserialized =
        serde_json::from_str::<Config>(&workspace_config).expect("Could not deserialize config");

    deserialized
        .entrypoints
        .insert(definition.get_id(), index_path.to_path_buf());

    fs::write(
        config_path,
        serde_json::to_string_pretty(&deserialized).unwrap(),
    )?;

    Ok(())
}

fn create_index_file(definition: &CustomElementDefinition, index_path: &PathBuf) -> Result<()> {
    let index_content = CUSTOM_ELEMENT_INDEX
        .replace(
            CUSTOM_ELEMENT_APP_NAME_CAMEL,
            &definition.get_camelcase_name(),
        )
        .replace(CUSTOM_ELEMENT_NAME, definition.get_custom_element_name());

    fs::write(index_path, index_content)?;

    Ok(())
}

fn create_css_file(definition: &CustomElementDefinition, app_path: &Path) -> Result<()> {
    let css_content =
        CUSTOM_ELEMENT_CSS.replace(CUSTOM_ELEMENT_NAME, definition.get_custom_element_name());

    fs::write(app_path.join(CUSTOM_ELEMENT_CSS_FILENAME), css_content)?;
    Ok(())
}

fn create_app_file(definition: &CustomElementDefinition, app_path: &Path) -> Result<()> {
    let app_content = CUSTOM_ELEMENT_APP
        .replace(
            CUSTOM_ELEMENT_APP_NAME_CAMEL,
            &definition.get_camelcase_name(),
        )
        .replace(CUSTOM_ELEMENT_APP_NAME, definition.get_name());

    fs::write(app_path.join(CUSTOM_ELEMENT_APP_FILENAME), app_content)?;
    Ok(())
}
