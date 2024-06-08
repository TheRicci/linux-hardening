use std::process::Command;
use crate::config::config::FirewallRule;

pub fn configure_firewall(firewall_rules: &[FirewallRule]) {
    println!("Configuring firewall...");
    for rule in firewall_rules {
        let command = match rule.action.as_str() {
            "allow" => format!("ufw allow {} {}/{}", rule.port, rule.protocol, rule.port),
            "deny" => format!("ufw deny {} {}/{}", rule.port, rule.protocol, rule.port),
            _ => continue,
        };
        let output = Command::new("sh")
            .arg("-c")
            .arg(&command)
            .output()
            .expect("Failed to configure firewall");
        if output.status.success() {
            println!("Firewall rule applied: {}", command);
        } else {
            eprintln!("Failed to apply firewall rule: {}: {}", command, String::from_utf8_lossy(&output.stderr));
        }
    }
    Command::new("sh")
        .arg("-c")
        .arg("ufw enable")
        .output()
        .expect("Failed to enable UFW");
    println!("Firewall configured successfully.");
}

