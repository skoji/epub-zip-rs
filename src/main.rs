use std::{error::Error, fs::File, io, path::Path};

use walkdir::{DirEntry, WalkDir};
use zip::write::FileOptions;

fn main() {
    let mut args = std::env::args();
    if args.len() < 2 {
        eprintln!("should specify directory");
        std::process::exit(1);
    }
    let src_dir = args.nth(1).unwrap();
    dozip(&src_dir).expect("panic");
}

fn zip_dir(
    it: &mut dyn Iterator<Item = DirEntry>,
    prefix: &str,
    file: File,
) -> Result<(), Box<dyn Error>> {
    let mut zip = zip::ZipWriter::new(file);
    let options = FileOptions::default().compression_method(zip::CompressionMethod::Deflated);
    for entry in it {
        let path = entry.path();
        let entry_name = path
            .strip_prefix(Path::new(prefix))?
            .to_string_lossy()
            .into_owned();
        if path.is_file() {
            zip.start_file(entry_name, options)?;
        }
    }
    zip.finish()?;
    Ok(())
}

fn dozip(src_dir: &str) -> Result<(), Box<dyn Error>> {
    let src_path = Path::new(src_dir);
    if !src_path.is_dir() {
        return Err(Box::new(io::Error::new(
            io::ErrorKind::NotFound,
            "no directory",
        )));
    }
    let dest_path = src_path.with_extension("epub");
    let dest_file = File::create(&dest_path)?;
    let walkdir = WalkDir::new(src_dir.to_string());
    let mut iter = walkdir.into_iter().filter_map(|e| e.ok());
    zip_dir(&mut iter, src_dir, dest_file)?;
    Ok(())
}
