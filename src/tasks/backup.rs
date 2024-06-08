use std::process::Command;

pub fn schedule_backups(backup_paths: &[String]) {
    println!("Scheduling backups...");
    let backup_script = r#"
#!/bin/bash
BACKUP_DIR="/backup"
mkdir -p $BACKUP_DIR
for path in "$@"; do
    rsync -av --delete "$path" $BACKUP_DIR
done
"#;

    std::fs::write("/usr/local/bin/backup.sh", backup_script)
        .expect("Failed to write backup script");
    Command::new("sh")
        .arg("-c")
        .arg("chmod +x /usr/local/bin/backup.sh")
        .output()
        .expect("Failed to make backup script executable");

    let cron_job = format!("0 2 * * * /usr/local/bin/backup.sh {}", backup_paths.join(" "));
    std::fs::write("/etc/cron.d/backup", cron_job)
        .expect("Failed to write cron job");

    println!("Backups scheduled successfully.");
}

pub fn test_recovery(backup_paths: &[String]) {
    println!("Testing backup recovery...");
    for path in backup_paths {
        let output = Command::new("sh")
            .arg("-c")
            .arg(format!("rsync -av /backup/{} /restore/{}", path, path))
            .output()
            .expect("Failed to test backup recovery");
        if output.status.success() {
            println!("Recovery test for {} completed successfully.", path);
        } else {
            eprintln!("Failed to test recovery for {}: {}", path, String::from_utf8_lossy(&output.stderr));
        }
    }
}


