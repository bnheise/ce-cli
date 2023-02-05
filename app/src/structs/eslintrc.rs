use crate::cli::FrameworkOption;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::{ConfigFile, ConfigFormat};

#[derive(Debug, Serialize, Deserialize)]
pub struct EslintRc {
    pub extends: Vec<String>,
    pub plugins: Vec<String>,
    #[serde(flatten)]
    extra: HashMap<String, serde_yaml::Value>,
}

impl EslintRc {
    const REACT_EXTENDS: [&'static str; 1] = ["plugin:react/recommended"];
    const REACT_PLUGINS: [&'static str; 1] = ["react"];
    pub fn set_framework_settings(&mut self, framework: FrameworkOption) {
        match framework {
            FrameworkOption::React => {
                for plugin in Self::REACT_PLUGINS {
                    self.plugins.push(plugin.to_owned())
                }

                for extend in Self::REACT_EXTENDS {
                    self.extends.push(extend.to_owned())
                }
            }
            FrameworkOption::Angular => todo!(),
            FrameworkOption::Vue => todo!(),
        }
    }
}

impl<'a> ConfigFile<'a> for EslintRc {
    const FILENAME: &'static str = ".eslintrc.yml";
    const FORMAT: super::ConfigFormat = ConfigFormat::Yaml;
}
