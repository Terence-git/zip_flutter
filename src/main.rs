use chrono::Local;
use std::{env, io};

fn main() -> io::Result<()> {
    let current_dir = env::current_dir()?;
    let forlder_name = current_dir.file_name().and_then(|n| n.to_str()).unwrap_or("archive");
    let timestamp = Local::now().format("%Y%m%d_%H%M%S");
    //println!("Répertoire actuel : {}", current_dir.display());
    //println!("nom du répertoire : {}", forlder_name);
    //println!("date : {}", timestamp);
    let nom_zip = format!("{}_remise_{}", forlder_name, timestamp);
    //print!("{}", nom_zip);
    
    Ok(())
}