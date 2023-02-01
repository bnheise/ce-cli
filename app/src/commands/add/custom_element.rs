use crate::{
    commands::{
        format_yaml, get_client_ext_yaml, get_client_extension_yaml_path, update_workspace_config,
    },
    structs::client_extension_yaml::ClientExtType,
    structs::client_extension_yaml::{ClientExtId, CustomElementDefinition, PortletCategoryNames},
    templates::app_templates::custom_element::{
        CUSTOM_ELEMENT_APP, CUSTOM_ELEMENT_APP_FILENAME, CUSTOM_ELEMENT_APP_NAME,
        CUSTOM_ELEMENT_APP_NAME_CAMEL, CUSTOM_ELEMENT_CSS, CUSTOM_ELEMENT_CSS_FILENAME,
        CUSTOM_ELEMENT_INDEX, CUSTOM_ELEMENT_INDEX_FILENAME, CUSTOM_ELEMENT_NAME,
        CUSTOM_ELEMENT_VIEW, CUSTOM_ELEMENT_VIEW_FILENAME, CUSTOM_ELEMENT_WIDGET,
        CUSTOM_ELEMENT_WIDGET_FILENAME,
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
    let view_path = app_path.join(CUSTOM_ELEMENT_VIEW_FILENAME);
    let widget_path = app_path.join(CUSTOM_ELEMENT_WIDGET_FILENAME);

    create_custom_element_file(&definition, &app_path)?;

    create_css_file(&definition, &app_path)?;
    create_index_file(&definition, &index_path)?;
    create_view_file(&definition, &view_path)?;
    create_widget_file(&widget_path)?;

    update_workspace_config(|config| {
        config
            .entrypoints
            .insert(definition.get_id(), index_path.to_path_buf());
    })?;

    update_client_ext_yaml(definition)?;

    Ok(())
}

fn update_client_ext_yaml(definition: CustomElementDefinition) -> Result<()> {
    let path = get_client_extension_yaml_path();

    let mut deserialized = get_client_ext_yaml(&path);

    deserialized.add_app(ClientExtType::CustomElement(definition));

    let yaml = format_yaml(deserialized);

    fs::write(path, yaml)?;

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

fn create_custom_element_file(definition: &CustomElementDefinition, app_path: &Path) -> Result<()> {
    let app_content = CUSTOM_ELEMENT_APP
        .replace(
            CUSTOM_ELEMENT_APP_NAME_CAMEL,
            &definition.get_camelcase_name(),
        )
        .replace(CUSTOM_ELEMENT_APP_NAME, definition.get_name());

    fs::write(app_path.join(CUSTOM_ELEMENT_APP_FILENAME), app_content)?;
    Ok(())
}

fn create_view_file(definition: &CustomElementDefinition, view_path: &Path) -> Result<()> {
    let view_content = CUSTOM_ELEMENT_VIEW.replace(CUSTOM_ELEMENT_APP_NAME, definition.get_name());

    fs::write(view_path, view_content)?;
    Ok(())
}

fn create_widget_file(widget_path: &Path) -> Result<()> {
    fs::write(widget_path, CUSTOM_ELEMENT_WIDGET)?;
    Ok(())
}
