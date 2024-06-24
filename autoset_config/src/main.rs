
/*// Acquisizione dei parametri nel file ini
fn take_data() -> Result<()> {

    Ok(())
}

// Processo di colazione della repo
fn colone_repo() -> Result<()> {

}

// Opzione dei set dei file di config ed eliminazione di quelli vecchi
fn set_config() -> Result<()> {

}

// Opzione di solo download dei file di config
fn only_download() -> Result<()> {

}

// Opzione di set dei file di config e salvataggio di quelli vecchi
fn set_and_save_old() -> Result<()> {

}*/

use config::{Config, File, FileFormat};
// use git2::{Repository, Remote, FetchOptions};
use std::path::Path;

fn main() {
    // Create a ConfigBuilder
    let builder = Config::builder()
        .add_source(File::new("C:\\Users\\FRANSHIT\\Desktop\\rust\\autoset_config\\settings.ini", FileFormat::Ini));
    
    // Build the Config with error handling
    let settings = builder.build();

    // Read variables from the Config
    let rofi_config: String = settings.get::<String>("files.rofi_config");
    let only_download: bool = settings.get::<bool>("options.only_download");
    let user: String = settings.get::<String>("database.user");
    let repo: String = settings.get::<String>("repos.conf_repos");
    let download_path: String = settings.get::<String>("path.only_download_path");

    let path_rofi = &Path::new(rofi_config);

}
