use chrono::Local;
use std::{
    env,
    fs::File,
    io::{self, BufWriter},
    path::Path,
};
use walkdir::WalkDir;
use zip::{ZipWriter, write::SimpleFileOptions};

// liste des exclusions du fichier build
const SKIP_PATHS: &[&str] = &[
    "build",
    ".dart_tool",
    ".git",
    ".idea",
    ".vscode",
    ".pub-cache",
    ".pub",
    "ios",
    "web",
    "linux",
    "macos",
    "windows",
    "android/.gradle",
    "android/build",
    "android/app/build",
    "android/app/.cxx",
    "android/.idea",
    "android/captures",
];

const SKIP_FILES: &[&str] = &[
    ".flutter-plugins",
    ".flutter-plugins-dependencies",
    ".packages",
    "android/local.properties",
    "android/key.properties",
];

const SKIP_EXTENSIONS: &[&str] = &["iml", "log", "DS_Store", "jks", "keystore"];
// Zip le dossier courant
fn main() -> io::Result<()> {
    // nom de l'archive zip
    let current_dir = env::current_dir()?;
    let folder_name = current_dir
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("archive");

    let timestamp = Local::now().format("%Y%m%d_%H%M%S");
    let nom_zip = format!("{}_remise_{}.zip", folder_name, timestamp);
    let zip_path = Path::new(&nom_zip);

    // initialisation du script Zip
    let zip_file = File::create(zip_path)?;
    let mut zip = ZipWriter::new(BufWriter::new(zip_file));

    // confirmation des permissions UNIX pour lecture et écriture
    let file_options = SimpleFileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated)
        .unix_permissions(0o644);

    let dir_options = SimpleFileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated)
        .unix_permissions(0o755);

    // création du fichier zip a la racine
    // ! temporaire !
    zip.add_directory(folder_name, dir_options)?;

    let mut zip_entry_str = String::with_capacity(256);

    // traitement de l'arborescence et filtrage
    let walker = WalkDir::new(".").into_iter().filter_entry(|e| {
        let path = e.path();
        let relative = path.strip_prefix(".").unwrap_or(path);

        let relative_str = relative.to_string_lossy().replace('\\', "/");

        // Exclusion des dossiers précis
        if e.file_type().is_dir() {
            for folder in SKIP_PATHS {
                if relative_str == *folder {
                    return false;
                }
            }
        }

        // Exclusion des fichiers précis
        if e.file_type().is_file() {
            if SKIP_FILES.contains(&relative_str.as_ref()) {
                return false;
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
        zip_entry_str.clear();
        zip_entry_str.push_str(folder_name);
        for component in relative_path.components() {
            zip_entry_str.push('/');
            zip_entry_str.push_str(&component.as_os_str().to_string_lossy());
        }

        if entry.file_type().is_file() {
            zip.start_file(&zip_entry_str, file_options)?;
            let mut file = File::open(entry_path)?;
            io::copy(&mut file, &mut zip)?;
        } else if entry.file_type().is_dir() {
            zip.add_directory(&zip_entry_str, dir_options)?;
        }
    }

    zip.finish()?;
    println!("Successfully created archive: {}", nom_zip);
    Ok(())
}
