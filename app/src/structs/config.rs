use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf};

use crate::cli::FrameworkOption;

use super::{ConfigFile, ConfigFormat};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub project_name: String,
    pub deploy_path: PathBuf,
    pub entrypoints: HashMap<String, PathBuf>,
    pub dev_server_port: u16,
    pub framework: FrameworkOption,
}

impl<'a> ConfigFile<'a> for Config {
    const FILENAME: &'static str = "workspace-config.json";
    const FORMAT: super::ConfigFormat = ConfigFormat::Json;
}

#[derive(Debug, Default)]
pub struct ConfigBuilder {
    project_name: Option<String>,
    deploy_path: Option<PathBuf>,
    framework: Option<FrameworkOption>,
}

impl ConfigBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_project_name(&mut self, project_name: String) {
        self.project_name = Some(project_name);
    }

    pub fn set_deploy_path(&mut self, deploy_path: PathBuf) {
        self.deploy_path = Some(deploy_path);
    }

    pub fn set_framework(&mut self, framework: FrameworkOption) {
        self.framework = Some(framework);
    }

    pub fn build(self) -> Config {
        Config {
            project_name: self
                .project_name
                .expect("Expected project_name to be set but it was None"),
            deploy_path: self
                .deploy_path
                .expect("Expected to get a bundle path but got None"),
            entrypoints: HashMap::new(),
            dev_server_port: 3000,
            framework: self.framework.unwrap_or(FrameworkOption::React),
        }
    }
}
