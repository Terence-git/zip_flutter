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

    // 1. Ajouter d'abord le dossier parent racine dans l'archive
    zip.add_directory(folder_name, options)?;

    let walker = WalkDir::new(".").into_iter().filter_entry(|e| {
        let path = e.path();

        if e.depth() == 1 && e.file_type().is_dir() {
            if let Some(name) = path.file_name().and_then(|s| s.to_str()) {
                if SKIP_FOLDERS.contains(&name) {
                    return false;
                }
            }
        }

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

        if relative_path == zip_path || relative_path.as_os_str().is_empty() {
            continue;
        }

        // 2. Construire le chemin interne avec le dossier parent (ex: mon_projet/lib/main.dart)
        let zip_entry_path = Path::new(folder_name).join(relative_path);
        let zip_entry_str = zip_entry_path.to_str().unwrap().replace('\\', "/");

        if entry_path.is_file() {
            zip.start_file(&zip_entry_str, options)?;
            let mut file = File::open(entry_path)?;
            file.read_to_end(&mut buffer)?;
            zip.write_all(&buffer)?;
            buffer.clear();
        } else {
            zip.add_directory(&zip_entry_str, options)?;
        }
    }

    zip.finish()?;

    println!("Successfully created archive: {}", nom_zip);
    Ok(())
}