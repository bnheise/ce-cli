use crate::{
    cli::ClientExtType,
    structs::client_extension_yaml::{
        ClientExtensionYaml, CustomElementDefinition, PortletCategoryNames,
    },
    templates::CLIENT_EXT_YAML_FILENAME,
};
use std::{fs, io::Result, path::Path};

pub fn handle_add(extension_type: ClientExtType) -> Result<()> {
    match extension_type {
        ClientExtType::RemoteApp {
            name,
            html_element_name,
            friendly_url_mapping,
            instanceable,
            portlet_category_name,
            description,
            use_esm,
        } => handle_custom_element(
            name,
            html_element_name,
            friendly_url_mapping,
            instanceable,
            portlet_category_name,
            description,
            use_esm,
        )?,
    }

    Ok(())
}

fn handle_custom_element(
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
    }

    if let Some(use_esm) = use_esm {
        definition.set_use_esm(use_esm);
    }
    println!("HERE");
    let client_ext_yaml = fs::read_to_string(Path::new(CLIENT_EXT_YAML_FILENAME))
        .expect("Unable to locate client-extension.yaml file");
    println!("HERE");
    let deserialized = serde_yaml::from_str::<ClientExtensionYaml>(&client_ext_yaml).unwrap();
    println!("{:?}", deserialized);
    todo!("Get info for custom element from command line or provide defaults");
    Ok(())
}
