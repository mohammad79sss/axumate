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
    /// Generate an entity
    Entity { name: String },
    /// Generate a dto
    DTO { name: String },
    /// Generate a module
    Module { name: String },
    /// Generate a module
    Middleware { name: String },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::New { name } => commands::new::create_new_project(name)?,
        Commands::Generate { kind } => match kind {
            GenerateKind::Module { name } => commands::generate_module::generate_module(name)?,
            GenerateKind::Controller { name } => commands::generate_item::generate_controller(name)?,
            GenerateKind::Service { name } => commands::generate_item::generate_service(name)?,
            GenerateKind::Entity { name } => commands::generate_item::generate_entity(name)?,
            GenerateKind::DTO { name } => commands::generate_item::generate_dto(name)?,
            GenerateKind::Middleware { name } => commands::generate_item::generate_middleware(name)?,
        },
    }

    Ok(())
}
