use std::io::Result;

use self::custom_element::handle_custom_element;
use crate::cli::ClientExtType;

use super::dev_deploy::handle_dev_deploy;

pub mod custom_element;

pub fn handle_add(extension_type: ClientExtType) -> Result<()> {
    match extension_type {
        ClientExtType::CustomElement {
            name,
            html_element_name,
            friendly_url_mapping,
            instanceable,
            portlet_category_name,
            description,
            use_esm,
        } => handle_custom_element(
            name,
            html_element_name,
            friendly_url_mapping,
            instanceable,
            portlet_category_name,
            description,
            use_esm,
        )?,
    }
    handle_dev_deploy()?;
    Ok(())
}
