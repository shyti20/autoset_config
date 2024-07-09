use std::process::Command;
use std::io::Error; // Assuming you might encounter IO errors
use ini::Ini;

// repository clone
pub fn clone_repo(conf: Ini) -> Result<(), Error> {
    let repo_url = conf.section(Some("repos")).unwrap().get("conf_repo").unwrap();
    let repo_path = conf.section(Some("repos")).unwrap().get("path_repo").unwrap();
    println!("Clonazione della repository da {}...", repo_url);

    let output = Command::new("bash")
        .arg("clone_repo.sh")
        .arg(repo_url)
        .arg(repo_path)
        .output()?;

    if !output.status.success() {
        eprintln!("Errore nel clonare la repository: {}", String::from_utf8_lossy(&output.stderr));
        //return Err(Error::new(io::ErrorKind::Other, "Clone failed"));
    }

    println!("Operazione completata con successo!");
    Ok(())
}

// Only autoset
pub fn autoset(conf: Ini, section_path: String, section_dest_path: String) {
    let files_section = conf.section(Some(section_path)).unwrap();
    let section_dest = conf.section(Some(section_dest_path)).unwrap();

    for (key, value) in files_section.iter() {
        if value.to_string() != "-" {
            for (key_dest, value_dest) in section_dest.iter() {
                if value_dest.to_string() != "" {
                    if key_dest.to_string().split("_").next().unwrap() == key.to_string().split("_").next().unwrap() {
                        //let dest = conf.section(Some("path")).unwrap().get(key_dest).unwrap();
                        //let file = conf.section(Some("files")).unwrap().get(key).unwrap();

                        let command = format!("bash copy_file.sh {} {}", value, value_dest);

                        Command::new("bash").arg(command);
                
                        println!("Operazione completata con successo!");
                    }
                }
            }
        }
    }
}

// Autoset and save old config file
pub fn save_old(conf: Ini, section_path: String, section_dest_path: String) {
    let files_section = conf.section(Some(section_path)).unwrap();
    let section_dest = conf.section(Some(section_dest_path.clone())).unwrap();
    let save_old_dir = conf.section(Some(section_dest_path)).unwrap().get("save_old").unwrap();

    for (key, value) in files_section.iter() {
        if value.to_string() != "-" {
            for (key_dest, value_dest) in section_dest.iter() {
                if value_dest.to_string() != "" {
                    if key_dest.to_string().split("_").next().unwrap() == key.to_string().split("_").next().unwrap() {
                        let command = format!("bash save_old.sh {} {} {}", value, value_dest, save_old_dir);

                        Command::new("bash").arg(command);
                        
                        println!("Operazione completata con successo!");
                    }
                }
            }
        }
    }
}