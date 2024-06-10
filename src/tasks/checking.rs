use std::process::Command;

pub fn ensure_service_installed_and_enabled(package: &str, service: &str) -> bool {
    // Check if the package is installed
    let output = Command::new("sh")
        .arg("-c")
        .arg(format!("dpkg -l | grep {}", package))
        .output()
        .expect("Failed to check if package is installed");
    if output.status.success() {
        println!("{} is already installed.", package);

        // Check if the service is enabled
        let output = Command::new("sh")
            .arg("-c")
            .arg(format!("systemctl is-enabled {}", service))
            .output()
            .expect("Failed to check if service is enabled");
        if output.status.success() {
            println!("{} service is already enabled.", service);
            false
        } else {
            println!("{} service is not enabled. Enabling it now...", service);

            // Enable the service
            let output = Command::new("sh")
                .arg("-c")
                .arg(format!("systemctl enable {} && systemctl start {}", service, service))
                .output()
                .expect("Failed to enable service");
            if output.status.success() {
                println!("{} service enabled successfully.", service);
                false
            } else {
                eprintln!("Failed to enable {} service: {}", service, String::from_utf8_lossy(&output.stderr));
                false
            }
        }
    } else {
        println!("{} is not installed. Installing it now...", package);

        // Install the package
        let output = Command::new("sh")
            .arg("-c")
            .arg(format!("apt-get install -y {}", package))
            .output()
            .expect("Failed to install package");
        if output.status.success() {
            println!("{} installed successfully.", package);

            // Enable the service after installation
            let output = Command::new("sh")
                .arg("-c")
                .arg(format!("systemctl enable {} && systemctl start {}", service, service))
                .output()
                .expect("Failed to enable service after installation");
            if output.status.success() {
                println!("{} service enabled successfully.", service);
                true
            } else {
                eprintln!("Failed to enable {} service after installation: {}", service, String::from_utf8_lossy(&output.stderr));
                false
            }
        } else {
            eprintln!("Failed to install {}: {}", package, String::from_utf8_lossy(&output.stderr));
            false
        }
    }
}