use std::{process::Command};

#[derive(Debug)]
pub struct Commands {}

impl Commands {
    pub fn execute(command: String) {
        if cfg!(target_os = "windows") {
            Command::new("powershell")
                    .arg("/C")
                    .arg(command)
                    .output()
                    .expect("Failed to execute the command")
        } else {
            Command::new("sh")
                    .arg("-c")
                    .arg(command)
                    .output()
                    .expect("Failed to execute the command")
        };
    }
    
    pub fn _return_os() -> String {
        if cfg!(target_os = "windows") {
            "windows".to_string()
        } else if cfg!(target_os = "linux") {
            "linux".to_string()
        } else {
            "macos".to_string()
        }
    }

    pub fn _return_os_family() -> String {
        if cfg!(target_family = "windows") {
            "windows".to_string()
        } else {
            "unix".to_string()
        }
    }
}
