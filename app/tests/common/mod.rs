use std::{fmt::Display, path::Path};

use assert_cmd::Command;

pub mod blade {
    use std::path::Path;

    use assert_cmd::Command;

  pub const BLADE: &str = "blade";
  pub const INIT: &str = "init";
  pub const LIFERAY_VERSION: &str = "--liferay-version";
  pub const LATEST: &str = "dxp-7.4-u68";
  pub const GW: &str = "gw";
  pub const BUILD: &str = "build";

  pub fn build<T>(path: T) -> Result<assert_cmd::assert::Assert, Box<dyn std::error::Error>> where T: AsRef<Path> {
    Ok(Command::new(BLADE)
        .current_dir(path)
        .arg(GW)
        .arg(BUILD)
        .assert()
        .success())
  }
}

pub mod folders {
  pub const CLIENT_EXTENSIONS_DIRNAME: &str = "client-extensions";
  pub const TEST_WORKSPACE_DIRNAME: &str = "test-workspace";
  pub const PROJECT_ROOT: &str = "dummy-project";
}

pub mod ce_cli {
  pub const INIT: &str = "init";
  pub const PROJECT_NAME: &str = "--project-name";
  pub const FRAMEWORK: &str = "--framework";
  pub const INSTANCE_ID: &str = "--instance-id";
  pub const DEPLOY_PATH: &str = "--deploy-path";
  pub const DEFAULT: &str = "default";
}

pub mod util {
    use std::{fs, path::PathBuf};
    use assert_cmd::Command;
    use assert_fs::TempDir;
    use super::{folders, blade};

  pub fn setup() -> Result<(TempDir, PathBuf), Box<dyn std::error::Error>> {
    let workspace_dir = assert_fs::fixture::TempDir::new()?;
    let project_root = workspace_dir.path().join(folders::PROJECT_ROOT);
    fs::create_dir_all(&project_root)?;

    Command::new(blade::BLADE).current_dir(&project_root).arg(blade::INIT).arg(blade::LIFERAY_VERSION).arg(blade::LATEST).assert().success();
    let ce_cli_workspace_path = project_root.join(folders::CLIENT_EXTENSIONS_DIRNAME).join(folders::TEST_WORKSPACE_DIRNAME);
    fs::create_dir_all(&ce_cli_workspace_path)?;
    Ok((workspace_dir, ce_cli_workspace_path))
  }
}

pub enum PackageManager {
  Npm,
  Yarn,
  Pnpm
}

impl PackageManager {
  pub fn install<T>(&self, directory: T) -> Result<assert_cmd::assert::Assert, Box<dyn std::error::Error>> where T: AsRef<Path> {
    match self {
        PackageManager::Npm => Ok(Command::new(self.to_string())
        .current_dir(directory)
        .arg("install")
        .assert()
        .success()),
        PackageManager::Yarn => Ok(Command::new(self.to_string())
        .current_dir(directory)
        .arg("install")
        .assert()
        .success()),
        PackageManager::Pnpm => Ok(Command::new(self.to_string())
        .current_dir(directory)
        .arg("install")
        .assert()
        .success()),
    }
  }
}

impl Display for PackageManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Npm => write!(f, "npm"),
            Self::Yarn => write!(f, "yarn"),
            Self::Pnpm => write!(f, "pnpm"),
        }
    }
}

pub enum Framework {
  React,
  Vue,
}

impl Display for Framework {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
          Self::React => write!(f, "react"),
          Self::Vue => write!(f, "vue"),
        }
    }
}