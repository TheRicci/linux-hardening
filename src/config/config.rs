use serde::Deserialize;
use std::error::Error;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub update: UpdateConfig,
    pub user: UserConfig,
    pub service: ServiceConfig,
    pub network: NetworkConfig,
    pub log: LogConfig,
    pub kernel: KernelConfig,
    pub application: ApplicationConfig,
    pub ids: IdsConfig,
    pub backup: BackupConfig,
    pub audit: AuditConfig,
}

#[derive(Debug, Deserialize)]
pub struct UpdateConfig {
    pub auto_update: bool,
    pub repositories: Vec<Repository>,
}

#[derive(Debug, Deserialize)]
pub struct Repository {
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct UserConfig {
    pub enforce_mfa: bool,
}

#[derive(Debug, Deserialize)]
pub struct ServiceConfig {
    pub list_services: bool,
    pub disable_services: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct NetworkConfig {
    pub configure_firewall: bool,
    pub firewall_rules: Vec<FirewallRule>,
}

#[derive(Debug, Deserialize)]
pub struct FirewallRule {
    pub action: String,
    pub port: u16,
    pub protocol: String,
}

#[derive(Debug, Deserialize)]
pub struct LogConfig {
    pub enable_logging: bool,
    pub centralize_logs: bool,
    pub log_server: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct KernelConfig {
    pub harden_kernel: bool,
    pub parameters: Vec<KernelParameter>,
}

#[derive(Debug, Deserialize)]
pub struct KernelParameter {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Deserialize)]
pub struct ApplicationConfig {
    pub secure_config: bool,
}

#[derive(Debug, Deserialize)]
pub struct IdsConfig {
    pub deploy_tool: bool,
    pub tool_name: String,
}

#[derive(Debug, Deserialize)]
pub struct BackupConfig {
    pub schedule_backups: bool,
    pub backup_paths: Vec<String>,
    pub test_recovery: bool,
}

#[derive(Debug, Deserialize)]
pub struct AuditConfig {
    pub run_audit: bool,
    pub audit_tool: String,
}

impl Config {
    pub fn load_from_file(path: &str) -> Result<Self, Box<dyn Error>> {
        let config_content = fs::read_to_string(path)?;
        let config: Config = serde_yaml::from_str(&config_content)?;
        Ok(config)
    }
}