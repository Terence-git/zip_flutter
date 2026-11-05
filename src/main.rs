use chrono::Local;
use std::{
    env,
    fs::File,
    io::{self, Read, Write},
    path::Path,
};
use walkdir::WalkDir;
use zip::{write::SimpleFileOptions, ZipWriter};

const SKIP_EXTENSIONS: &[&str] = &["iml", "log"];
const SKIP_FILES: &[&str] = &[".gitignore"];
const SKIP_FOLDERS: &[&str] = &[
    "build", ".dart_tool", ".git", ".idea", ".vscode", 
    "android", "ios", "web", "linux", "macos", "windows"
];

fn main() -> io::Result<()> {
    let current_dir = env::current_dir()?;
    let folder_name = current_dir
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("archive");

    let timestamp = Local::now().format("%Y%m%d_%H%M%S");
    let nom_zip = format!("{}_remise_{}.zip", folder_name, timestamp);
    let zip_path = Path::new(&nom_zip);

    let zip_file = File::create(zip_path)?;
    let mut zip = ZipWriter::new(zip_file);

    let options = SimpleFileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated)
        .unix_permissions(0o755);

    let mut buffer = Vec::new();

    let walker = WalkDir::new(".").into_iter().filter_entry(|e| {
        let path = e.path();

        // 1. Ignorer les dossiers cibles à la racine (depth == 1)
        if e.depth() == 1 && e.file_type().is_dir() {
            if let Some(name) = path.file_name().and_then(|s| s.to_str()) {
                if SKIP_FOLDERS.contains(&name) {
                    return false; // Prune : WalkDir n'entrera PAS dans ce dossier
                }
            }
        }

        // 2. Ignorer les fichiers spécifiques et extensions
        if e.file_type().is_file() {
            if let Some(name) = path.file_name().and_then(|s| s.to_str()) {
                if SKIP_FILES.contains(&name) {
                    return false;
                }
            }
            if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
                if SKIP_EXTENSIONS.contains(&ext) {
                    return false;
                }
            }
        }

        true
    });

    for entry in walker.filter_map(|e| e.ok()) {
        let entry_path = entry.path();
        let relative_path = entry_path.strip_prefix(".").unwrap_or(entry_path);

        // Ignorer l'archive ZIP en cours de création
        if relative_path == zip_path {
            continue;
        }

        if entry_path.is_file() {
            zip.start_file(relative_path.to_str().unwrap(), options)?;
            let mut file = File::open(entry_path)?;
            file.read_to_end(&mut buffer)?;
            zip.write_all(&buffer)?;
            buffer.clear();
        } else if !relative_path.as_os_str().is_empty() {
            zip.add_directory(relative_path.to_str().unwrap(), options)?;
        }
    }

    zip.finish()?;

    println!("Successfully created archive: {}", nom_zip);
    Ok(())
}