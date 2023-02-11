use crate::cli::FrameworkOption;
use crate::error::CliError;
use crate::structs::config::Config;
use crate::structs::eslintrc::EslintRc;
use crate::structs::package_json::PackageJson;
use crate::structs::ClientExt;
use crate::structs::ConfigFile;
use crate::structs::TemplateContext;
use include_dir::include_dir;
use include_dir::Dir;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

pub struct AssetsDir;

impl AssetsDir {
    const ASSETS: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/assets");

    pub fn initialize_templates<T: ClientExt>(
        config: &Config,
        definition: &T,
    ) -> Result<(), CliError> {
        let templates = Self::ASSETS
            .get_dir(
                PathBuf::new()
                    .join("app_templates")
                    .join(config.framework.to_string())
                    .join(definition.get_type_name()),
            )
            .expect("Failed to load the custom_element templates folder");

        let context = definition.get_context();
        let ext_path = definition.get_ext_path();

        for file in templates.files() {
            let mut content = file
                .contents_utf8()
                .expect("Could not parse template file as utf-8")
                .to_owned();

            let name = match file.path().components().last().unwrap() {
                std::path::Component::Normal(filename) => filename.to_str().unwrap_or_default(),
                _ => unreachable!(),
            };

            for (key, val) in context.iter() {
                let replacer = TemplateContext::format_key(key);
                content = content.replace(&replacer, val);
            }

            fs::write(ext_path.join(name), content)
                .map_err(|e| CliError::WriteError(name.to_owned(), e))?;
        }

        Ok(())
    }

    pub fn generate_static_files() -> Result<(), CliError> {
        let dir = Self::ASSETS
            .get_dir("common")
            .expect("Failed to find common directory");
        Self::generate_static_recurse(dir)?;
        Ok(())
    }

    fn generate_static_recurse(dir: &Dir) -> Result<(), CliError> {
        let path = dir.path().components().skip(1).collect::<PathBuf>();
        fs::create_dir_all(&path)
            .map_err(|e| CliError::WriteError(path.to_str().unwrap_or_default().to_owned(), e))?;
        for entry in dir.entries() {
            match entry {
                include_dir::DirEntry::Dir(dir) => {
                    Self::generate_static_recurse(dir)?;
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
        let base_dir = Self::ASSETS
            .get_dir(base)
            .expect("Base directory was not found");

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

                cypress_config = cypress_config.replace(
                    &TemplateContext::format_key(TemplateContext::FRAMEWORK),
                    &config.framework.to_string(),
                );

                fs::write(Path::new("./").join(cypres_config_filename), cypress_config)
                    .map_err(|e| CliError::WriteError(cypres_config_filename.to_owned(), e))?;
            }
            FrameworkOption::Angular => unimplemented!(),
            FrameworkOption::Vue => unimplemented!(),
        }

        Ok(())
    }
}
