use std::collections::HashMap;

use clap::ValueEnum;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ClientExtensionYaml {
    assemble: Vec<AssembleMember>,

    #[serde(flatten)]
    apps: HashMap<String, ClientExtType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssembleMember {
    from: String,
    include: String,
    into: String,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CustomElementDefinition {
    #[serde(rename = "cssURLs")]
    css_urls: Vec<String>,

    #[serde(rename = "friendlyURLMapping")]
    friendly_url_mapping: String,

    html_element_name: String,
    instanceable: bool,
    name: String,
    portlet_category_name: PortletCategoryNames,
    properties: HashMap<String, String>,
    description: Option<String>,
    urls: Vec<String>,

    #[serde(rename = "useESM")]
    use_esm: bool,
}

impl CustomElementDefinition {
    pub fn new(name: String) -> Self {
        let url_friendly = name.to_lowercase().replace(' ', "-");
        Self {
            name,
            friendly_url_mapping: url_friendly.to_owned(),
            html_element_name: url_friendly,
            ..Default::default()
        }
    }

    pub fn set_friendly_url_mapping(&mut self, friendly_url_mapping: String) {
        self.friendly_url_mapping = friendly_url_mapping;
    }

    pub fn set_html_element_name(&mut self, html_element_name: String) {
        self.html_element_name = html_element_name;
    }

    pub fn set_instanceable(&mut self, instanceable: bool) {
        self.instanceable = instanceable;
    }

    pub fn set_portlet_category_name(&mut self, portlet_category_name: PortletCategoryNames) {
        self.portlet_category_name = portlet_category_name;
    }

    pub fn set_description(&mut self, description: String) {
        self.description = Some(description);
    }

    pub fn set_use_esm(&mut self, use_esm: bool) {
        self.use_esm = use_esm;
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
pub enum ClientExtType {
    CustomElement(CustomElementDefinition),
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, ValueEnum)]
#[serde(rename_all = "camelCase")]
pub enum PortletCategoryNames {
    #[serde(rename = "category.remote-apps")]
    #[default]
    RemoteApps,
}
