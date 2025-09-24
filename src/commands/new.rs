use anyhow::Result;
use std::process::Command;
use std::path::Path;

use super::templates::new_templates::main_template;

pub fn create_new_project(name: String) -> Result<()> {
    println!("Creating new Cargo project: {}", name);

    // Step 1: Run `cargo new <name>`
    let status = Command::new("cargo")
        .arg("new")
        .arg(&name)
        .status()?;
    if !status.success() {
        anyhow::bail!("Failed to create new Cargo project");
    }

    let project_dir = Path::new(&name);

    // Step 2: Add dependencies using `cargo add`
    for dep in &["axum", "tokio@1 --features full", "once_cell", "serde --features derive"] {
        let status = Command::new("cargo")
            .current_dir(project_dir)
            .arg("add")
            .args(dep.split_whitespace())
            .status()?;
        if !status.success() {
            anyhow::bail!("Failed to add dependency: {}", dep);
        }
    }

    // Step 3: Replace main.rs with our template
    let main_rs_path = project_dir.join("src/main.rs");
    std::fs::write(main_rs_path, main_template(&name))?;

    println!("Axum project '{}' created successfully!", name);
    Ok(())
}
