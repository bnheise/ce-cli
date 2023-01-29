use serde::Serialize;
use std::{collections::HashMap, path::PathBuf};

#[derive(Debug, Serialize)]
pub struct Config {
    pub project_name: String,
    pub bundle_path: PathBuf,
    pub entrypoints: HashMap<String, PathBuf>,
}

#[derive(Debug, Default)]
pub struct ConfigBuilder {
    project_name: Option<String>,
    bundle_path: Option<PathBuf>,
    entrypoints: HashMap<String, PathBuf>,
}

impl ConfigBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_project_name(&mut self, project_name: String) {
        self.project_name = Some(project_name);
    }

    pub fn set_bundle_path(&mut self, bundle_path: PathBuf) {
        self.bundle_path = Some(bundle_path);
    }

    pub fn build(self) -> Config {
        Config {
            project_name: self
                .project_name
                .expect("Expected project_name to be set but it was None"),
            bundle_path: self
                .bundle_path
                .expect("Expected to get a bundle path but got None"),
            entrypoints: HashMap::new(),
        }
    }
}
