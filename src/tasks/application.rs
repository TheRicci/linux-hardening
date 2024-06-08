use std::process::Command;

pub fn secure_config() {
    println!("Securing application configurations...");

    // Check if nginx is installed
    let nginx_installed = Command::new("sh")
        .arg("-c")
        .arg("which nginx")
        .output()
        .expect("Failed to check nginx installation")
        .status
        .success();

    if nginx_installed {
        println!("Hardening nginx configuration...");
        harden_nginx();
    } else {
        println!("Nginx is not installed.");
    }

    // Check if apache is installed
    let apache_installed = Command::new("sh")
        .arg("-c")
        .arg("which apache2")
        .output()
        .expect("Failed to check apache installation")
        .status
        .success();

    if apache_installed {
        println!("Hardening Apache configuration...");
        harden_apache();
    } else {
        println!("Apache is not installed.");
    }
}

fn harden_nginx() {
    // Example of adding security headers to nginx configuration
    let nginx_security_config = r#"
    server {
        listen 80 default_server;
        listen [::]:80 default_server;

        server_name _;

        add_header X-Content-Type-Options nosniff;
        add_header X-XSS-Protection "1; mode=block";
        add_header X-Frame-Options SAMEORIGIN;
        add_header Content-Security-Policy "default-src 'self'";
    }
    "#;

    std::fs::write("/etc/nginx/conf.d/security.conf", nginx_security_config)
        .expect("Failed to write nginx security configuration");

    // Restart nginx to apply the changes
    Command::new("sh")
        .arg("-c")
        .arg("systemctl restart nginx")
        .output()
        .expect("Failed to restart nginx");

    println!("Nginx configuration hardened.");
}

fn harden_apache() {
    // Example of adding security headers to Apache configuration
    let apache_security_config = r#"
    <IfModule mod_headers.c>
        Header set X-Content-Type-Options "nosniff"
        Header set X-XSS-Protection "1; mode=block"
        Header set X-Frame-Options "SAMEORIGIN"
        Header set Content-Security-Policy "default-src 'self'"
    </IfModule>
    "#;

    std::fs::write("/etc/apache2/conf-available/security.conf", apache_security_config)
        .expect("Failed to write Apache security configuration");

    // Enable the security configuration
    Command::new("sh")
        .arg("-c")
        .arg("a2enconf security")
        .output()
        .expect("Failed to enable Apache security configuration");

    // Restart Apache to apply the changes
    Command::new("sh")
        .arg("-c")
        .arg("systemctl restart apache2")
        .output()
        .expect("Failed to restart Apache");

    println!("Apache configuration hardened.");
}
