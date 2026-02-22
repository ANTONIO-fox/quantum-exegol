//! Quantum Exegol - Environmental Cybersecurity Framework
//! A Rust-based alternative to Exegol for offensive security operations

use clap::{Parser, Subcommand};
use colored::*;
use std::process::exit;

mod cli;
mod container;
mod image;
mod config;
mod manager;
mod utils;
mod docker;

#[derive(Parser)]
#[command(name = "quantum-exegol")]
#[command(about = "Environmental Cybersecurity Framework - Rust alternative to Exegol", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[arg(short, long, default_value = "false")]
    verbose: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Install a new security environment image
    Install {
        #[arg(short, long)]
        name: Option<String>,

        #[arg(short, long)]
        tag: Option<String>,
    },

    /// Start a container with selected environment
    Start {
        #[arg(short, long)]
        name: Option<String>,

        #[arg(short, long)]
        image: Option<String>,
    },

    /// Stop a running container
    Stop {
        #[arg(short, long)]
        name: Option<String>,
    },

    /// Execute command in running container
    Exec {
        #[arg(short, long)]
        name: Option<String>,

        #[arg(last = true)]
        command: Vec<String>,
    },

    /// List available images
    Images,

    /// List running containers
    Ps,

    /// Remove a container
    Remove {
        #[arg(short, long)]
        name: String,
    },

    /// Update images and wrapper
    Update {
        #[arg(short, long)]
        image: Option<String>,
    },

    /// Build custom image
    Build {
        #[arg(short, long)]
        dockerfile: Option<String>,
    },

    /// Display version information
    Version,

    /// Configure Quantum Exegol
    Config,

    /// Restart a container
    Restart {
        #[arg(short, long)]
        name: Option<String>,
    },

    /// Uninstall an image
    Uninstall {
        #[arg(short, long)]
        name: Option<String>,
    },

    /// Activate license
    Activate {
        #[arg(short, long)]
        key: Option<String>,
    },
}

fn main() {
    // Initialize logger
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .format_timestamp_millis()
        .init();

    let cli = Cli::parse();

    // Execute command
    let result = match cli.command {
        Commands::Install { name, tag } => cli::install(name, tag),
        Commands::Start { name, image } => cli::start(name, image),
        Commands::Stop { name } => cli::stop(name),
        Commands::Exec { name, command } => cli::exec(name, command),
        Commands::Images => cli::list_images(),
        Commands::Ps => cli::list_containers(),
        Commands::Remove { name } => cli::remove_container(name),
        Commands::Update { image } => cli::update(image),
        Commands::Build { dockerfile } => cli::build(dockerfile),
        Commands::Version => cli::version(),
        Commands::Config => cli::config(),
        Commands::Restart { name } => cli::restart(name),
        Commands::Uninstall { name } => cli::uninstall(name),
        Commands::Activate { key } => cli::activate(key),
    };

    match result {
        Ok(_) => {
            println!("\n{}", "✓ Opération réussie".green());
            exit(0);
        }
        Err(e) => {
            eprintln!("\n{}: {}", "Erreur".red(), e);
            exit(1);
        }
    }
}

fn get_help_text() -> String {
    format!(
        r#"
{}
    
{}

Usage: quantum-exegol <COMMAND>

Commands:
  install    Install a new security environment image
  start      Start a container with selected environment
  stop       Stop a running container
  exec       Execute command in running container
  images     List available images
  ps         List running containers
  remove     Remove a container
  update     Update images and wrapper
  build      Build custom image
  config     Configure Quantum Exegol
  version    Display version information
  help       Display this help information

Options:
  -v, --verbose  Enable verbose output
  -h, --help     Display help information

{}
"#,
        "╔═══════════════════════════════════════════════════════════════╗".cyan(),
        "║      QUANTUM EXEGOL - Cybersecurity Environment Framework     ║".cyan(),
        "╚═══════════════════════════════════════════════════════════════╝".cyan()
    )
}
