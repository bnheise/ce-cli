use self::{
    custom_element::handle_custom_element, shared_component::handle_shared_component,
    shared_dependency::handle_shared_dependency,
};
use super::dev_deploy::handle_dev_deploy;
use crate::{cli::AddOption, error::CliError};
use lazy_static::lazy_static;
use regex::Regex;

mod custom_element;
mod shared_component;
mod shared_dependency;

pub fn handle_add(extension_type: AddOption) -> Result<(), CliError> {
    match extension_type {
        AddOption::CustomElement(args) => handle_custom_element(args)?,
        AddOption::SharedComponent { name } => handle_shared_component(name)?,
        AddOption::SharedDependency { package } => handle_shared_dependency(package)?,
    }
    handle_dev_deploy()?;
    Ok(())
}

fn is_extension_name_valid(name: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r#"^[a-zA-Z][a-zA-Z0-9 -]*$"#)
            .expect("Failed to parse extension name validation");
    }
    RE.is_match(name)
}
