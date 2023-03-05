use crate::cli::FrameworkOption;

use super::{config::Config, ConfigFile, ConfigFormat, FrameworkConfigurable};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TSConfigProdJson {
    compiler_options: CompilerOptions,
    #[serde(flatten)]
    rest: HashMap<String, serde_json::Value>,
}

impl<'a> ConfigFile<'a> for TSConfigProdJson {
    const FILENAME: &'static str = "tsconfig.prod.json";

    const FORMAT: ConfigFormat = ConfigFormat::Json;

    fn add_project_settings<'b: 'a>(
        &mut self,
        _config: &'b Config,
    ) -> Result<(), crate::error::CliError> {
        Ok(())
    }
}

impl FrameworkConfigurable for TSConfigProdJson {
    fn set_framework_settings(&mut self, framework: FrameworkOption) {
        if framework == FrameworkOption::Vue {
            self.compiler_options.no_unused_locals = false;
            self.compiler_options.no_unused_parameters = false;
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompilerOptions {
    no_unused_locals: bool,
    no_unused_parameters: bool,
    #[serde(flatten)]
    rest: HashMap<String, serde_json::Value>,
}
