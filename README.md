## Autoset_config: Versione ibrida Rust e Bash

### Introduzione

Autoset_config è un programma scritto in Rust con alcune aggiunte di Bash che consente di automatizzare la gestione dei file di configurazione. Il programma permette di clonare un repository GitHub contenente i file di configurazione e di scegliere tra tre funzionalità:

**1. Inserimento automatico dei file di configurazione nelle cartelle appropriate:** Questa funzionalità copia automaticamente i file di configurazione dalle cartelle di origine alle cartelle di destinazione specificate nel file di configurazione.

**2. Salvataggio dei file di configurazione in una cartella specifica:** Questa funzionalità sposta i file di configurazione esistenti dalla loro posizione attuale a una cartella di backup specificata nel file di configurazione.

**3. Inserimento automatico dei file di configurazione e salvataggio delle vecchie configurazioni:** Questa funzionalità combina le funzionalità 1 e 2, copiando i nuovi file di configurazione nelle cartelle di destinazione e spostando le vecchie configurazioni nella cartella di backup.

Tutte le informazioni relative al programma e alle sue impostazioni vengono salvate in un file .ini modificabile.

### Installazione

L'installazione di Autoset_config è semplice e richiede solo l'installazione del compilatore Rust. Ecco i passaggi per l'installazione su diverse distribuzioni Linux:

**Ubuntu:**

```bash
sudo apt install rustc cargo
```

**Arch Linux:**

```bash
sudo pacman -S rust
```

**Fedora:**

```bash
sudo dnf install rust
```

Una volta installato Rust, è possibile clonare il repository Autoset_config ed eseguire il programma:

```bash
git clone https://git-scm.com/book/en/Customizing-Git-Git-Configuration
cd Autoset_config
cargo build
cargo run
```

In alternativa, è possibile eseguire il file eseguibile direttamente dalla cartella `~/auto_config_rust_bash/target/debug/`.

### Esecuzione

Per eseguire Autoset_config, è sufficiente eseguire il comando `cargo run` dalla cartella del programma. Il programma presenterà un menu con le tre opzioni di funzionalità tra cui scegliere. Selezionando un'opzione, il programma eseguirà l'operazione corrispondente.

### Opzioni di configurazione

Le opzioni di configurazione di Autoset_config possono essere modificate nel file `settings.ini` situato all'interno della cartella del programma. In questo file è possibile modificare i percorsi delle cartelle di origine e destinazione, la cartella di backup e la funzionalità da utilizzare.

### Conclusione

Autoset_config è uno strumento utile per automatizzare la gestione dei file di configurazione, semplificando il processo di configurazione di software e sistemi. La sua interfaccia semplice e le opzioni di configurazione flessibili lo rendono adatto a utenti di tutti i livelli di esperienza.

### Note

* Assicurarsi di sostituire `https://git-scm.com/book/en/Customizing-Git-Git-Configuration` con l'URL effettivo del repository GitHub.
* Si consiglia di leggere la documentazione ufficiale di Rust e Bash per maggiori informazioni su come utilizzare questi linguaggi.
* Per un'esperienza utente più completa, si potrebbe considerare la creazione di un'interfaccia grafica per Autoset_config.

Spero che questo file .md sia completo e utile! 
