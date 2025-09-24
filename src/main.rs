mod commands;
mod utils;
use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::Path;
use std::process::Command;
use std::str;

#[derive(Parser)]
#[command(name = "axumate", version, about = "CLI for Axum project scaffolding")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new Axum project
    New {
        name: String
    },
    /// Generate boilerplate (controller, service, etc.)
    Generate {
        #[command(subcommand)]
        kind: GenerateKind,
    }
}

#[derive(Subcommand)]
enum GenerateKind {
    /// Generate a controller
    Controller { name: String },
    /// Generate a service
    Service { name: String },
    /// Generate a entity
    Entity { name: String },
    /// Generate a dto
    DTO { name: String },
/*    Module { name: String },*/
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::New { name } => commands::new::create_new_project(name)?,
        Commands::Generate { kind } => match kind {
            GenerateKind::Controller { name } => commands::generate::generate_controller(name)?,
            GenerateKind::Service { name } => commands::generate::generate_service(name)?,
            GenerateKind::Entity { name } => commands::generate::generate_entity(name)?,
            GenerateKind::DTO { name } => commands::generate::generate_dto(name)?,
/*            GenerateKind::Module { name } => commands::generate::generate_module(name)?,*/
        },
    }

    Ok(())
}
