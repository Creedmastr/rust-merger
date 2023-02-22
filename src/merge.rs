use std::fs;

use crate::bcommands::Commands;

fn cmd_exe(cmd: String) {
    Commands::execute(cmd.to_string());
}

pub fn merge(path_previous: &String, path_next: &String) {
    if path_previous.ends_with("/") {
        fs::remove_dir_all(path_next).expect("Failed to remove the previous next dir (arg2)");
        fs::copy("/.rust-merger-bak/".to_string() + &path_previous, path_next)
            .expect("Failed to copy old file");
        cmd_exe("rm -r ".to_string() + path_previous);
    } else {
        // cmd_exe("rm -r ".to_string() + path_next);
        fs::remove_file(path_next).expect("Failed to remove the previous next file (arg2)");
        fs::copy("/.rust-merger-bak/".to_string() + &path_previous, path_next)
            .expect("Failed to copy old file");
        cmd_exe("rm -r ".to_string() + path_previous);
    }
}
