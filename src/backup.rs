use std::{fs, ops::Not, path::Path};

use crate::bcommands::Commands;

fn cmd_exe(cmd: String) {
    Commands::execute(cmd.to_string());
}

pub fn backup(path: &String) {
    if Path::new("/.rust-merger-bak/").exists().not() {
        fs::create_dir("/.rust-merger-bak/").expect("Error when searching for bak dir");
    }

    cmd_exe("cp ".to_string() + &path + " /.rust-merger-bak/");
    
    fs::copy(&path, "/.rust-merger-bak/".to_string() + &path).expect("");
}
