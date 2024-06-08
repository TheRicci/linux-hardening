/* 
use std::fs;
use std::io::{self, Write};
use std::os::unix::fs::PermissionsExt;

pub fn set_permissions(path: &str, permissions: u32) -> io::Result<()> {
    let metadata = fs::metadata(path)?;
    let mut perms = metadata.permissions();
    perms.set_mode(permissions);
    fs::set_permissions(path, perms)
}

pub fn backup_permissions(path: &str) -> io::Result<()> {
    let metadata = fs::metadata(path)?;
    let permissions = metadata.permissions().mode();
    let mut file = fs::OpenOptions::new().append(true).create(true).open("permissions_backup.txt")?;
    writeln!(file, "{} {}", path, permissions)?;
    Ok(())
}

pub fn restore_permissions(path: &str) -> io::Result<()> {
    let contents = fs::read_to_string("permissions_backup.txt")?;
    for line in contents.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() == 2 && parts[0] == path {
            let permissions = u32::from_str_radix(parts[1], 8)?;
            set_permissions(path, permissions)?;
        }
    }
    Ok(())
}
*/