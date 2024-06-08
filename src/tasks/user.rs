use std::process::Command;

pub fn manage_users() {
    let output = Command::new("cut")
        .arg("-d:")
        .arg("-f1")
        .arg("/etc/passwd")
        .output()
        .expect("Failed to list users");

    if output.status.success() {
        let users = String::from_utf8_lossy(&output.stdout);
        println!("Users:\n{}", users);
    } else {
        eprintln!("Failed to list users: {}", String::from_utf8_lossy(&output.stderr));
    }
}

pub fn enforce_mfa() {
    println!("Enforcing MFA...");
    // Example implementation to enforce MFA using pam_google_authenticator
    let output = Command::new("sh")
        .arg("-c")
        .arg("apt-get install -y libpam-google-authenticator")
        .output()
        .expect("Failed to install libpam-google-authenticator");
    if output.status.success() {
        println!("libpam-google-authenticator installed successfully.");
    } else {
        eprintln!("Failed to install libpam-google-authenticator: {}", String::from_utf8_lossy(&output.stderr));
        return;
    }

    // Configure PAM to use Google Authenticator
    let pam_content = "auth required pam_google_authenticator.so";
    std::fs::write("/etc/pam.d/common-auth", pam_content)
        .expect("Failed to write to /etc/pam.d/common-auth");

    println!("MFA enforced successfully.");
}