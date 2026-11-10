use chrono::Local;
use std::{
    env,
    fs::File,
    io::{self, BufWriter},
    path::Path,
};
use walkdir::WalkDir;
use zip::{CompressionMethod, ZipWriter, write::SimpleFileOptions};

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
    ".DS_Store",
    "android/local.properties",
    "android/key.properties",
];

const SKIP_EXTENSIONS: &[&str] = &["iml", "log", "jks", "keystore", "DS_Store"];

fn verifier_projet_flutter(projet_dir: &Path) -> bool {
    let pubspec = projet_dir.join("pubspec.yaml");
    let lib = projet_dir.join("lib");

    if !pubspec.is_file() {
        eprintln!("Erreur : pubspec.yaml introuvable.");
        return false;
    }

    if !lib.is_dir() {
        eprintln!("Erreur : dossier lib/ introuvable.");
        return false;
    }

    println!("Projet Flutter détecté");
    true
}

fn main() -> io::Result<()> {
    let current_dir = env::current_dir()?;
        if !verifier_projet_flutter(&current_dir) {
        std::process::exit(1);
    }
    let folder_name = current_dir
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("archive");

    let timestamp = Local::now().format("%Y%m%d_%H%M%S");
    let nom_zip = format!("{}_remise_{}.zip", folder_name, timestamp);
    let zip_path = Path::new(&nom_zip);

    let options = SimpleFileOptions::default()
        .compression_method(CompressionMethod::Deflated)
        .compression_level(Some(9));

    let zip_file = File::create(zip_path)?;
    let mut zip = ZipWriter::new(BufWriter::new(zip_file));

    zip.add_directory(folder_name, options)?;

    let walker = WalkDir::new(".").into_iter().filter_entry(|e| {
        let path = e.path();

        if path == zip_path {
            return false;
        }

        let relative = path.strip_prefix(".").unwrap_or(path);
        let relative_str = relative.to_string_lossy().replace('\\', "/");

        if relative_str.is_empty() {
            return true;
        }

        if e.file_type().is_dir() {
            for folder in SKIP_PATHS {
                if relative_str == *folder || relative_str.starts_with(&format!("{}/", folder)) {
                    return false;
                }
            }
        }

        if e.file_type().is_file() {
            if SKIP_FILES.contains(&relative_str.as_str()) {
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

        if relative_path.as_os_str().is_empty() {
            continue;
        }

        let clean_rel_path = relative_path.to_string_lossy().replace('\\', "/");
        let zip_entry_name = format!("{}/{}", folder_name, clean_rel_path);

        if entry.file_type().is_file() {
            zip.start_file(&zip_entry_name, options)?;
            let mut file = File::open(entry_path)?;
            io::copy(&mut file, &mut zip)?;
        } else if entry.file_type().is_dir() {
            zip.add_directory(&zip_entry_name, options)?;
        }
    }

    zip.finish()?;

    Ok(())
}
