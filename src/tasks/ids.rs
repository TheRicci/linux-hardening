use std::process::Command;

use crate::tasks::checking::ensure_service_installed_and_enabled;

pub fn deploy_tool(tool_name: &str) {
    match tool_name {
        "Snort" => {
            if !ensure_service_installed_and_enabled("snort", "snort"){
                return;
            }
            configure_snort();
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