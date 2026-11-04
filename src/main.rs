use std::env;
use std::path::PathBuf;
use chrono::Local;

fn main() {
    let current_dir: PathBuf = env::current_dir()
        .expect("impossible de lire le dossier courant.");

    let folder_name = current_dir
        .file_name()
        .and_then(|n| n.to_str())
        .expect("invalide");

    let date_str = Local::now().format("%Y%m%d_%H%M%S");

    let  zip_filename = format!("{}_remise_{}", folder_name, date_str);

    println!("Dossier source : {}", folder_name);
    println!("Fichier cible  : {}", zip_filename);
}