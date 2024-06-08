use std::process::Command;

pub fn deploy_tool(tool_name: &str) {
    match tool_name {
        "Snort" => {
            println!("Deploying Snort IDS...");
            let output = Command::new("sh")
                .arg("-c")
                .arg("apt-get install -y snort")
                .output()
                .expect("Failed to install Snort");
            if output.status.success() {
                println!("Snort installed successfully.");
                configure_snort();
            } else {
                eprintln!("Failed to install Snort: {}", String::from_utf8_lossy(&output.stderr));
            }
        }
        _ => eprintln!("Unsupported IDS tool: {}", tool_name),
    }
}

fn configure_snort() {
    println!("Configuring Snort...");
    // Add Snort configuration logic here
    let snort_conf = r#"
# Snort configuration
var HOME_NET any
var EXTERNAL_NET !$HOME_NET

# Configure additional rules here
"#;

    std::fs::write("/etc/snort/snort.conf", snort_conf)
        .expect("Failed to write Snort configuration");

    // Restart Snort to apply the changes
    Command::new("sh")
        .arg("-c")
        .arg("systemctl restart snort")
        .output()
        .expect("Failed to restart Snort");

    println!("Snort configured successfully.");
}