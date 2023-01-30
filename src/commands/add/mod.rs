use std::io::Result;

use self::custom_element::handle_custom_element;
use crate::cli::ClientExtType;

pub mod custom_element;

pub fn handle_add(extension_type: ClientExtType) -> Result<()> {
    match extension_type {
        ClientExtType::RemoteApp {
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

    Ok(())
}
