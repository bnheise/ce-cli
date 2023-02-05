use super::client_extension_yaml::{ClientExtId, ClientExtensionYaml, CustomElementDefinition};
use serde::Serialize;
use std::collections::HashMap;

pub const CET_CONFIG_FULLY_QUALIFIED_PATH: &str =
    "com.liferay.client.extension.type.configuration.CETConfiguration";

#[derive(Debug, Serialize)]
pub struct CetConfiguration {
    #[serde(flatten)]
    apps: HashMap<String, CetDefinition>,
}

impl From<ClientExtensionYaml> for CetConfiguration {
    fn from(value: ClientExtensionYaml) -> Self {
        let apps = value
            .get_apps()
            .iter()
            .map(|(_, val)| (val.get_cet_config_id(), CetDefinition::from(val.clone())))
            .fold(HashMap::new(), |mut map, (key, val)| {
                map.insert(key, val);
                map
            });
        Self { apps }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CetDefinition {
    #[serde(rename = "baseURL")]
    base_url: String,
    description: String,
    #[serde(rename = "dxp.lxc.liferay.com.virtualInstanceId")]
    virtual_instance_id: String,
    name: String,
    properties: Vec<Attribute>,
    #[serde(rename = "sourceCodeURL")]
    sourcecode_url: String,
    _type: ClientExtType,
    type_settings: Vec<Attribute>,
}

impl Default for CetDefinition {
    fn default() -> Self {
        Self {
            base_url: Default::default(),
            description: Default::default(),
            virtual_instance_id: "default".to_string(),
            name: Default::default(),
            properties: Default::default(),
            sourcecode_url: Default::default(),
            _type: Default::default(),
            type_settings: Default::default(),
        }
    }
}

impl From<CustomElementDefinition> for CetDefinition {
    fn from(value: CustomElementDefinition) -> Self {
        let props = value.get_properties().map(|map| {
            map.iter()
                .map(|(key, val)| Attribute::new(key.to_owned(), val.to_owned()))
                .collect::<Vec<_>>()
        });

        let friendly_url_mapping = value
            .get_friendly_url_mapping()
            .map(|url| Attribute::new("friendlyURLMapping".to_string(), url.to_owned()));

        let instanceable = Some(Attribute::new(
            "instanceable".to_string(),
            value.get_instanceable().to_string(),
        ));

        let urls = Some(Attribute::new(
            "urls".to_string(),
            value.get_js_urls().join("\n"),
        ));

        let css_urls = Some(Attribute::new(
            "cssURLs".to_string(),
            value.get_css_urls().join("\n"),
        ));

        let use_esm = Some(Attribute::new(
            "useESM".to_string(),
            value.get_use_esm().to_string(),
        ));

        let html_element_name = Some(Attribute::new(
            "htmlElementName".to_string(),
            value.get_html_element_name().to_owned(),
        ));

        let portlet_category_name = Some(Attribute::new(
            "portletCategoryName".to_string(),
            value.get_portlet_category_name().to_string(),
        ));

        Self {
            base_url: format!("${{portalURL}}/o/{}", value.get_id()),
            description: value.get_descripton().unwrap_or(&"".to_string()).to_owned(),
            name: value.get_name().to_owned(),
            properties: props.unwrap_or_default(),
            sourcecode_url: value
                .get_source_code_url()
                .unwrap_or(&"".to_string())
                .to_owned(),
            _type: ClientExtType::CustomElement,
            type_settings: vec![
                friendly_url_mapping,
                instanceable,
                urls,
                use_esm,
                html_element_name,
                css_urls,
                portlet_category_name,
            ]
            .into_iter()
            .flatten()
            .collect::<Vec<_>>(),
            ..Default::default()
        }
    }
}

impl From<super::client_extension_yaml::ClientExtType> for CetDefinition {
    fn from(value: super::client_extension_yaml::ClientExtType) -> Self {
        match value {
            super::client_extension_yaml::ClientExtType::CustomElement(custom_element) => {
                CetDefinition::from(custom_element)
            }
        }
    }
}

#[derive(Debug, Serialize, Default)]
#[serde(rename_all = "camelCase")]
enum ClientExtType {
    #[default]
    CustomElement,
}

#[derive(Debug)]
pub struct Attribute {
    key: String,
    value: String,
}

impl Attribute {
    pub fn new(key: String, value: String) -> Self {
        Self { key, value }
    }
}

impl Serialize for Attribute {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let string = format!("{}={}", self.key, self.value);
        serializer.serialize_str(&string)
    }
}

pub trait CetConfigId: ClientExtId {
    fn get_cet_config_id(&self) -> String {
        format!("{CET_CONFIG_FULLY_QUALIFIED_PATH}~{}", self.get_id())
    }
}
