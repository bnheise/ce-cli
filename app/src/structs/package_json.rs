use super::{ConfigFile, ConfigFormat};
use crate::cli::FrameworkOption;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PackageJson {
    pub name: String,
    dev_dependencies: HashMap<String, String>,
    #[serde(flatten)]
    extra: HashMap<String, serde_json::Value>,
}

impl PackageJson {
    const REACT_DEV_DEPENDENCIES: [(&str, &str); 5] = [
        ("react", "^16.12.0"),
        ("@types/react", "^16.12.0"),
        ("@types/react-dom", "^16.9.9"),
        ("eslint-plugin-react", "^7.32.1"),
        ("react-dom", "^16.12.0"),
    ];

    pub fn set_framework_settings(&mut self, framework: FrameworkOption) {
        match framework {
            FrameworkOption::React => {
                for (package, version) in Self::REACT_DEV_DEPENDENCIES {
                    self.dev_dependencies
                        .insert(package.to_owned(), version.to_owned());
                }
            }
            FrameworkOption::Angular => todo!(),
            FrameworkOption::Vue => todo!(),
        }
    }
}

impl<'a> ConfigFile<'a> for PackageJson {
    const FILENAME: &'static str = "package.json";
    const FORMAT: super::ConfigFormat = ConfigFormat::Json;
}
