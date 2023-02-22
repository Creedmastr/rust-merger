use std::fs;

use crate::bcommands::Commands;

fn cmd_exe(cmd: String) {
    Commands::execute(cmd.to_string());
}

pub fn merge(path_previous: &String, path_next: &String) {
    if Commands::_return_os_family() == "windows" {
        if path_previous.ends_with("/") {
            cmd_exe("rm -r ".to_string() + path_next);
            cmd_exe(
                "Copy-Item -Path ".to_string()
                    + path_previous
                    + " -Destination "
                    + path_next
                    + " -Recurse",
            );
            cmd_exe("rm -r ".to_string() + path_previous);
        } else {
            // cmd_exe("rm -r ".to_string() + path_next);
            fs::remove_file(path_next).expect("");
            fs::copy("/.rust-merger-bak/".to_string() + &path_previous, path_next);
            cmd_exe("rm -r ".to_string() + path_previous);
        }
    } else {
        cmd_exe("rm -r ".to_string() + path_next);
        cmd_exe("cp ".to_string() + path_previous + " " + path_next);
        cmd_exe("rm -r ".to_string() + path_previous);
    }
}
