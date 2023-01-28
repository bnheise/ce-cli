use std::{
    env, fs,
    io::{self, Result},
    path::PathBuf,
};

#[derive(Debug)]
pub struct Config {
    project_name: String,
    bundle_path: PathBuf,
}

impl Config {
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> ConfigBuilder {
        ConfigBuilder::new()
    }
}

#[derive(Debug, Default)]
pub struct ConfigBuilder {
    project_name: Option<String>,
    bundle_path: Option<PathBuf>,
}

impl ConfigBuilder {
    fn new() -> Self {
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
        }
    }
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

    println!(
        "{} {}",
        user_input.is_empty(),
        env_bundle_path.components().count() > 0
    );
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
