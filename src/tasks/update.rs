use std::fs;
use std::process::Command;

pub fn system_update(update_interval: &str) {
    println!("Setting up automatic updates...");

    let auto_update_script = r#"
                                    #!/bin/bash
                                    apt-get update && apt-get upgrade -y
                                    "#;

    let cron_directory = match update_interval {
        "daily" => "/etc/cron.daily",
        "weekly" => "/etc/cron.weekly",
        "monthly" => "/etc/cron.monthly",
        _ => {
            eprintln!("Invalid update interval specified. Use 'daily', 'weekly', or 'monthly'.");
            return;
        }
    };

    let script_path = format!("{}/auto_update.sh", cron_directory);

    fs::write(&script_path, auto_update_script)
        .expect("Failed to write auto update script");

    Command::new("sh")
        .arg("-c")
        .arg(format!("chmod +x {}", script_path))
        .output()
        .expect("Failed to make auto update script executable");

    println!("Automatic updates configured for {}.", update_interval);

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
