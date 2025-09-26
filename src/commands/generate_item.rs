use anyhow::Result;
use std::fs;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use crate::utils::file::*;

use super::templates::generate_item_templates::{
    simple_controller_template,
    simple_dto_template,
    simple_entity_template,
    simple_service_template,
    middleware_function_template,
    middleware_dependencies_template
};

/// Generate a standalone controller in {cwd}/{name}/{name}_controller.rs
pub fn generate_controller(name: String) -> Result<()> {
    println!("Generating simple controller: {}", name);

    // ensure target module dir
    let mdir = ensure_dir(&Path::new(&name))?;
    let mod_rs = ensure_mod_rs(&mdir)?;

    // file path
    let filename = mdir.join(format!("{}_controller.rs", name));
    fs::write(&filename, simple_controller_template(&name))?;

    // declare controller in that module's mod.rs
    ensure_pub_mod_decl(&mod_rs, &format!("{}_controller", name))?;

    // also declare the module itself in parent mod.rs (or lib.rs if at root)
    let parent_mod = current_mod_file();
    ensure_pub_mod_decl(&parent_mod, &name)?;

    println!("Controller {} created at {}", name, filename.to_string_lossy());
    Ok(())
}

/// Generate a standalone service in {cwd}/{name}/{name}_service.rs
pub fn generate_service(name: String) -> Result<()> {
    println!("Generating simple service: {}", name);

    let mdir = ensure_dir(&Path::new(&name))?;
    let mod_rs = ensure_mod_rs(&mdir)?;

    let filename = mdir.join(format!("{}_service.rs", name));
    fs::write(&filename, simple_service_template(&name))?;

    ensure_pub_mod_decl(&mod_rs, &format!("{}_service", name))?;

    let parent_mod = current_mod_file();
    ensure_pub_mod_decl(&parent_mod, &name)?;

    println!("Service {} created at {}", name, filename.to_string_lossy());
    Ok(())
}

/// Generate a standalone DTO in {cwd}/{name}/{name}_dto.rs
pub fn generate_dto(name: String) -> Result<()> {
    println!("Generating simple dto: {}", name);

    let mdir = ensure_dir(&Path::new(&name))?;
    let mod_rs = ensure_mod_rs(&mdir)?;

    let filename = mdir.join(format!("{}_dto.rs", name));
    fs::write(&filename, simple_dto_template(&name))?;

    ensure_pub_mod_decl(&mod_rs, &format!("{}_dto", name))?;

    let parent_mod = current_mod_file();
    ensure_pub_mod_decl(&parent_mod, &name)?;

    println!("DTO {} created at {}", name, filename.to_string_lossy());
    Ok(())
}

/// Generate a standalone entity in {cwd}/{name}/{name}_entity.rs
pub fn generate_entity(name: String) -> Result<()> {
    println!("Generating simple entity: {}", name);

    let mdir = ensure_dir(&Path::new(&name))?;
    let mod_rs = ensure_mod_rs(&mdir)?;

    let filename = mdir.join(format!("{}_entity.rs", name));
    fs::write(&filename, simple_entity_template(&name))?;

    ensure_pub_mod_decl(&mod_rs, &format!("{}_entity", name))?;

    let parent_mod = current_mod_file();
    ensure_pub_mod_decl(&parent_mod, &name)?;

    println!("Entity {} created at {}", name, filename.to_string_lossy());
    Ok(())
}

pub fn generate_middleware(name: String) -> anyhow::Result<()> {
    println!("Generating middleware: {}", name);

    // --- Step 1: Ensure `src/middlewares.rs` exists and has the function ---
    let middlewares_path = Path::new("src/middlewares.rs");

    if middlewares_path.exists() {
        let mut content = fs::read_to_string(&middlewares_path)?;
        let func_identifier = format!("pub async fn {}_middleware", name);
        if !content.contains(&func_identifier) {
            content.push_str(&middleware_function_template(&name));
            fs::write(&middlewares_path, content)?;
            println!("Added `{}` to middlewares.rs", func_identifier);
        } else {
            println!("Middleware `{}` already exists in middlewares.rs", name);
        }
    } else {
        let mut content = String::new();
        content.push_str(&middleware_dependencies_template());
        content.push_str("\n");
        content.push_str(&middleware_function_template(&name));
        fs::write(&middlewares_path, content)?;
        println!("Created middlewares.rs with `{}`", name);
    }

    // --- Step 2: Update main.rs ---
    let main_path = Path::new("src/main.rs");
    let mut content = String::new();
    fs::File::open(&main_path)?.read_to_string(&mut content)?;

    // Insert `mod middlewares;` if missing
    let mod_decl = "mod middlewares;";
    if !content.contains(mod_decl) {
        if let Some(pos) = content.rfind("use ") {
            if let Some(end) = content[pos..].find(';') {
                let insert_pos = pos + end + 1;
                content.insert_str(insert_pos, &format!("\n{}", mod_decl));
            }
        } else {
            content = format!("{}\n{}", mod_decl, content);
        }
    }

    // Insert `.layer(...)` **right after `Router::new()`**
    let layer_line = format!(
        "    .layer(middleware::from_fn(middlewares::{}_middleware))",
        name
    );
    if !content.contains(&layer_line) {
        if let Some(pos) = content.find("Router::new()") {
            let insert_pos = pos + "Router::new()".len();
            content.insert_str(insert_pos, &format!("\n{}", layer_line));
        }
    }

    fs::write(&main_path, content)?;

    println!("main.rs updated with middleware `{}` at the top of Router::new()", name);
    println!("Middleware `{}` generated successfully!", name);

    Ok(())
}
