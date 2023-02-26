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
    pub externals: HashMap<String, String>,
    pub alias: HashMap<String, Vec<PathBuf>>,
    pub default_instance_id: String,
    pub shared_dependencies: HashMap<String, String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            project_name: Default::default(),
            deploy_path: Default::default(),
            entrypoints: Default::default(),
            dev_server_port: 3000,
            framework: Default::default(),
            externals: Default::default(),
            alias: Default::default(),
            default_instance_id: String::from("default"),
            shared_dependencies: Default::default(),
        }
    }
}

impl<'a> ConfigFile<'a> for Config {
    const FILENAME: &'static str = "workspace-config.json";
    const FORMAT: super::ConfigFormat = ConfigFormat::Json;

    fn add_project_settings<'b: 'a>(
        &mut self,
        _config: &'b Config,
    ) -> Result<(), crate::error::CliError> {
        Ok(())
    }
}

#[derive(Debug, Default)]
pub struct ConfigBuilder {
    project_name: Option<String>,
    deploy_path: Option<PathBuf>,
    framework: Option<FrameworkOption>,
    instance_id: Option<String>,
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

    pub fn set_instance_id(&mut self, instance_id: String) {
        self.instance_id = Some(instance_id)
    }

    pub fn build(self) -> Config {
        Config {
            project_name: self
                .project_name
                .expect("Expected project_name to be set but it was None"),
            deploy_path: self
                .deploy_path
                .expect("Expected to get a bundle path but got None"),
            framework: self.framework.unwrap_or(FrameworkOption::React),
            default_instance_id: self.instance_id.unwrap_or_default(),
            ..Default::default()
        }
    }
}
