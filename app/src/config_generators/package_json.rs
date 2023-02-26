use super::{ConfigFile, ConfigFormat, FrameworkConfigurable};
use crate::cli::FrameworkOption;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PackageJson<'a> {
    pub name: &'a str,
    dev_dependencies: HashMap<&'a str, &'a str>,
    pub dependencies: HashMap<&'a str, &'a str>,
    pub scripts: HashMap<&'a str, String>,
    #[serde(flatten)]
    extra: HashMap<&'a str, serde_json::Value>,
}

impl<'a> PackageJson<'a> {
    const REACT_DEV_DEPENDENCIES: [(&str, &str); 5] = [
        ("react", "^16.12.0"),
        ("@types/react", "^16.12.0"),
        ("@types/react-dom", "^16.9.9"),
        ("eslint-plugin-react", "^7.32.1"),
        ("react-dom", "^16.12.0"),
    ];

    const VUE_DEV_DEPENDENCIES: [(&str, &str); 9] = [
        ("vue-loader", "^17.0.1"),
        ("vue-template-compiler", "^2.7.14"),
        ("vue", "^3.2.47"),
        ("@vue/eslint-config-prettier", "^7.1.0"),
        ("@vue/eslint-config-typescript", "^11.0.2"),
        ("@vue/tsconfig", "^0.1.3"),
        ("@vue/test-utils", "^2.3.0"),
        ("eslint-plugin-vue", "^9.9.0"),
        ("vue-tsc", "^1.1.5"),
    ];

    const SHARED_DEP_BUILD_CMD: &'static str = "webpack --config ./webpack.sharedDeps.js";

    pub fn add_shared_dep_build(&mut self) {
        self.scripts.entry("build").and_modify(|build| {
            let _build = build.clone();
            build.as_mut();
            build.push_str(Self::SHARED_DEP_BUILD_CMD);
            build.push_str(" && ");
            build.push_str(&_build);
        });
    }
}

impl<'a> FrameworkConfigurable for PackageJson<'a> {
    fn set_framework_settings(&mut self, framework: FrameworkOption) {
        match framework {
            FrameworkOption::React => {
                for (package, version) in Self::REACT_DEV_DEPENDENCIES {
                    self.dev_dependencies.insert(package, version);
                }
            }
            FrameworkOption::Angular => todo!(),
            FrameworkOption::Vue => {
                for (package, version) in Self::VUE_DEV_DEPENDENCIES {
                    self.dev_dependencies.insert(package, version);
                }
            }
        }
    }
}

impl<'a> ConfigFile<'a> for PackageJson<'a> {
    const FILENAME: &'static str = "package.json";
    const FORMAT: ConfigFormat = ConfigFormat::Json;
}
