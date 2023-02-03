use std::fs::File;
use std::io::prelude::*;
use std::io::{Seek, Write};
use std::iter::Iterator;
use std::path::PathBuf;
use walkdir::{DirEntry, WalkDir};
use zip::write::FileOptions;

use crate::error::CliError;

fn zip_dir<T>(
    it: &mut dyn Iterator<Item = DirEntry>,
    prefix: PathBuf,
    writer: T,
    method: zip::CompressionMethod,
) -> Result<(), CliError>
where
    T: Write + Seek,
{
    let mut zip = zip::ZipWriter::new(writer);
    let options = FileOptions::default()
        .compression_method(method)
        .unix_permissions(0o755);

    let mut buffer = Vec::new();
    for entry in it {
        let path = entry.path();
        let name = path.strip_prefix(&prefix).unwrap();

        if path.is_file() {
            #[allow(deprecated)]
            zip.start_file_from_path(name, options)
                .map_err(CliError::ZipError)?;
            let mut f = File::open(path)
                .map_err(|e| CliError::OpenFileError(path.to_string_lossy().to_string(), e))?;

            f.read_to_end(&mut buffer)
                .map_err(|e| CliError::ReadFileError(path.to_string_lossy().to_string(), e))?;
            zip.write_all(&buffer)
                .map_err(|e| CliError::WriteError((path.to_string_lossy().to_string(), e)))?;
            buffer.clear();
        } else if !name.as_os_str().is_empty() {
            #[allow(deprecated)]
            zip.add_directory_from_path(name, options).map_err(|e| {
                CliError::WriteError((name.to_string_lossy().to_string(), e.into()))
            })?;
        }
    }
    zip.finish().map_err(CliError::ZipError)?;
    Result::Ok(())
}

pub fn zip_directory(src_dir: PathBuf, dst_file: PathBuf) -> Result<(), CliError> {
    if !src_dir.is_dir() {
        return Err(CliError::NotADirectoryError(
            src_dir.to_string_lossy().to_string(),
        ));
    }

    let file = File::create(dst_file).expect("Could not create zip file");

    let walkdir = WalkDir::new(&src_dir);
    let it = walkdir.into_iter();

    zip_dir(
        &mut it.filter_map(|e| e.ok()),
        src_dir,
        file,
        zip::CompressionMethod::Deflated,
    )?;

    Ok(())
}
