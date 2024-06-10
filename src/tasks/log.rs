use std::process::Command;
use crate::tasks::checking::ensure_service_installed_and_enabled;

pub fn centralize_logs(log_server: Option<&str>) {
    if let Some(server) = log_server {
        if !server.is_empty() {
            install_log_server();
            println!("Centralizing logs to server: {}", server);
            // Configure rsyslog to send logs to the remote server
            let rsyslog_conf = format!(
                "*.* @@{}:514",
                server
            );
            std::fs::write("/etc/rsyslog.d/50-default.conf", rsyslog_conf)
                .expect("Failed to write rsyslog configuration");

            // Restart rsyslog to apply the changes
            Command::new("sh")
                .arg("-c")
                .arg("systemctl restart rsyslog")
                .output()
                .expect("Failed to restart rsyslog");

            println!("Logs are being centralized to the remote server.");
        } else {
            println!("Log server address is empty, installing local log server...");
            install_log_server();
        }
    } else {
        println!("Log server address not provided, installing local log server...");
        install_log_server();
    }
}

fn install_log_server() {
    if !ensure_service_installed_and_enabled("rsyslog", "rsyslog"){
        return
    }

    // Configure rsyslog as a log server
    let rsyslog_server_conf = r#"
    module(load="imudp")
    input(type="imudp" port="514")

    module(load="imtcp")
    input(type="imtcp" port="514")

    *.* /var/log/remote.log
    "#;
    std::fs::write("/etc/rsyslog.d/49-server.conf", rsyslog_server_conf)
        .expect("Failed to write rsyslog server configuration");

    // Restart rsyslog to apply the changes
    Command::new("sh")
        .arg("-c")
        .arg("systemctl restart rsyslog")
        .output()
        .expect("Failed to restart rsyslog");

    println!("Log server installed and configured.");
}
