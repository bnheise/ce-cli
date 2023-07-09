use std::path::Path;

use regex::Regex;

use crate::{
    assets_dir::AssetsDir,
    config_generators::{
        client_extension_yaml::ClientExtensionYaml, config::Config, package_json::PackageJson,
        ConfigFile, External,
    },
    error::CliError,
};

pub fn handle_shared_dependency(package: String) -> Result<(), CliError> {
    let raw = Config::try_open()?;
    let mut config = Config::try_parse(&raw)?;

    let package_info = PackageInfo::try_from(package.as_str())?;
    println!("{:?}", package_info);
    config
        .shared_dependencies
        .insert(package_info.name.to_owned(), package_info.name.to_owned());

    package_info.add_to_externals(&mut config);

    let raw = Config::try_serialize(config)?;
    Config::write(raw)?;

    let raw = PackageJson::try_open()?;
    let mut package_json = PackageJson::try_parse(&raw)?;
    package_json
        .dependencies
        .entry(&package_info.name)
        .or_insert_with(|| &package_info.version);
    package_json.add_shared_dep_build();
    let raw = PackageJson::try_serialize(package_json)?;
    PackageJson::write(raw)?;

    let raw = ClientExtensionYaml::try_open()?;
    let mut client_ext_yaml = ClientExtensionYaml::try_parse(&raw)?;

    client_ext_yaml.add_shared_dep_assemble_if_not_exists();

    let raw = ClientExtensionYaml::try_serialize(client_ext_yaml)?;
    ClientExtensionYaml::write(raw)?;

    AssetsDir::generate_static_from_folder(Path::new("app_templates/shared_dep"))?;

    Ok(())
}

#[derive(Debug)]
struct PackageInfo {
    pub name: String,
    pub version: String,
}

impl External for PackageInfo {
    fn get_filename(&self) -> String {
        self.name.to_owned()
    }
}

impl<'a> TryFrom<&'a str> for PackageInfo {
    type Error = CliError;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let matcher = Regex::new(r"^(?P<name>@?[\w\-_/]+){1}(@?)(?P<version>[\d\.]+)?$")
            .expect("Failed to parse regular expression");
        let caps = matcher
            .captures(value)
            .ok_or(CliError::ParsePackageName(value.to_owned()))?;

        let name = caps
            .name("name")
            .ok_or(CliError::ParsePackageName(value.to_owned()))?
            .as_str()
            .to_string();
        println!("name {:?}", name);
        let version = match caps.name("version") {
            Some(cap) => cap.as_str().to_string(),
            None => "*".to_owned(),
        };
        println!("version {:?}", version);
        Ok(Self { name, version })
    }
}
//
