use std::fs::File;
use std::io::{Read, Seek, Write};
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
            zip.start_file_from_path(name, options)?;
            let mut f = File::open(path)?;

            f.read_to_end(&mut buffer)?;
            zip.write_all(&buffer)?;
            buffer.clear();
        } else if !name.as_os_str().is_empty() {
            #[allow(deprecated)]
            zip.add_directory_from_path(name, options)?;
        }
    }
    zip.finish()?;
    Result::Ok(())
}

pub fn zip_directory(src_dir: PathBuf, dst_file: PathBuf) -> Result<(), CliError> {
    if !src_dir.is_dir() {
        return Err(CliError::FileSystemError(
            "Specified path should be a directory".into(),
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
