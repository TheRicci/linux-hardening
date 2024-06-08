use std::process::Command;
use crate::config::config::Repository;

pub fn system_update( repositories: &[Repository]) {
    println!("Setting up automatic updates...");
    let auto_update_script = r#"
                                    #!/bin/bash
                                    apt-get update && apt-get upgrade -y
                                    "#;
    std::fs::write("/etc/cron.daily/auto_update.sh", auto_update_script)
        .expect("Failed to write auto update script");
    Command::new("sh")
        .arg("-c")
        .arg("chmod +x /etc/cron.daily/auto_update.sh")
        .output()
        .expect("Failed to make auto update script executable");
    println!("Automatic updates configured.");
    
    for repo in repositories {
        println!("Adding repository: {}", repo.url);
        let output = Command::new("sh")
            .arg("-c")
            .arg(format!("add-apt-repository -y {}", repo.url))
            .output()
            .expect("Failed to add repository");
        if output.status.success() {
            println!("Repository {} added successfully.", repo.url);
        } else {
            eprintln!("Failed to add repository {}: {}", repo.url, String::from_utf8_lossy(&output.stderr));
        }
    }

    println!("Updating system...");
    let output = Command::new("sh")
        .arg("-c")
        .arg("apt-get update && apt-get upgrade -y")
        .output()
        .expect("Failed to update system");
    if output.status.success() {
        println!("System updated successfully.");
    } else {
        eprintln!("Failed to update system: {}", String::from_utf8_lossy(&output.stderr));
    }
}