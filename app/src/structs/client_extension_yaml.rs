use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt::Display};

use super::cet_configuration::CetConfigId;

#[derive(Debug, Serialize, Deserialize)]
pub struct ClientExtensionYaml {
    assemble: Vec<AssembleMember>,

    #[serde(flatten)]
    apps: HashMap<String, ClientExtType>,
}

impl ClientExtensionYaml {
    pub fn add_app(&mut self, definition: ClientExtType) {
        self.apps.insert(definition.get_id(), definition);
    }

    pub fn set_dev_urls(mut self, port: u16) -> Self {
        self.apps
            .iter_mut()
            .map(|(key, ext)| {
                match ext {
                    ClientExtType::CustomElement(elem) => {
                        elem.css_urls = elem
                            .css_urls
                            .iter()
                            .map(|url| format!("http://localhost:{port}{url}"))
                            .collect::<Vec<_>>();

                        elem.urls = elem
                            .urls
                            .iter()
                            .map(|url| format!("http://localhost:{port}{url}"))
                            .collect::<Vec<_>>();
                        elem
                    }
                };
                (key, ext)
            })
            .fold(HashMap::new(), |mut hashmap, (key, ext)| {
                hashmap.insert(key, ext);
                hashmap
            });

        self
    }

    pub fn get_apps(&self) -> &HashMap<String, ClientExtType> {
        &self.apps
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssembleMember {
    from: String,
    include: String,
    into: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CustomElementDefinition {
    #[serde(rename = "cssURLs")]
    css_urls: Vec<String>,

    #[serde(rename = "friendlyURLMapping")]
    friendly_url_mapping: Option<String>,

    html_element_name: String,
    instanceable: bool,
    name: String,
    portlet_category_name: PortletCategoryNames,
    properties: Option<HashMap<String, String>>,
    description: Option<String>,
    urls: Vec<String>,

    #[serde(rename = "useESM")]
    use_esm: bool,
    #[serde(rename = "sourceCodeURL")]
    source_code_url: Option<String>,
}

impl Default for CustomElementDefinition {
    fn default() -> Self {
        Self {
            css_urls: Default::default(),
            friendly_url_mapping: Default::default(),
            html_element_name: Default::default(),
            instanceable: false,
            name: Default::default(),
            portlet_category_name: Default::default(),
            properties: Some(HashMap::new()),
            description: None,
            urls: Default::default(),
            use_esm: true,
            source_code_url: Default::default(),
        }
    }
}

impl CustomElementDefinition {
    pub fn new(name: String) -> Self {
        let mut new = Self {
            name,
            ..Default::default()
        };
        new.set_friendly_url_mapping(new.get_id());
        new.set_html_element_name(new.get_id());
        new.add_css_filename(format!("{}.css", new.get_id()));
        new.add_js_filename(format!("{}.js", new.get_id()));
        new
    }

    pub fn set_friendly_url_mapping(&mut self, friendly_url_mapping: String) {
        self.friendly_url_mapping = Some(friendly_url_mapping);
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

    pub fn get_custom_element_name(&self) -> &str {
        &self.html_element_name
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_descripton(&self) -> Option<&String> {
        self.description.as_ref()
    }

    pub fn add_css_filename(&mut self, filename: String) {
        self.css_urls.push(format!("/{filename}"));
    }

    pub fn add_js_filename(&mut self, filename: String) {
        self.urls.push(format!("/{filename}"));
    }

    pub fn get_source_code_url(&self) -> Option<&String> {
        self.source_code_url.as_ref()
    }

    pub fn get_friendly_url_mapping(&self) -> Option<&String> {
        self.friendly_url_mapping.as_ref()
    }

    pub fn get_camelcase_name(&self) -> String {
        self.get_name()
            .split(' ')
            .map(|part| part[0..1].to_uppercase() + &part[1..])
            .collect::<String>()
    }

    pub fn get_properties(&self) -> Option<&HashMap<String, String>> {
        self.properties.as_ref()
    }

    pub fn get_instanceable(&self) -> bool {
        self.instanceable
    }

    pub fn get_js_urls(&self) -> &Vec<String> {
        &self.urls
    }

    pub fn get_use_esm(&self) -> bool {
        self.use_esm
    }

    pub fn get_html_element_name(&self) -> &str {
        self.html_element_name.as_str()
    }

    pub fn get_css_urls(&self) -> &Vec<String> {
        &self.css_urls
    }

    pub fn get_portlet_category_name(&self) -> &PortletCategoryNames {
        &self.portlet_category_name
    }
}

impl ClientExtId for CustomElementDefinition {
    fn get_id(&self) -> String {
        self.name.to_lowercase().replace(' ', "-")
    }
}

impl CetConfigId for CustomElementDefinition {}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum ClientExtType {
    CustomElement(CustomElementDefinition),
}

impl ClientExtType {
    pub fn get_id(&self) -> String {
        match self {
            ClientExtType::CustomElement(app) => app.get_id(),
        }
    }

    pub fn get_cet_config_id(&self) -> String {
        match self {
            ClientExtType::CustomElement(app) => app.get_cet_config_id(),
        }
    }
}

pub trait ClientExtId {
    fn get_id(&self) -> String;
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, ValueEnum)]
#[serde(rename_all = "camelCase")]
pub enum PortletCategoryNames {
    #[serde(rename = "category.remote-apps")]
    #[default]
    RemoteApps,
}

impl Display for PortletCategoryNames {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PortletCategoryNames::RemoteApps => write!(f, "category.remote-apps"),
        }
    }
}
