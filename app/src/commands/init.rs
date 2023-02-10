use crate::cli::FrameworkOption;
use crate::error::CliError;
use crate::structs::config::{Config, ConfigBuilder};
use crate::structs::eslintrc::EslintRc;
use crate::structs::package_json::PackageJson;
use crate::structs::ConfigFile;
use crate::version_check::run_version_check;
use crate::ASSETS;
use dialoguer::{Input, Select};
use include_dir::Dir;
use regex::Regex;
use std::path::Path;
use std::vec;
use std::{env, fs, path::PathBuf};

pub fn handle_init(
    config: ConfigBuilder,
    project_name: Option<String>,
    bundle_path: Option<PathBuf>,
    config_path: Option<PathBuf>,
    framework: Option<FrameworkOption>,
) -> Result<(), CliError> {
    if !current_dir_empty()? {
        return Err(CliError::InitError("Current directory is not empty".into()));
    }

    let config = initialize_config(config, project_name, bundle_path, config_path, framework)?;

    generate_framework_files(&config)?;
    generate_static_files(
        ASSETS
            .get_dir("common")
            .expect("Failed to find common directory"),
    )?;

    let raw = Config::try_serialize(config)?;
    Config::write(raw)?;

    run_version_check()?;

    Ok(())
}

fn current_dir_empty() -> Result<bool, CliError> {
    Ok(env::current_dir()
        .map_err(|e| CliError::CurrentDirectoryError(Some(e)))?
        .read_dir()
        .map_err(|e| CliError::CurrentDirectoryError(Some(e)))?
        .next()
        .is_none())
}

fn initialize_config(
    mut config: ConfigBuilder,
    project_name: Option<String>,
    deploy_path: Option<PathBuf>,
    config_path: Option<PathBuf>,
    framework: Option<FrameworkOption>,
) -> Result<Config, CliError> {
    if let Some(_config_path) = config_path {
        return Err(CliError::NotImplemented(
            crate::error::UpcomingFeature::SetConfigPathOnInit,
        ));
    }

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

    let config = config.build();
    Ok(config)
}

fn get_framework_from_user() -> Result<FrameworkOption, CliError> {
    let options = vec![
        FrameworkOption::React,
        // FrameworkOption::Angular,
        // FrameworkOption::Vue,
    ];
    let user_response = Select::new()
        .with_prompt("Please enter the framework that you wish to use")
        .items(&options)
        .default(0)
        .interact()
        .map_err(CliError::InputError)?;

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
        .map_err(CliError::InputError)?;

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
        .map_err(|e| CliError::CurrentDirectoryError(Some(e)))?
        .components()
        .last()
        .ok_or(CliError::CurrentDirectoryError(None))?
    {
        std::path::Component::Normal(dirname) => dirname,
        _ => return Err(CliError::CurrentDirectoryError(None)),
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
        .map_err(CliError::InputError)?;

    Ok(user_response)
}

pub fn generate_static_files(dir: &Dir) -> Result<(), CliError> {
    let path = dir.path().components().skip(1).collect::<PathBuf>();
    fs::create_dir_all(&path)
        .map_err(|e| CliError::WriteError(path.to_str().unwrap_or_default().to_owned(), e))?;
    for entry in dir.entries() {
        match entry {
            include_dir::DirEntry::Dir(dir) => {
                generate_static_files(dir)?;
            }
            include_dir::DirEntry::File(file) => {
                let path = file.path().components().skip(1).collect::<PathBuf>();
                fs::write(&path, file.contents()).map_err(|e| {
                    CliError::WriteError(path.to_str().unwrap_or_default().to_owned(), e)
                })?;
            }
        }
    }
    Ok(())
}

pub fn generate_framework_files(config: &Config) -> Result<(), CliError> {
    let base: &str = "base";
    let cypres_config_filename = "cypress.config.js";
    let base_dir = ASSETS.get_dir(base).expect("Base directory was not found");

    let eslintrc_raw = base_dir
        .get_file(PathBuf::from(base).join(EslintRc::FILENAME))
        .expect("Didn't find eslintrc")
        .contents_utf8()
        .expect("Couldn't read eslintrc as utf-8")
        .to_owned();

    let package_json_raw = base_dir
        .get_file(PathBuf::from(base).join(PackageJson::FILENAME))
        .expect("Didn't find package.json")
        .contents_utf8()
        .expect("Couldn't read package.json as utf-8")
        .to_owned();

    let mut cypress_config = base_dir
        .get_file(PathBuf::from(base).join(cypres_config_filename))
        .expect("Didn't find cypress.config.js")
        .contents_utf8()
        .expect("Cound't read cypres.conifg.js as utf-8")
        .to_owned();

    match config.framework {
        FrameworkOption::React => {
            let mut eslint = EslintRc::try_parse(&eslintrc_raw)?;
            eslint.set_framework_settings(config.framework);
            let raw = EslintRc::try_serialize(eslint)?;
            EslintRc::write(raw)?;

            let mut package_json = PackageJson::try_parse(&package_json_raw)?;
            package_json.set_framework_settings(config.framework);
            package_json.name = config.project_name.clone();
            let raw = PackageJson::try_serialize(package_json)?;
            PackageJson::write(raw)?;

            cypress_config =
                cypress_config.replace("{{ framework }}", &config.framework.to_string());

            fs::write(Path::new("./").join(cypres_config_filename), cypress_config)
                .map_err(|e| CliError::WriteError(cypres_config_filename.to_owned(), e))?;
        }
        FrameworkOption::Angular => unimplemented!(),
        FrameworkOption::Vue => unimplemented!(),
    }

    Ok(())
}
