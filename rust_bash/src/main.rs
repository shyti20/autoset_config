use std::process::Command;
use ini::Ini;

fn main() {
    // Caricare il file di configurazione
    let config_path = "~/auto_config/settings.ini";
    let conf = Ini::load_from_file(config_path).expect("Errore nella lettura del file di configurazione");

    // Eseguire il clone del repository (se necessario)
    if let Some(repos_section) = conf.section(Some("repos")) {
        clone_repo(repos_section, &conf);
    }

    // Copiare i file di configurazione
    autoset(&conf);
}

fn clone_repo(conf: Ini ) {
    let repo_url = conf.section(Some("repos")).unwrap().get("conf_repo").unwrap();
    let repo_path = conf.section(Some("repos")).unwrap().get("path_repo").unwrap();
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

}

fn autoset(section_file: String, section_dest: String, conf: Ini) {
    let files_section = conf.section(Some(section_file)).unwrap();

    for (file_name, dest_path) in files_section.iter() {
        let dest_path = conf.section(Some(section_dest)).unwrap().get("path").unwrap();

        let command = format!("bash copy_file.sh {} {}", file_path, dest_path);

        match Command::new("bash").arg(command).output() {
            Ok(output) => {
                if !output.status.success() {
                    eprintln!("Errore nel copiare il file: {}", String::from_utf8_lossy(&output.stderr));
                    return;
                }
            }
            Err(err) => {
                eprintln!("Errore durante l'esecuzione del comando: {}", err);
                return;
            }
        }

        println!("Operazione completata con successo!");
    }
}
