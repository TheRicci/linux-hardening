use std::process::{Command, Stdio};
use std::io::{BufReader, BufRead};

pub fn run_audit(audit_tool: &str) {
    match audit_tool {
        "Lynis" => {
            println!("Running Lynis audit...");

            // Install Lynis
            let install_output = Command::new("sh")
                .arg("-c")
                .arg("apt-get install -y lynis")
                .output()
                .expect("Failed to run Lynis audit");
            
            if install_output.status.success() {
                println!("Lynis installed successfully.");
            } else {
                eprintln!("Failed to install Lynis: {}", String::from_utf8_lossy(&install_output.stderr));
                return;
            }

            // Run Lynis audit system
            let mut audit_command = Command::new("sh")
                .arg("-c")
                .arg("lynis audit system")
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .expect("Failed to start Lynis audit");

            let stdout = audit_command.stdout.take().expect("Failed to capture stdout");
            let stderr = audit_command.stderr.take().expect("Failed to capture stderr");

            let stdout_reader = BufReader::new(stdout);
            let stderr_reader = BufReader::new(stderr);

            // Read stdout in a new thread
            let stdout_handle = std::thread::spawn(move || {
                for line in stdout_reader.lines() {
                    match line {
                        Ok(line) => println!("{}", line),
                        Err(err) => eprintln!("Error reading stdout: {}", err),
                    }
                }
            });

            // Read stderr in a new thread
            let stderr_handle = std::thread::spawn(move || {
                for line in stderr_reader.lines() {
                    match line {
                        Ok(line) => eprintln!("{}", line),
                        Err(err) => eprintln!("Error reading stderr: {}", err),
                    }
                }
            });

            // Wait for both threads to finish
            let _ = stdout_handle.join();
            let _ = stderr_handle.join();

            // Ensure the audit command completes
            let audit_output = audit_command.wait_with_output().expect("Failed to wait on Lynis audit process");
            if audit_output.status.success() {
                println!("Lynis audit completed successfully.");
            } else {
                eprintln!("Lynis audit process failed.");
            }
        }
        _ => eprintln!("Unsupported audit tool: {}", audit_tool),
    }
}