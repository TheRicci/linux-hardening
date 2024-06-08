use std::process::Command;

pub fn list_services() {
    println!("Listing running services...");
    let output = Command::new("sh")
        .arg("-c")
        .arg("systemctl list-units --type=service --state=running")
        .output()
        .expect("Failed to list running services");
    if output.status.success() {
        println!("Running services:\n{}", String::from_utf8_lossy(&output.stdout));
    } else {
        eprintln!("Failed to list running services: {}", String::from_utf8_lossy(&output.stderr));
    }
}

pub fn disable_service(service_name: &str) {
    println!("Disabling service: {}", service_name);
    let output = Command::new("sh")
        .arg("-c")
        .arg(format!("systemctl disable {} && systemctl stop {}", service_name, service_name))
        .output()
        .expect("Failed to disable service");
    if output.status.success() {
        println!("Service {} disabled successfully.", service_name);
    } else {
        eprintln!("Failed to disable service {}: {}", service_name, String::from_utf8_lossy(&output.stderr));
    }
}