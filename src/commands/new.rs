use anyhow::Result;
use std::process::Command;
use std::path::Path;

/// create_new_project actually runs cargo new and cargo add, which will download dependencies — too heavy for a normal unit test.
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
    for dep in &["axum", "tokio@1 --features full"] {
        let status = Command::new("cargo")
            .current_dir(project_dir)
            .arg("add")
            .args(dep.split_whitespace())
            .status()?;
        if !status.success() {
            anyhow::bail!("Failed to add dependency: {}", dep);
        }
    }

    // Step 3: Replace main.rs
    let main_rs_path = project_dir.join("src/main.rs");
    std::fs::write(
        main_rs_path,
        r#"
use axum::{Router, routing::get};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(|| async { "Hello, Axum!" }));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
"#,
    )?;

    println!("Axum project '{}' created successfully!", name);
    Ok(())
}
