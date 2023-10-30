use crate::{constants, loaders_utils};

use std::{path::Path, fs::File, io::{Seek, Write, prelude::*}};
use walkdir::{DirEntry, WalkDir};
use zip::write::FileOptions;

pub fn create(publish_path: &String, args: &Vec<String>) -> Result<bool, String> {
    if args.contains(&constants::NO_ZIP.to_string()) {
        return Ok(false);
    }
    
    let temp_archive_name = constants::TEMP_ARCHIVE_NAME.to_string();
    let path = Path::new(&temp_archive_name);
    let file = File::create(&path).unwrap();

    let walkdir = WalkDir::new(publish_path);
    let iterator = walkdir.into_iter();

    let result = zip_dir(&mut iterator.filter_map(|e| e.ok()), publish_path, file, zip::CompressionMethod::BZIP2);
    match result {
        Err(error) => return Err(error.to_string()),
        Ok(_) => return Ok(true),
    }
}

fn zip_dir<T>(
        iterator: &mut dyn Iterator<Item = DirEntry>,
        prefix: &str,
        writer: T,
        method: zip::CompressionMethod,
        ) -> zip::result::ZipResult<()>
where T: Write + Seek,
{
    let mut zip = zip::ZipWriter::new(writer);
    let options = FileOptions::default()
    .compression_method(method)
    .unix_permissions(0o755);

    let mut buffer = Vec::new();

    let spinner = loaders_utils::get_spinner("Compressione...");

    for entry in iterator {
        let path = entry.path();
        let name = path.strip_prefix(Path::new(prefix)).unwrap();

        // Write file or directory explicitly
        // Some unzip tools unzip files with directory paths correctly, some do not!
        if path.is_file() {            
            spinner.set_message(format!("Aggiungo file {:?} ...", path));
            #[allow(deprecated)]
            zip.start_file_from_path(name, options)?;
            let mut f = File::open(path)?;

            f.read_to_end(&mut buffer)?;
            zip.write_all(&*buffer)?;
            buffer.clear();
        } else if !name.as_os_str().is_empty() {
            // Only if not root! Avoids path spec / warning
            // and mapname conversion failed error on unzip
            spinner.set_message(format!("Aggiungo sottocartella {:?} ...", path));
            #[allow(deprecated)]
            zip.add_directory_from_path(name, options)?;
        }
    }
    zip.finish()?;

    spinner.finish_with_message("Pacchetto creato.");

    Result::Ok(())
}