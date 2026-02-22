//! Docker API Module - High-performance Docker integration using bollard
//! This module provides async Docker operations for containers and images

use bollard::container::{
    ListContainersOptions, LogOutput, LogsOptions,
    RemoveContainerOptions, StartContainerOptions, StopContainerOptions,
};
use bollard::image::{CreateImageOptions, ListImagesOptions, RemoveImageOptions};
use bollard::Docker;
use bollard::exec::{CreateExecOptions, StartExecResults};
use bollard::models::Port;
use futures_util::StreamExt;
use tokio::sync::OnceCell;

#[allow(unused_imports)]
use crate::config::ConfigManager;

/// Global Docker connection (singleton)
static DOCKER: OnceCell<Docker> = OnceCell::const_new();

/// Initialize Docker connection
pub async fn get_docker() -> Result<&'static Docker, String> {
    DOCKER.get_or_try_init(|| async {
        let config = ConfigManager::load();
        
        // Connect to Docker daemon
        let docker = Docker::connect_with_local_defaults()
            .map_err(|e| format!("Failed to connect to Docker: {}", e))?;
        
        // Verify connection
        docker.ping().await
            .map_err(|e| format!("Docker ping failed: {}", e))?;
        
        Ok(docker)
    }).await
}

/// List all containers (including stopped)
pub async fn list_containers(all: bool) -> Result<Vec<ContainerInfo>, String> {
    let docker = get_docker().await?;
    
    let options = ListContainersOptions::<String> {
        all,
        ..Default::default()
    };
    
    let containers = docker.list_containers(Some(options))
        .await
        .map_err(|e| format!("Failed to list containers: {}", e))?;
    
    Ok(containers.into_iter().map(|c| ContainerInfo {
        id: c.id.unwrap_or_default(),
        names: c.names.unwrap_or_default(),
        image: c.image.unwrap_or_default(),
        status: c.status.unwrap_or_default(),
        state: c.state.unwrap_or_default(),
        ports: c.ports.unwrap_or_default(),
    }).collect())
}

/// List all images
pub async fn list_images() -> Result<Vec<ImageInfo>, String> {
    let docker = get_docker().await?;
    
    let options = ListImagesOptions::<String> {
        all: false,
        ..Default::default()
    };
    
    let images = docker.list_images(Some(options))
        .await
        .map_err(|e| format!("Failed to list images: {}", e))?;
    
    Ok(images.into_iter().map(|i| ImageInfo {
        id: i.id,
        repo_tags: i.repo_tags,
        size: i.size,
        created: i.created,
    }).collect())
}

/// Pull an image from registry
pub async fn pull_image(name: &str, tag: &str) -> Result<(), String> {
    let docker = get_docker().await?;
    
    let options = CreateImageOptions {
        from_image: name,
        tag,
        ..Default::default()
    };
    
    let mut stream = docker.create_image(Some(options), None, None);
    
    while let Some(result) = stream.next().await {
        match result {
            Ok(info) => {
                if let Some(status) = info.status {
                    println!("  • {}", status);
                }
            }
            Err(e) => return Err(format!("Failed to pull image: {}", e)),
        }
    }
    
    Ok(())
}

/// Start a container
pub async fn start_container(name: &str) -> Result<(), String> {
    let docker = get_docker().await?;
    
    docker.start_container(name, None::<StartContainerOptions<String>>)
        .await
        .map_err(|e| format!("Failed to start container: {}", e))?;
    
    Ok(())
}

/// Stop a container
pub async fn stop_container(name: &str) -> Result<(), String> {
    let docker = get_docker().await?;
    
    docker.stop_container(name, Some(StopContainerOptions {
        t: 10,
    }))
    .await
    .map_err(|e| format!("Failed to stop container: {}", e))?;
    
    Ok(())
}

/// Restart a container
pub async fn restart_container(name: &str) -> Result<(), String> {
    let docker = get_docker().await?;
    
    docker.restart_container(name, Some(bollard::container::RestartContainerOptions {
        t: 10,
    }))
    .await
    .map_err(|e| format!("Failed to restart container: {}", e))?;
    
    Ok(())
}

/// Remove a container
pub async fn remove_container(name: &str, force: bool) -> Result<(), String> {
    let docker = get_docker().await?;
    
    docker.remove_container(name, Some(RemoveContainerOptions {
        force,
        ..Default::default()
    }))
    .await
    .map_err(|e| format!("Failed to remove container: {}", e))?;
    
    Ok(())
}

/// Remove an image
pub async fn remove_image(name: &str, force: bool) -> Result<(), String> {
    let docker = get_docker().await?;
    
    docker.remove_image(name, Some(RemoveImageOptions {
        force,
        ..Default::default()
    }), None)
    .await
    .map_err(|e| format!("Failed to remove image: {}", e))?;
    
    Ok(())
}

/// Execute command in container
pub async fn exec_in_container(name: &str, cmd: &[String]) -> Result<String, String> {
    let docker = get_docker().await?;
    
    // Create exec instance
    let exec = docker.create_exec(name, CreateExecOptions {
        cmd: Some(cmd.to_vec()),
        attach_stdout: Some(true),
        attach_stderr: Some(true),
        ..Default::default()
    })
    .await
    .map_err(|e| format!("Failed to create exec: {}", e))?;
    
    // Start exec and get output
    let output = docker.start_exec(&exec.id, None)
        .await
        .map_err(|e| format!("Failed to start exec: {}", e))?;
    
    let mut output_string = String::new();
    
    match output {
        StartExecResults::Attached { mut output, .. } => {
            while let Some(Ok(msg)) = output.next().await {
                match msg {
                    LogOutput::StdOut { message } => {
                        output_string.push_str(&String::from_utf8_lossy(&message));
                    }
                    LogOutput::StdErr { message } => {
                        output_string.push_str(&String::from_utf8_lossy(&message));
                    }
                    _ => {}
                }
            }
        }
        StartExecResults::Detached => {}
    }
    
    Ok(output_string)
}

/// Get container logs
pub async fn get_container_logs(name: &str, tail: usize) -> Result<String, String> {
    let docker = get_docker().await?;
    
    let options = LogsOptions::<String> {
        stdout: true,
        stderr: true,
        tail: tail.to_string(),
        ..Default::default()
    };
    
    let mut logs = docker.logs(name, Some(options));
    
    let mut output = String::new();
    while let Some(log) = logs.next().await {
        match log {
            Ok(msg) => output.push_str(&msg.to_string()),
            Err(e) => return Err(format!("Failed to read logs: {}", e)),
        }
    }
    
    Ok(output)
}

/// Check if Docker is available
pub async fn check_docker() -> Result<bool, String> {
    match get_docker().await {
        Ok(_) => Ok(true),
        Err(_) => Ok(false),
    }
}

// Data structures for container and image info

#[derive(Debug, Clone)]
pub struct ContainerInfo {
    pub id: String,
    pub names: Vec<String>,
    pub image: String,
    pub status: String,
    pub state: String,
    pub ports: Vec<Port>,
}

#[derive(Debug, Clone)]
pub struct ImageInfo {
    pub id: String,
    pub repo_tags: Vec<String>,
    pub size: i64,
    pub created: i64,
}

/// Format size to human readable
pub fn format_size(bytes: i64) -> String {
    const KB: i64 = 1024;
    const MB: i64 = KB * 1024;
    const GB: i64 = MB * 1024;

    if bytes >= GB {
        format!("{:.1} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.1} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.1} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}
