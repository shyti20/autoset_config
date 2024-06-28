use std::process::Command;
use ini::Ini;

fn main() {
    // Percorso del file di configurazione
    let config_path = "config.ini";
    
    // Legge il file di configurazione
    let conf = Ini::load_from_file(config_path).expect("Errore nella lettura del file di configurazione");

    // Ottiene i path dal file di configurazione
    let repo_url = conf.section(Some("paths")).unwrap().get("repo_url").unwrap();
    let repo_path = conf.section(Some("paths")).unwrap().get("repo_path").unwrap();
    let file_path = conf.section(Some("paths")).unwrap().get("file").unwrap();
    let dest_path = conf.section(Some("paths")).unwrap().get("destination").unwrap();

    // Lancia lo script Bash per clonare la repository
    println!("Clonazione della repository da {}...", repo_url);
    let output = Command::new("bash")
        .arg("clone_repo.sh")
        .arg(repo_url)
        .arg(repo_path)
        .output()
        .expect("Errore nel clonare la repository");
    if !output.status.success() {
        eprintln!("Errore nel clonare la repository: {}", String::from_utf8_lossy(&output.stderr));
        return;
    }

    // Lancia lo script Bash per copiare il file
    println!("Copia del file da {} a {}...", file_path, dest_path);
    let output = Command::new("bash")
        .arg("copy_file.sh")
        .arg(file_path)
        .arg(dest_path)
        .output()
        .expect("Errore nel copiare il file");
    if !output.status.success() {
        eprintln!("Errore nel copiare il file: {}", String::from_utf8_lossy(&output.stderr));
        return;
    }

    println!("Operazione completata con successo!");
}