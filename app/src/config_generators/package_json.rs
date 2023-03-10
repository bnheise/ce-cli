use super::{config::Config, ConfigFile, ConfigFormat, FrameworkConfigurable};
use crate::cli::FrameworkOption;
use serde::{Deserialize, Serialize, Serializer};
use std::collections::{BTreeMap, HashMap};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PackageJson<'a> {
    pub name: &'a str,
    #[serde(rename(serialize = "type"))]
    pub _type: &'a str,
    pub version: &'a str,
    pub license: &'a str,
    pub scripts: HashMap<&'a str, String>,
    #[serde(serialize_with = "ordered_map")]
    dev_dependencies: HashMap<&'a str, &'a str>,
    #[serde(serialize_with = "ordered_map")]
    pub dependencies: HashMap<&'a str, &'a str>,
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

    const VUE_DEV_DEPENDENCIES: [(&str, &str); 10] = [
        ("vue-loader", "^17.0.1"),
        ("vue-template-compiler", "^2.7.14"),
        ("@vue/eslint-config-prettier", "^7.1.0"),
        ("@vue/eslint-config-typescript", "^11.0.2"),
        ("@vue/tsconfig", "^0.1.3"),
        ("@vue/test-utils", "^2.3.0"),
        ("eslint-plugin-vue", "^9.9.0"),
        ("vue-tsc", "^1.1.5"),
        ("@vue/compiler-dom", "^3.2.47"),
        ("@vue/server-renderer", "^3.2.47"),
    ];

    const SHARED_DEP_BUILD_CMD: &'static str = "webpack --config ./webpack.sharedDeps.js";

    pub fn add_shared_dep_build(&mut self) {
        self.scripts.entry("build").and_modify(|build| {
            let _build = build.clone();
            let mut split = _build.split(" && ");
            build.clear();
            build.push_str(split.next().unwrap());
            build.push_str(" && ");
            build.push_str(Self::SHARED_DEP_BUILD_CMD);
            build.push_str(" && ");
            build.push_str(split.next().unwrap());
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

    fn add_project_settings<'b: 'a>(
        &mut self,
        config: &'b Config,
    ) -> Result<(), crate::error::CliError> {
        self.name = &config.project_name;

        Ok(())
    }
}

fn ordered_map<'a, S>(value: &HashMap<&'a str, &'a str>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let ordered: BTreeMap<_, _> = value.iter().collect();
    ordered.serialize(serializer)
}
