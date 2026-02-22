//! Utilities Module - Common utility functions

use std::path::PathBuf;

/// Get the application data directory
pub fn get_app_data_dir() -> PathBuf {
    dirs::data_local_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("quantum-exegol")
}

/// Get the logs directory
pub fn get_logs_dir() -> PathBuf {
    get_app_data_dir().join("logs")
}

/// Get the workspace directory
pub fn get_workspace_dir() -> PathBuf {
    dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("quantum-workspace")
}

/// Format timestamp to readable string
pub fn format_timestamp(timestamp: &str) -> String {
    // Simple implementation - could be enhanced with chrono
    timestamp.replace("T", " ").replace("Z", "")
}

/// Validate container name
pub fn validate_container_name(name: &str) -> Result<(), String> {
    if name.is_empty() {
        return Err("Container name cannot be empty".to_string());
    }
    
    if name.len() > 64 {
        return Err("Container name too long (max 64 characters)".to_string());
    }
    
    // Check for valid characters
    let valid_chars = name.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_');
    if !valid_chars {
        return Err("Container name contains invalid characters".to_string());
    }
    
    Ok(())
}

/// Validate image name
pub fn validate_image_name(name: &str) -> Result<(), String> {
    if name.is_empty() {
        return Err("Image name cannot be empty".to_string());
    }
    
    // Basic validation for image name format
    if !name.contains('/') && !name.contains(':') {
        return Err("Invalid image name format".to_string());
    }
    
    Ok(())
}

/// Print a banner
pub fn print_banner() {
    println!(r#"
    ╔═══════════════════════════════════════════════════════════════╗
    ║                                                               ║
    ║     ██████╗ ███████╗███████╗██╗     ██╗███╗   ██╗███████╗    ║
    ║     ██╔══██╗██╔════╝██╔════╝██║     ██║████╗  ██║██╔════╝    ║
    ║     ██║  ██║█████╗  █████╗  ██║     ██║██╔██╗ ██║█████╗      ║
    ║     ██║  ██║██╔══╝  ██╔══╝  ██║     ██║██║╚██╗██║██╔══╝      ║
    ║     ██████╔╝███████╗███████╗███████╗██║██║ ╚████║███████╗    ║
    ║     ╚═════╝ ╚══════╝╚══════╝╚══════╝╚═╝╚═╝  ╚═══╝╚══════╝    ║
    ║                                                               ║
    ║           Environmental Cybersecurity Framework               ║
    ║                 Rust-based Alternative to Exegol            ║
    ║                                                               ║
    ╚═══════════════════════════════════════════════════════════════╝
    "#);
}
