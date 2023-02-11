use super::{config::Config, typescript_config_json::TSConfigJson, ClientExt, TemplateContext};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct SharedComponentDefinition {
    name: String,
}

impl SharedComponentDefinition {
    pub fn new(name: String) -> Self {
        Self { name }
    }

    pub fn add_to_typescript_paths(&self, ts_config: &mut TSConfigJson) {
        ts_config.add_path(self.get_id(), self.get_ext_path());
    }

    pub fn add_to_aliases(&self, config: &mut Config) {
        let key = format!("{}$", self.get_id());
        let path = self.get_ext_path().join("index.ts");
        config
            .alias
            .entry(key)
            .and_modify(|aliases| aliases.push(path.to_owned()))
            .or_insert(vec![path]);
    }
}

impl ClientExt for SharedComponentDefinition {
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
        ]
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_type_name(&self) -> &'static str {
        "shared_component"
    }
}
