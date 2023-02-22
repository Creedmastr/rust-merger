use std::{fs, ops::Not, path::Path};

use crate::bcommands::Commands;

fn cmd_exe(cmd: String) {
    Commands::execute(cmd.to_string());
}

pub fn backup(file_path: &String) {
    if Path::new("/.rust-bak/").exists().not() {
        fs::create_dir("/.rust-bak/").expect("Error when searching for bak dir");
    }

    cmd_exe("cp ".to_string() + &file_path + " /.rust-bak/");
}
