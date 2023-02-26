use super::{config::Config, ConfigFile, ConfigFormat};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TSConfigJson {
    compiler_options: CompilerOptions,
    #[serde(flatten)]
    rest: HashMap<String, serde_json::Value>,
}

impl TSConfigJson {
    pub fn add_path(&mut self, key: String, path: PathBuf) {
        self.compiler_options
            .paths
            .entry(key)
            .and_modify(|paths| paths.push(path.to_owned()))
            .or_insert(vec![path]);
    }
}

impl<'a> ConfigFile<'a> for TSConfigJson {
    const FILENAME: &'static str = "tsconfig.json";

    const FORMAT: ConfigFormat = ConfigFormat::Json;

    fn add_project_settings<'b: 'a>(
        &mut self,
        _config: &'b Config,
    ) -> Result<(), crate::error::CliError> {
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompilerOptions {
    paths: HashMap<String, Vec<PathBuf>>,
    #[serde(flatten)]
    rest: HashMap<String, serde_json::Value>,
}
