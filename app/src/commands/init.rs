use crate::assets_dir::AssetsDir;
use crate::cli::{FrameworkOption, InitArgs};
use crate::config_generators::config::{Config, ConfigBuilder};
use crate::config_generators::ConfigFile;
use crate::error::CliError;
use crate::version_check::run_version_check;
use dialoguer::{Input, Select};
use regex::Regex;
use std::vec;
use std::{env, fs, path::PathBuf};

pub fn handle_init(args: InitArgs) -> Result<(), CliError> {
    let config = ConfigBuilder::new();

    if !current_dir_empty()? {
        return Err(CliError::Init("Current directory is not empty".into()));
    }

    let config = initialize_config(config, args)?;

    AssetsDir::generate_static_files()?;
    AssetsDir::generate_framework_files(&config)?;

    let raw = Config::try_serialize(config)?;
    Config::write(raw)?;

    run_version_check()?;

    Ok(())
}

fn current_dir_empty() -> Result<bool, CliError> {
    Ok(env::current_dir()
        .map_err(|e| CliError::CurrentDirectory(Some(e)))?
        .read_dir()
        .map_err(|e| CliError::CurrentDirectory(Some(e)))?
        .next()
        .is_none())
}

fn initialize_config(mut config: ConfigBuilder, args: InitArgs) -> Result<Config, CliError> {
    let InitArgs {
        project_name,
        deploy_path,
        framework,
        instance_id,
        ..
    } = args;

    if let Some(project_name) = project_name {
        config.set_project_name(project_name);
    } else {
        let project_name = get_project_name_from_user()?;
        config.set_project_name(project_name);
    }

    if let Some(deploy_path) = deploy_path {
        config.set_deploy_path(deploy_path);
    } else {
        let bundle_path = get_bundle_path_from_environment();
        let deploy_path = get_deploy_path_from_user(bundle_path)?;
        config.set_deploy_path(deploy_path);
    }

    if let Some(framework) = framework {
        config.set_framework(framework);
    } else {
        let framework = get_framework_from_user()?;
        config.set_framework(framework);
    }

    if let Some(instance_id) = instance_id {
        config.set_instance_id(instance_id)
    } else {
        let instance_id = get_instance_id_from_user()?;
        config.set_instance_id(instance_id)
    }

    let config = config.build();
    Ok(config)
}

fn get_instance_id_from_user() -> Result<String, CliError> {
    let instance_id = Input::new()
        .with_prompt("Please enter the web id for the virtual instance that you want to deploy to")
        .with_initial_text(Config::default().default_instance_id)
        .interact_text()
        .map_err(CliError::Input)?;

    Ok(instance_id)
}

fn get_framework_from_user() -> Result<FrameworkOption, CliError> {
    let options = vec![
        FrameworkOption::React,
        // FrameworkOption::Angular,
        FrameworkOption::Vue,
    ];
    let user_response = Select::new()
        .with_prompt("Please enter the framework that you wish to use")
        .items(&options)
        .default(0)
        .interact()
        .map_err(CliError::Input)?;

    Ok(options[user_response])
}

fn get_deploy_path_from_user(bundle_path: Option<PathBuf>) -> Result<PathBuf, CliError> {
    let default_deploy_path = if let Some(bundle_path) = bundle_path {
        bundle_path.join("osgi").join("client-extensions")
    } else {
        PathBuf::new()
    };

    let user_response: String = Input::new()
        .with_prompt("Please enter the path to your Liferay deploy folder")
        .with_initial_text(default_deploy_path.to_str().unwrap_or_default())
        .validate_with(|input: &String| -> Result<(), &str> {
            let path = PathBuf::from(input);

            if path.is_dir() {
                Ok(())
            } else if !path.exists() {
                Err("The path you entered does not exist.")
            } else if path.is_file() {
                Err("The path you entered points to a file rather than a directory.")
            } else {
                Err("It was not possible to parse your input.")
            }
        })
        .interact_text()
        .map_err(CliError::Input)?;

    Ok(PathBuf::from(user_response))
}

pub fn get_bundle_path_from_environment() -> Option<PathBuf> {
    let mut current_dir = env::current_dir().unwrap();

    loop {
        let paths = fs::read_dir(&current_dir).unwrap();

        for path in paths {
            let path = path.unwrap().path();
            if path.is_dir() {
                let dirname = match path.components().last().unwrap() {
                    std::path::Component::Normal(dirname) => dirname.to_str().unwrap(),
                    _ => unreachable!(),
                };

                if dirname == "bundles" {
                    return Some(path);
                }
            }
        }

        if !current_dir.pop() {
            break;
        }
    }

    None
}

pub fn get_project_name_from_user() -> Result<String, CliError> {
    let folder_name = match env::current_dir()
        .map_err(|e| CliError::CurrentDirectory(Some(e)))?
        .components()
        .last()
        .ok_or(CliError::CurrentDirectory(None))?
    {
        std::path::Component::Normal(dirname) => dirname,
        _ => return Err(CliError::CurrentDirectory(None)),
    }
    .to_str()
    .unwrap_or_default()
    .to_owned();

    let user_response: String = Input::new()
        .with_prompt("Please enter a project name")
        .with_initial_text(folder_name.replace(' ', "-"))
        .report(true)
        .validate_with(|input: &String| -> Result<(), &str> {
            let is_url_safe = Regex::new(r"^[a-zA-Z0-9_-]*$").unwrap();
            if is_url_safe.is_match(input) {
                Ok(())
            } else {
                Err("Project name must not contain characters that cannot be displayed in a url")
            }
        })
        .interact_text()
        .map_err(CliError::Input)?;

    Ok(user_response)
}
