use std::{fs, path::Path};

pub fn backup(file_path: String) {
    if !Path::new("~/.rust-bak/").exists() {
        fs::create_dir("~/.rust-bak/").expect("Error when searching for bak dir");
    }

    fs::copy(file_path, "~/.rust-bak/").expect("Error when backing files");
}
