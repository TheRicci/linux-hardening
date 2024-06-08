use std::process::Command;
use std::io;

pub fn configure_firewall() -> io::Result<()> {
    let output = Command::new("sudo")
        .arg("ufw")
        .arg("default")
        .arg("deny")
        .output()?;

    if !output.status.success() {
        return Err(io::Error::new(io::ErrorKind::Other, "Failed to set default deny policy"));
    }

    let output = Command::new("sudo")
        .arg("ufw")
        .arg("enable")
        .output()?;

    if output.status.success() {
        Ok(())
    } else {
        Err(io::Error::new(io::ErrorKind::Other, "Failed to enable firewall"))
    }
}

pub fn allow_service(service: &str) -> io::Result<()> {
    let output = Command::new("sudo")
        .arg("ufw")
        .arg("allow")
        .arg(service)
        .output()?;

    if output.status.success() {
        Ok(())
    } else {
        Err(io::Error::new(io::ErrorKind::Other, format!("Failed to allow service {}", service)))
    }
}

pub fn deny_service(service: &str) -> io::Result<()> {
    let output = Command::new("sudo")
        .arg("ufw")
        .arg("deny")
        .arg(service)
        .output()?;

    if output.status.success() {
        Ok(())
    } else {
        Err(io::Error::new(io::ErrorKind::Other, format!("Failed to deny service {}", service)))
    }
}