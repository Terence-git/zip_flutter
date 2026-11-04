use std::env;
use std::path::PathBuf;

fn main() {
    let current_dir: PathBuf = env::current_dir()
        .expect("impossible de lire le dossier courant.");

    let folder_name = current_dir
        .file_name()
        .and_then(|n| n.to_str())
        .expect("invalide");

    println!("{}", folder_name);
}