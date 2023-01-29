use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ClientExtensionYaml {
    assemble: Vec<AssembleMember>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssembleMember {
    from: String,
    include: String,
    into: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExtDefinition {
    #[serde(rename = "cssURLs")]
    css_urls: Vec<String>,

    #[serde(rename = "friendlyURLMapping")]
    friendly_url_mapping: String,

    html_element_name: String,
    instanceable: bool,
    name: String,
    portlet_category_name: PortletCategoryNames,
    properties: HashMap<String, String>,
    descriptions: String,
    urls: Vec<String>,

    #[serde(rename = "useESM")]
    use_esm: bool,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
enum ClientExtType {
    #[default]
    CustomElement,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
enum PortletCategoryNames {
    #[serde(rename = "category.remote-apps")]
    #[default]
    RemoteApps,
}
