use assert_cmd::Command;
use std::{
    env,
};

mod common;
use common::util;
use common::ce_cli;
use common::folders;
use common::blade;
use common::Framework;
use common::PackageManager;


#[test]
fn test_react_npm() -> Result<(), Box<dyn std::error::Error>> {
    test_flow(PackageManager::Npm, Framework::React)?;
    Ok(())
}

#[test]
fn test_react_yarn() -> Result<(), Box<dyn std::error::Error>> {
    test_flow(PackageManager::Yarn, Framework::React)?;
    Ok(())
}

#[test]
fn test_react_pnpm() -> Result<(), Box<dyn std::error::Error>> {
    test_flow(PackageManager::Pnpm, Framework::React)?;
    Ok(())
}

#[test]
fn test_vue_npm() -> Result<(), Box<dyn std::error::Error>> {
    test_flow(PackageManager::Npm, Framework::Vue)?;
    Ok(())
}

#[test]
fn test_vue_yarn() -> Result<(), Box<dyn std::error::Error>> {
    test_flow(PackageManager::Yarn, Framework::Vue)?;
    Ok(())
}

#[test]
fn test_vue_pnpm() -> Result<(), Box<dyn std::error::Error>> {
    test_flow(PackageManager::Pnpm, Framework::Vue)?;
    Ok(())
}

fn test_flow(package_manager: PackageManager, framework: Framework) -> Result<(), Box<dyn std::error::Error>> {
    let (_temp_dir, ce_cli_workspace_path) = util::setup()?;
    
    Command::cargo_bin(env!("CARGO_PKG_NAME"))?
        .current_dir(&ce_cli_workspace_path)
        .arg(ce_cli::INIT)
        .arg(ce_cli::PROJECT_NAME)
        .arg(folders::TEST_WORKSPACE_DIRNAME)
        .arg(ce_cli::FRAMEWORK)
        .arg(framework.to_string())
        .arg(ce_cli::INSTANCE_ID)
        .arg(ce_cli::DEFAULT)
        .arg(ce_cli::DEPLOY_PATH)
        .arg(&ce_cli_workspace_path)
        .assert()
        .success();

    package_manager.install(&ce_cli_workspace_path)?;

    blade::build(&ce_cli_workspace_path)?;

    Ok(())
}