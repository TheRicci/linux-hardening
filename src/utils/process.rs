use std::process::Command;
use std::io;

pub fn list_services() -> io::Result<()> {
    let output = Command::new("systemctl")
        .arg("list-units")
        .arg("--type=service")
        .arg("--state=running")
        .output()?;

    if output.status.success() {
        println!("{}", String::from_utf8_lossy(&output.stdout));
        return Ok(())
    } 
    Err(io::Error::new(io::ErrorKind::Other, "Failed to list running services"))
    
}

pub fn disable_service(service: &str) -> io::Result<()> {
    let output = Command::new("sudo")
        .arg("systemctl")
        .arg("disable")
        .arg(service)
        .output()?;

    if output.status.success() {
        println!("Service '{}' disabled successfully.", service);
        return Ok(())
    } 
    Err(io::Error::new(io::ErrorKind::Other, format!("Failed to disable service {}", service)))
    
}

pub fn restart_service(service: &str) -> io::Result<()> {
    let output = Command::new("sudo")
        .arg("systemctl")
        .arg("restart")
        .arg(service)
        .output()?;

    if output.status.success() {
        println!("Service '{}' restarted successfully.", service);
        return Ok(())
    } 
    Err(io::Error::new(io::ErrorKind::Other, format!("Failed to restart service {}", service)))
}