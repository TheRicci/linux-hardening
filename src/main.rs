use clap::{Parser, Subcommand};
use log::info;

mod tasks;
mod config;
mod utils;

#[derive(Parser, Debug)]
#[command(name = "Hardener")]
#[command(version = "0.1.0")]
#[command(author = "Your Name <you@example.com>")]
#[command(about = "Linux Hardening Tool", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    List,
    Disable {
        #[arg(short, long)]
        service_name: String,
    },
}

fn main() {
    utils::logging::init_logging();
    info!("Starting the Hardener tool");

    let config_path = "config.yaml";
    let config = match config::config::Config::load_from_file(config_path) {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("Error loading config: {}", e);
            std::process::exit(1);
        }
    };

    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::List => tasks::service::list_services(),
            Commands::Disable { service_name } => {
                tasks::service::disable_service(&service_name)
            }
        }
        
    } else {
        // Automatic mode if no command is passed
        automatic_mode(&config);
    }
}

fn automatic_mode(config: &config::config::Config) {
    println!("Starting automatic hardening based on the configuration file...");

    // Update
    if config.update.auto_update {
        tasks::update::system_update(&config.update.update_interval);
    }

    // User
    if config.user.enforce_mfa {
        tasks::user::enforce_mfa();
    }

    // Service
    if config.service.list_services {
        tasks::service::list_services();
    }

    for service in &config.service.disable_services {
        tasks::service::disable_service(service);
    }

    // Network
    if config.network.configure_firewall {
        tasks::network::configure_firewall(&config.network.firewall_rules);
    }

    // Log
    if config.log.centralize_logs {
        tasks::log::centralize_logs(Some(&config.log.log_server.as_ref().unwrap()));
    }

    // Kernel
    if config.kernel.harden_kernel {
        println!("Hardening kernel...");
        tasks::kernel::harden_kernel(&config.kernel.parameters);
    }

    // Application
    if config.application.secure_config {
        println!("Securing application configs...");
        tasks::application::secure_config()
    }

    // IDS
    if config.ids.deploy_tool {
        println!("Deploying IDS tool: {}", config.ids.tool_name);
        tasks::ids::deploy_tool(&config.ids.tool_name);
    }

    // Backup
    if config.backup.schedule_backups {
        println!("Scheduling backups...");
        tasks::backup::schedule_backups(&config.backup.backup_paths);
    }

    if config.backup.test_recovery {
        println!("Testing backup recovery...");
        tasks::backup::test_recovery(&config.backup.backup_paths);
    }

    // Audit
    if config.audit.run_audit {
        println!("Running audit with tool: {}", config.audit.audit_tool);
        tasks::audit::run_audit(&config.audit.audit_tool);
    }

    println!("Automatic hardening complete.");
}