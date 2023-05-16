use super::{
    cet_configuration::{CetConfigId, DEFAULT_VIRTUAL_INSTANCE_ID},
    config::Config,
    ClientExt, ConfigFile, ConfigFormat, TemplateContext,
};
use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt::Display};

#[derive(Debug, Serialize, Deserialize)]
#[serde(bound(deserialize = "'de: 'a"))]
pub struct ClientExtensionYaml<'a> {
    assemble: Vec<AssembleMember<'a>>,
    #[serde(flatten)]
    apps: HashMap<String, ClientExtType>,
    #[serde(flatten)]
    other: HashMap<String, serde_yaml::Value>,
}

impl<'a> ClientExtensionYaml<'a> {
    const SHARED_DEP_ASSEMBLE: AssembleMember<'a> = AssembleMember::Files(AssembleFiles {
        from: "sharedDeps",
        include: "*.js",
        into: "static/",
    });

    pub fn add_app(&mut self, definition: ClientExtType) {
        self.apps.insert(definition.get_id(), definition);
    }

    pub fn add_shared_dep_assemble_if_not_exists(&mut self) -> bool {
        if self.assemble.contains(&Self::SHARED_DEP_ASSEMBLE) {
            false
        } else {
            self.add_assemble_member(Self::SHARED_DEP_ASSEMBLE);
            true
        }
    }

    pub fn add_assemble_member(&mut self, assemble_member: AssembleMember<'a>) {
        self.assemble.push(assemble_member)
    }

    pub fn set_dev_urls(mut self, port: u16) -> Self {
        self.apps
            .iter_mut()
            .map(|(key, ext)| {
                match ext {
                    ClientExtType::CustomElement(elem) => {
                        elem.css_urls = Vec::with_capacity(0);

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

impl<'a> ConfigFile<'a> for ClientExtensionYaml<'a> {
    const FILENAME: &'static str = "client-extension.yaml";
    const FORMAT: ConfigFormat = ConfigFormat::Yaml;

    fn add_project_settings<'b: 'a>(
        &mut self,
        _config: &'b Config,
    ) -> Result<(), crate::error::CliError> {
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(bound(deserialize = "'de: 'a"))]
#[serde(untagged)]
pub enum AssembleMember<'a> {
    Task(AssembleTask<'a>),
    Files(AssembleFiles<'a>),
    Unkown(serde_yaml::Value),
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct AssembleFiles<'a> {
    pub from: &'a str,
    pub include: &'a str,
    pub into: &'a str,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AssembleTask<'a> {
    pub from_task: &'a str,
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

    #[serde(rename = "dxp.lxc.liferay.com.virtualInstanceId")]
    instance_id: Option<String>,
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
            description: Some(String::new()),
            urls: Default::default(),
            use_esm: true,
            source_code_url: Default::default(),
            instance_id: Some(String::from(DEFAULT_VIRTUAL_INSTANCE_ID)),
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
        new.add_js_filename(format!("{}.js", new.get_id()));
        new
    }

    pub fn set_instance_id(&mut self, instance_id: &str) {
        self.instance_id = Some(instance_id.to_owned());
    }

    pub fn get_instance_id(&self) -> Option<&String> {
        self.instance_id.as_ref()
    }

    pub fn set_source_code_url(&mut self, source_code_url: String) {
        self.source_code_url = Some(source_code_url);
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

impl ClientExt for CustomElementDefinition {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_id(&self) -> String {
        self.name.to_lowercase().replace(' ', "-")
    }

    fn get_context(&self) -> Vec<(String, String)> {
        vec![
            (
                TemplateContext::NAME_CAMELCASE.into(),
                self.get_camelcase_name(),
            ),
            (TemplateContext::EXT_NAME.into(), self.get_name().into()),
            (
                TemplateContext::ELEMENT_NAME.into(),
                self.get_custom_element_name().into(),
            ),
        ]
    }

    fn get_type_name(&self) -> &'static str {
        "custom_element"
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

#[derive(Debug, Serialize, Deserialize, Default, Clone, ValueEnum)]
#[serde(rename_all = "camelCase")]
pub enum PortletCategoryNames {
    #[serde(rename = "category.client-extensions")]
    #[default]
    ClientExtensions,
}

impl Display for PortletCategoryNames {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PortletCategoryNames::ClientExtensions => write!(f, "category.client-extensions"),
        }
    }
}
