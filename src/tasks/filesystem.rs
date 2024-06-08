use std::fs;
use std::process::Command;
use std::io::{self, Write};

pub fn set_permissions(path: &str, permissions: &str, recursive: bool) {
    if recursive {
        let result = Command::new("sudo")
            .arg("chmod")
            .arg("-R")
            .arg(permissions)
            .arg(path)
            .output();

        match result {
            Ok(output) => {
                if output.status.success() {
                    println!("Permissions for '{}' set to '{}' recursively.", path, permissions);
                } else {
                    eprintln!("Failed to set permissions recursively for '{}': {}", path, String::from_utf8_lossy(&output.stderr));
                }
            }
            Err(e) => eprintln!("Failed to execute chmod command: {}", e),
        }
    } else {
        let result = Command::new("sudo")
            .arg("chmod")
            .arg(permissions)
            .arg(path)
            .output();

        match result {
            Ok(output) => {
                if output.status.success() {
                    println!("Permissions for '{}' set to '{}'.", path, permissions);
                } else {
                    eprintln!("Failed to set permissions for '{}': {}", path, String::from_utf8_lossy(&output.stderr));
                }
            }
            Err(e) => eprintln!("Failed to execute chmod command: {}", e),
        }
    }
}

pub fn backup_permissions(path: &str) -> io::Result<()> {
    let output = Command::new("stat")
        .arg("-c")
        .arg("%a %n")
        .arg(path)
        .output()?;

    if output.status.success() {
        let permissions = String::from_utf8_lossy(&output.stdout);
        let mut file = fs::OpenOptions::new().append(true).create(true).open("permissions_backup.txt")?;
        writeln!(file, "{}", permissions)?;
        println!("Permissions for '{}' backed up successfully.", path);
    } else {
        eprintln!("Failed to get permissions for '{}': {}", path, String::from_utf8_lossy(&output.stderr));
    }

    Ok(())
}
