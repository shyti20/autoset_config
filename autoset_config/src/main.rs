use std::process::Command;
use ini::Ini;
use clap::{App, Arg};
use autoset_config_lib;

fn main() {
    let matches = App::new("autoset-config")
        .version("0.1.0")
        .author("shyti20")
        .arg(
            Arg::with_name("number")
            .short("n")
            .long("number")
            .takes_value(true)
            .required(true)
        )
        .get_matches();
    let num_str = matches.value_of("number").unwrap();

    let num: u32 = num_str.parse().unwrap();
    
    let config_path = "~/auto_config/settings.ini";
    let conf = Ini::load_from_file(config_path).expect("Errore nella lettura del file di configurazione");

    // Eseguire il clone del repository (se necessario)
    if let Some(repos_section) = conf.section(Some("repos")) {
        clone_repo(conf);
    }

    if num == 1 {
        println!("Autoset dei file in corso...");
        autoset(conf, "files", "path");
    } else if num == 2 {
        println!("Autoset e salvataggio delle vecchie configurazione in corso...");
        save_old();
    }
}

