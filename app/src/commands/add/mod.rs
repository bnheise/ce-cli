use self::custom_element::handle_custom_element;
use super::dev_deploy::handle_dev_deploy;
use crate::{cli::ClientExtType, error::CliError};

pub mod custom_element;

pub fn handle_add(extension_type: ClientExtType) -> Result<(), CliError> {
    match extension_type {
        ClientExtType::CustomElement {
            name,
            html_element_name,
            friendly_url_mapping,
            instanceable,
            portlet_category_name,
            description,
            use_esm,
            source_code_url,
        } => handle_custom_element(
            name,
            html_element_name,
            friendly_url_mapping,
            instanceable,
            portlet_category_name,
            description,
            use_esm,
            source_code_url,
        )?,
    }
    handle_dev_deploy()?;
    Ok(())
}
