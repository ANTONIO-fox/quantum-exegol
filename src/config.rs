//! Configuration Manager Module - Application configuration

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Docker socket path
    pub docker_socket: String,
    /// Default image to use
    pub default_image: String,
    /// Data directory for Quantum Exegol
    pub data_dir: String,
    /// Auto-update images
    pub auto_update: bool,
    /// Default shell
    pub default_shell: String,
    /// Workspace directory
    pub workspace: String,
    /// GPU support enabled
    pub gpu_enabled: bool,
    /// Network mode
    pub network_mode: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            docker_socket: get_default_docker_socket(),
            default_image: "quantum/security:latest".to_string(),
            data_dir: get_default_data_dir(),
            auto_update: true,
            default_shell: "/bin/bash".to_string(),
            workspace: get_default_workspace(),
            gpu_enabled: false,
            network_mode: "bridge".to_string(),
        }
    }
}

pub struct ConfigManager;

impl ConfigManager {
    /// Get config file path
    fn get_config_path() -> PathBuf {
        let config_dir = dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("quantum-exegol");
        
        fs::create_dir_all(&config_dir).ok();
        config_dir.join("config.json")
    }

    /// Load configuration from file
    pub fn load() -> Config {
        let config_path = Self::get_config_path();
        
        if config_path.exists() {
            match fs::read_to_string(&config_path) {
                Ok(content) => {
                    match serde_json::from_str(&content) {
                        Ok(config) => return config,
                        Err(e) => {
                            eprintln!("Warning: Failed to parse config: {}", e);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Warning: Failed to read config: {}", e);
                }
            }
        }
        
        // Return default config
        Config::default()
    }

    /// Save configuration to file
    pub fn save(config: &Config) -> Result<(), String> {
        let config_path = Self::get_config_path();
        
        let content = serde_json::to_string_pretty(config)
            .map_err(|e| format!("Failed to serialize config: {}", e))?;
        
        fs::write(&config_path, content)
            .map_err(|e| format!("Failed to write config: {}", e))?;
        
        println!("Configuration saved to: {:?}", config_path);
        Ok(())
    }

    /// Initialize default configuration
    pub fn init() -> Result<Config, String> {
        let config = Config::default();
        Self::save(&config)?;
        Ok(config)
    }

    /// Update a specific config value
    pub fn update(key: &str, value: &str) -> Result<(), String> {
        let mut config = Self::load();
        
        match key {
            "docker_socket" => config.docker_socket = value.to_string(),
            "default_image" => config.default_image = value.to_string(),
            "data_dir" => config.data_dir = value.to_string(),
            "auto_update" => config.auto_update = value.parse().unwrap_or(true),
            "default_shell" => config.default_shell = value.to_string(),
            "workspace" => config.workspace = value.to_string(),
            "gpu_enabled" => config.gpu_enabled = value.parse().unwrap_or(false),
            "network_mode" => config.network_mode = value.to_string(),
            _ => return Err(format!("Unknown config key: {}", key)),
        }
        
        Self::save(&config)
    }
}

/// Get default Docker socket path based on OS
fn get_default_docker_socket() -> String {
    if cfg!(target_os = "windows") {
        "npipe:////./pipe/docker_engine".to_string()
    } else {
        "/var/run/docker.sock".to_string()
    }
}

/// Get default data directory
fn get_default_data_dir() -> String {
    dirs::data_local_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("quantum-exegol")
        .to_string_lossy()
        .to_string()
}

/// Get default workspace directory
fn get_default_workspace() -> String {
    dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("quantum-workspace")
        .to_string_lossy()
        .to_string()
}
