use std::{
    env,
    fs::{self, File},
    io::{self, Result, Write},
    path::PathBuf,
};

use serde_json::Value;

use crate::{
    config::{Config, ConfigBuilder},
    templates::{
        CLIENT_EXT_YAML, CLIENT_EXT_YAML_FILENAME, DOCKERFILE, DOCKERFILE_FILENAME, ESLINTRC,
        ESLINTRC_FILENAME, GITIGNORE, GITIGNORE_FILENAME, LCP_JSON, LCP_JSON_FILENAME,
        LIFERAY_EXTERNALS, LIFERAY_EXTERNALS_FILENAME, PACKAGEJSON, PACKAGEJSON_FILENAME,
        PRETTIERRC, PRETTIERRCE_FILENAME, TSCONFIG_BASE, TSCONFIG_BASE_FILENAME, TSCONFIG_DEV,
        TSCONFIG_DEV_FILENAME, TSCONFIG_PROD, TSCONFIG_PROD_FILENAME, WEBPACK_CONFIG_COMMON,
        WEBPACK_CONFIG_COMMON_FILENAME, WEBPACK_CONFIG_DEV, WEBPACK_CONFIG_DEV_FILENAME,
        WEBPACK_CONFIG_PROD, WEBPACK_CONFIG_PROD_FILENAME, WORKSPACE_CONFIG_FILENAME,
    },
};

pub fn handle_init(
    mut config: ConfigBuilder,
    project_name: Option<String>,
    bundle_path: Option<PathBuf>,
    config_path: Option<PathBuf>,
) -> Result<()> {
    if let Some(_config_path) = config_path {
        todo!("Handle config path and ignore other arguments");
    }

    if let Some(project_name) = project_name {
        config.set_project_name(project_name);
    } else {
        let project_name = get_project_name_from_user()?;
        config.set_project_name(project_name);
    }

    if let Some(bundle_path) = bundle_path {
        config.set_bundle_path(bundle_path);
    } else {
        let bundle_path = get_bundle_path_from_environment();
        let bundle_path = get_bundle_path_from_user(bundle_path);
        config.set_bundle_path(bundle_path);
    }

    let config = config.build();

    fs::create_dir("./util")?;
    fs::create_dir("./src")?;

    generate_files(&config)?;

    Ok(())
}

pub fn get_bundle_path_from_user(bundle_path: Option<PathBuf>) -> PathBuf {
    let env_bundle_path = bundle_path.unwrap_or_default();

    println!(
        "Enter the path to your local Liferay installation ({})",
        env_bundle_path.display()
    );

    let stdin = io::stdin();
    let mut user_input = String::new();

    stdin
        .read_line(&mut user_input)
        .expect("Expected to get a project name from the user but failed");

    if user_input.trim().is_empty() && env_bundle_path.components().count() > 0 {
        env_bundle_path
    } else if !user_input.trim().is_empty() {
        user_input
            .trim()
            .try_into()
            .expect("Expected a valid path but received none")
    } else {
        panic!("A project name must be provided")
    }
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

pub fn get_project_name_from_user() -> Result<String> {
    let folder_name = match env::current_dir()?.components().last().unwrap() {
        std::path::Component::Normal(dirname) => dirname,
        _ => panic!("Expected to get the current directory name didn't"),
    }
    .to_str()
    .unwrap_or("")
    .to_owned();

    let stdin = io::stdin();
    let mut user_input = String::new();
    println!("Please enter a project name ({})", folder_name);
    stdin
        .read_line(&mut user_input)
        .expect("Expected to get a project name from the user but failed");

    if user_input.trim().is_empty() && !folder_name.trim().is_empty() {
        Ok(folder_name)
    } else if !user_input.trim().is_empty() {
        Ok(user_input)
    } else {
        panic!("A project name must be provided")
    }
}

pub fn generate_files(config: &Config) -> Result<()> {
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
    ];

    for (filname, content) in static_files.iter() {
        generate_file(filname, content)?;
    }

    let dynamic_files = [
        (PACKAGEJSON_FILENAME, prepare_package_json(config)?),
        (LCP_JSON_FILENAME, prepare_lcp_json(config)?),
        (WORKSPACE_CONFIG_FILENAME, prepare_config_file(config)?),
    ];

    for (filname, content) in dynamic_files.iter() {
        generate_file(filname, content)?;
    }

    Ok(())
}

fn generate_file(filname: &'static str, content: &str) -> Result<()> {
    let mut output = File::create(filname)?;
    output.write_all(content.as_bytes())?;

    Ok(())
}

fn prepare_package_json(config: &Config) -> Result<String> {
    let mut v = match serde_json::from_str(PACKAGEJSON)? {
        Value::Object(package) => package,
        _ => panic!("Should have gotten an Object from Package.json"),
    };

    v.entry("name").and_modify(|value| {
        match value {
            Value::String(val) => *val = config.project_name.clone(),
            _ => panic!("Should have gotten a string from 'name' field of package.json"),
        };
    });

    Ok(serde_json::to_string_pretty(&v).unwrap())
}

fn prepare_lcp_json(config: &Config) -> Result<String> {
    let mut v = match serde_json::from_str(LCP_JSON)? {
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

fn prepare_config_file(config: &Config) -> Result<String> {
    Ok(serde_json::to_string_pretty(&config).unwrap())
}
