use crate::cli::InitArgs;
use crate::config_generators::config::Config;
use crate::config_generators::eslintrc::EslintRc;
use crate::config_generators::package_json::PackageJson;
use crate::config_generators::typescript_config_prod_json::TSConfigProdJson;
use crate::config_generators::ClientExt;
use crate::config_generators::ConfigFile;
use crate::config_generators::FrameworkConfigurable;
use crate::config_generators::TemplateContext;
use crate::error::CliError;
use include_dir::include_dir;
use include_dir::Dir;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

pub struct AssetsDir;

impl AssetsDir {
    const ASSETS: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/assets");
    const BASE: &'static str = "base";

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

            let name = name.replace(
                &TemplateContext::format_key(&TemplateContext::NAME_CAMELCASE),
                &definition.get_camelcase_name(),
            );

            for (key, val) in context.iter() {
                let replacer = TemplateContext::format_key(key);
                content = content.replace(&replacer, val);
            }

            fs::write(ext_path.join(&name), content)?;
        }

        Ok(())
    }

    pub fn generate_static_files() -> Result<(), CliError> {
        Self::generate_static_from_folder("common")
    }

    pub fn generate_static_from_folder<S>(folder_name: S) -> Result<(), CliError>
    where
        S: AsRef<Path>,
    {
        let skip_count = folder_name.as_ref().components().count();

        let dir = Self::ASSETS
            .get_dir(folder_name)
            .expect("Failed to find directory");
        Self::generate_static_recurse(dir, skip_count)?;
        Ok(())
    }

    fn generate_static_recurse(dir: &Dir, skip_count: usize) -> Result<(), CliError> {
        let path = dir
            .path()
            .components()
            .skip(skip_count)
            .collect::<PathBuf>();

        fs::create_dir_all(&path)?;
        for entry in dir.entries() {
            match entry {
                include_dir::DirEntry::Dir(dir) => {
                    Self::generate_static_recurse(dir, skip_count)?;
                }
                include_dir::DirEntry::File(file) => {
                    let path = file
                        .path()
                        .components()
                        .skip(skip_count)
                        .collect::<PathBuf>();
                    fs::write(&path, file.contents())?;
                }
            }
        }
        Ok(())
    }

    pub fn generate_framework_files(config: &Config) -> Result<(), CliError> {
        let base_dir = Self::ASSETS
            .get_dir(Self::BASE)
            .expect("Base directory was not found");

        Self::handle_config::<EslintRc>(base_dir, config)?;
        Self::handle_config::<PackageJson>(base_dir, config)?;
        Self::handle_config::<TSConfigProdJson>(base_dir, config)?;

        Self::generate_static_from_folder(Path::new(config.framework.into()))?;

        Ok(())
    }

    fn handle_config<'a, T: ConfigFile<'a> + FrameworkConfigurable>(
        base_dir: &'a Dir,
        config: &'a Config,
    ) -> Result<(), CliError> {
        let raw = base_dir
            .get_file(PathBuf::from(Self::BASE).join(T::FILENAME))
            .unwrap_or_else(|| panic!("Didn't find ${}", T::FILENAME))
            .contents_utf8()
            .unwrap_or_else(|| panic!("Couldn't read {} as utf-8", T::FILENAME));

        let mut parsed = T::try_parse(raw)?;
        parsed.set_framework_settings(config.framework);
        parsed.add_project_settings(config)?;
        let raw = T::try_serialize(parsed)?;
        T::write(raw)?;
        Ok(())
    }

    pub fn set_env_file(args: &InitArgs) -> Result<(), CliError> {
        let base_dir = Self::ASSETS
            .get_dir(Self::BASE)
            .expect("Base directory was not found");
        let env_file = base_dir
            .get_file(".env")
            .expect("Didn't find the .env file");
        let mut contents = env_file.contents_utf8().unwrap().to_owned();

        if let Some(password) = &args.password {
            contents = contents.replace("test", password);
        }
        if let Some(username) = &args.username {
            contents = contents.replace("test@liferay.com", username);
        }

        fs::write(Path::new("./"), contents)?;
        Ok(())
    }
}
