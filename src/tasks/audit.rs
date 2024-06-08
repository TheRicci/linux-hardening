use std::process::Command;

pub fn run_audit(audit_tool: &str) {
    match audit_tool {
        "Lynis" => {
            println!("Running Lynis audit...");
            let output = Command::new("sh")
                .arg("-c")
                .arg("apt-get install -y lynis && lynis audit system")
                .output()
                .expect("Failed to run Lynis audit");
            if output.status.success() {
                println!("Lynis audit completed successfully.");
            } else {
                eprintln!("Failed to run Lynis audit: {}", String::from_utf8_lossy(&output.stderr));
            }
        }
        _ => eprintln!("Unsupported audit tool: {}", audit_tool),
    }
}