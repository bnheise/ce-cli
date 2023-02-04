use crate::{
    config::{Config, ConfigBuilder},
    error::CliError,
    templates::{
        configs::{
            CLIENT_EXT_YAML, CLIENT_EXT_YAML_FILENAME, DOCKERFILE, DOCKERFILE_FILENAME, ESLINTRC,
            ESLINTRC_FILENAME, GITIGNORE, GITIGNORE_FILENAME, JEST_CONFIG,
            JEST_CONFIG_JSON_FILENAME, LCP_JSON, LCP_JSON_FILENAME, PACKAGEJSON,
            PACKAGEJSON_FILENAME, PRETTIERRC, PRETTIERRCE_FILENAME, TSCONFIG_BASE,
            TSCONFIG_BASE_FILENAME, TSCONFIG_DEV, TSCONFIG_DEV_FILENAME, TSCONFIG_PROD,
            TSCONFIG_PROD_FILENAME, WEBPACK_CONFIG_COMMON, WEBPACK_CONFIG_COMMON_FILENAME,
            WEBPACK_CONFIG_DEV, WEBPACK_CONFIG_DEV_FILENAME, WEBPACK_CONFIG_PROD,
            WEBPACK_CONFIG_PROD_FILENAME, WORKSPACE_CONFIG_FILENAME,
        },
        scripts::{LIFERAY_EXTERNALS, LIFERAY_EXTERNALS_FILENAME},
        CLIENT_EXTENSIONS, OSGI,
    },
};
use dialoguer::Input;
use regex::Regex;
use serde_json::Value;
use std::io::Write;
use std::{env, fs, path::PathBuf};

pub fn handle_init(
    config: ConfigBuilder,
    project_name: Option<String>,
    bundle_path: Option<PathBuf>,
    config_path: Option<PathBuf>,
) -> Result<(), CliError> {
    let config = initialize_config(config, project_name, bundle_path, config_path)?;
    fs::create_dir("./util").map_err(|e| CliError::WriteError(("./util".to_owned(), e)))?;
    fs::create_dir("./src").map_err(|e| CliError::WriteError(("./src".to_owned(), e)))?;

    generate_files(&config)?;

    Ok(())
}

fn initialize_config(
    mut config: ConfigBuilder,
    project_name: Option<String>,
    deploy_path: Option<PathBuf>,
    config_path: Option<PathBuf>,
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

    let config = config.build();
    Ok(config)
}

fn get_deploy_path_from_user(bundle_path: Option<PathBuf>) -> Result<PathBuf, CliError> {
    let default_deploy_path = if let Some(bundle_path) = bundle_path {
        bundle_path.join(OSGI).join(CLIENT_EXTENSIONS)
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

pub fn generate_files(config: &Config) -> Result<(), CliError> {
    let static_files = [
        (ESLINTRC_FILENAME, ESLINTRC),
        (GITIGNORE_FILENAME, GITIGNORE),
        (PRETTIERRCE_FILENAME, PRETTIERRC),
        (TSCONFIG_PROD_FILENAME, TSCONFIG_PROD),
        (TSCONFIG_BASE_FILENAME, TSCONFIG_BASE),
        (TSCONFIG_DEV_FILENAME, TSCONFIG_DEV),
        (WEBPACK_CONFIG_COMMON_FILENAME, WEBPACK_CONFIG_COMMON),
        (WEBPACK_CONFIG_PROD_FILENAME, WEBPACK_CONFIG_PROD),
        (WEBPACK_CONFIG_DEV_FILENAME, WEBPACK_CONFIG_DEV),
        (DOCKERFILE_FILENAME, DOCKERFILE),
        (CLIENT_EXT_YAML_FILENAME, CLIENT_EXT_YAML),
        (LIFERAY_EXTERNALS_FILENAME, LIFERAY_EXTERNALS),
        (JEST_CONFIG_JSON_FILENAME, JEST_CONFIG),
    ];

    for (filname, content) in static_files.iter() {
        generate_file(filname, content)?;
    }

    let dynamic_files = [
        (PACKAGEJSON_FILENAME, prepare_package_json(config)),
        (LCP_JSON_FILENAME, prepare_lcp_json(config)?),
        (WORKSPACE_CONFIG_FILENAME, prepare_config_file(config)?),
    ];

    for (filname, content) in dynamic_files.iter() {
        generate_file(filname, content)?;
    }

    Ok(())
}

fn generate_file(filname: &'static str, content: &str) -> Result<(), CliError> {
    let mut output =
        fs::File::create(filname).map_err(|e| CliError::WriteError((filname.to_owned(), e)))?;
    output
        .write_all(content.as_bytes())
        .map_err(|e| CliError::WriteError((filname.to_owned(), e)))?;

    Ok(())
}

fn prepare_package_json(config: &Config) -> String {
    let mut v = match serde_json::from_str(PACKAGEJSON)
        .expect("The package.json template was not parseable")
    {
        Value::Object(package) => package,
        _ => panic!("Should have gotten an Object from Package.json"),
    };

    v.entry("name").and_modify(|value| {
        match value {
            Value::String(val) => *val = config.project_name.clone(),
            _ => panic!("Should have gotten a string from 'name' field of package.json"),
        };
    });

    serde_json::to_string_pretty(&v)
        .expect("Could not convert the package.json template back to a string")
}

fn prepare_lcp_json(config: &Config) -> Result<String, CliError> {
    let mut v = match serde_json::from_str(LCP_JSON)
        .map_err(|e| CliError::ParseJsonError(LCP_JSON_FILENAME, e))?
    {
        Value::Object(package) => package,
        _ => panic!("Should have gotten an Object from LCP.json"),
    };

    v.entry("id").and_modify(|value| {
        match value {
            Value::String(val) => *val = config.project_name.clone().replace('-', ""),
            _ => panic!("Should have gotten a string from 'id' field of package.json"),
        };
    });

    Ok(serde_json::to_string_pretty(&v).unwrap())
}

fn prepare_config_file(config: &Config) -> Result<String, CliError> {
    Ok(serde_json::to_string_pretty(&config).unwrap())
}
