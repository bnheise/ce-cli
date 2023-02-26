use crate::cli::FrameworkOption;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::{ConfigFile, ConfigFormat, FrameworkConfigurable};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EslintRc<'a> {
    pub extends: Vec<&'a str>,
    pub plugins: Vec<&'a str>,
    pub parser: &'a str,
    pub parser_options: HashMap<&'a str, serde_yaml::Value>,
    #[serde(flatten)]
    extra: HashMap<&'a str, serde_yaml::Value>,
}

impl<'a> EslintRc<'a> {
    const REACT_EXTENDS: [&'static str; 1] = ["plugin:react/recommended"];
    const REACT_PLUGINS: [&'static str; 1] = ["react"];
    const VUE_EXTENDS: [&'static str; 3] = [
        "plugin:vue/vue3-strongly-recommended",
        "@vue/eslint-config-typescript",
        "@vue/eslint-config-prettier/skip-formatting",
    ];
    const TYPESCRIPT_PARSER: &'static str = "@typescript-eslint/parser";
    const VUE_ESLINT_PARSER: &'static str = "vue-eslint-parser";
    const VUE_PLUGINS: [&'static str; 1] = ["vue"];
}

impl<'a> ConfigFile<'a> for EslintRc<'a> {
    const FILENAME: &'static str = ".eslintrc.yml";
    const FORMAT: super::ConfigFormat = ConfigFormat::Yaml;
}

impl<'a> FrameworkConfigurable for EslintRc<'a> {
    fn set_framework_settings(&mut self, framework: FrameworkOption) {
        match framework {
            FrameworkOption::React => {
                for plugin in Self::REACT_PLUGINS {
                    self.plugins.push(plugin)
                }

                for extend in Self::REACT_EXTENDS {
                    self.extends.push(extend)
                }

                self.parser = Self::TYPESCRIPT_PARSER;
            }
            FrameworkOption::Angular => todo!(),
            FrameworkOption::Vue => {
                for extend in Self::VUE_EXTENDS {
                    self.extends.push(extend)
                }

                for plugin in Self::VUE_PLUGINS {
                    self.plugins.push(plugin)
                }

                self.parser = Self::VUE_ESLINT_PARSER;

                self.parser_options.insert(
                    "parser",
                    serde_yaml::to_value(Self::TYPESCRIPT_PARSER)
                        .expect("Could not convert typescript parser to yaml value"),
                );
            }
        }
    }
}
