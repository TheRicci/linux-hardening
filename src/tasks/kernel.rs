use std::process::Command;
use crate::config::config::KernelParameter;

pub fn harden_kernel(parameters: &[KernelParameter]) {
    println!("Hardening kernel parameters...");
    let mut sysctl_conf = String::new();

    for param in parameters {
        sysctl_conf.push_str(&format!("{} = {}\n", param.name, param.value));
    }

    std::fs::write("/etc/sysctl.d/99-hardening.conf", sysctl_conf)
        .expect("Failed to write kernel parameters");

    Command::new("sh")
        .arg("-c")
        .arg("sysctl --system")
        .output()
        .expect("Failed to apply kernel parameters");

    println!("Kernel parameters hardened successfully.");
}

/* 
pub fn harden_kernel() {
    let sysctl_config = "/etc/sysctl.conf";

    // Add some basic kernel hardening settings
    let settings = [
        "net.ipv4.ip_forward = 0",
        "net.ipv4.conf.all.accept_source_route = 0",
        "net.ipv4.conf.all.accept_redirects = 0",
        "net.ipv4.conf.all.secure_redirects = 0",
        "net.ipv4.conf.all.log_martians = 1",
        "kernel.randomize_va_space = 2",
    ];

    let output = Command::new("sh")
        .arg("-c")
        .arg(format!("echo '{}' | sudo tee -a {}", settings.join("\n"), sysctl_config))
        .output()
        .expect("Failed to update sysctl configuration");

    if output.status.success() {
        println!("Kernel parameters hardened.");
        // Apply the new settings
        Command::new("sudo")
            .arg("sysctl")
            .arg("-p")
            .output()
            .expect("Failed to apply sysctl settings");
    } else {
        eprintln!("Failed to harden kernel parameters: {}", String::from_utf8_lossy(&output.stderr));
    }
}
*/