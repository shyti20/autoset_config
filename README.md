# Autoset_config: Hybrid Rust and Bash Version

**This repository is still under development and testing. It is not yet ready for public use.**

## Introduction

Autoset_config is a program written in Rust with some Bash additions that allows you to automate the management of configuration files. The program allows you to clone a GitHub repository containing the configuration files and choose between three functionalities:

**1. Automatically insert configuration files into the appropriate folders:** This feature automatically copies the configuration files from the source folders to the destination folders specified in the configuration file.

**2. Save configuration files to a specific folder:** This feature moves existing configuration files from their current location to a backup folder specified in the configuration file.

**3. Automatically insert configuration files and save old configurations:** This feature combines features 1 and 2, copying new configuration files to the destination folders and moving old configurations to the backup folder.

All information about the program and its settings is saved in a modifiable .ini file.

## Installation

Installing Autoset_config is simple and only requires the installation of the Rust compiler. Here are the installation steps for different Linux distributions:

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

Once Rust is installed, you can clone the Autoset_config repository and run the program:

```bash
git clone https://github.com/shyti20/autoset_config/edit/rust_bash/
cd autoset_config
cargo build
cargo run
```

Alternatively, you can run the executable file directly from the `~/auto_config_rust_bash/target/debug/` folder.

### Running the Program as a Terminal Command

To run your compiled Rust program as a terminal command, follow these steps:

1. **Compile the Program**

   ```bash
   rustc main.rs -o autoset-config
   ```

2. **Move the Executable**

   For a global setup (requires superuser permissions):

   ```bash
   sudo mv autoset /usr/local/bin
   ```

   For a local setup (no superuser permissions required):

   ```bash
   mkdir -p $HOME/.local/bin
   mv autoset-config $HOME/.local/bin
   ```

3. **Update PATH**

   Ensure `$HOME/.local/bin` is in your PATH. Check your PATH with:

   ```bash
   echo $PATH
   ```

   If `$HOME/.local/bin` is not in your PATH, add it by editing `.bashrc` (or `.zshrc` if using zsh):

   ```bash
   echo 'export PATH=$HOME/.local/bin:$PATH' >> ~/.bashrc
   source ~/.bashrc
   ```

4. **Execute the Command**

   Now you can run your program with:

   ```bash
   autoset-config
   ```

## Configuration Options

Autoset_config's configuration options can be modified in the `settings.ini` file located in the program's folder. In this file, you can change the paths to the source and destination folders, the backup folder, and the functionality to use.

## User Notes

1. **Make sure you clone the correct repository:**

   Ensure you clone the correct repository from the following URL: [https://github.com/shyti20/autoset_config/](https://github.com/shyti20/autoset_config/)

2. **Modify the URL of your GitHub repository in the `settings.ini` file:**

   Open the `settings.ini` file located in the `Autoset_config` folder. Replace the URL `https://your-github-username.github.io/your-repo-name` with the URL of your GitHub repository.

3. **Modify the paths of all parameters required for execution. If a parameter is not needed, simply leave a hyphen (`-`).**

**Example:**

```
[repos]
conf_repo = https://github.com/user/repo.git
path_repo = ~/repo_download/this_repo    

[files]
rofi_conf = ~/my_dir/file                                                                                                  
waybar_conf =  ~/my_dir/file                                
alacritty_conf =  ~/my_dir/file                             
hyprconf = ~/my_dir/file   

[path]            
rofi_path = ~/my_dir/
waybar_path = ~/my_dir/
alacritty_path = ~/my_dir/
hypr_path = ~/my_dir/
```

**Additional notes:**

* Make sure to use the correct syntax for configuration options.
* If you are unsure about any of the configuration options, please refer to the program's documentation.
* You can always add additional configuration options as needed.

By following these notes, you can ensure that your Autoset_config program is running smoothly and efficiently.

## Conclusion

Autoset_config is a useful tool for automating the management of configuration files, simplifying the process of configuring software and systems. Its simple interface and flexible configuration options make it suitable for users of all experience levels.
