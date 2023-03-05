use crate::{cli::ObjectOption, error::CliError};

use self::import::handle_import;

pub mod import;

pub fn handle_object(options: ObjectOption) -> Result<(), CliError> {
    match options {
        ObjectOption::Import(args) => handle_import(args)?,
        ObjectOption::Export(args) => todo!(),
    }
    Ok(())
}
