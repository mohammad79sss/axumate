use anyhow::Result;
use std::fs;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use crate::utils::file::*;

use super::templates::generate_module_templates::{
    controller_template,
    dto_template,
    entity_template,
    service_template,
};

/// Generate controller inside src/{module}/{module}_controller.rs and ensure mod declarations.
pub fn generate_controller(name: String) -> Result<()> {
    println!("Generating controller: {}", name);

    // 1️⃣ ensure module dir and mod.rs
    let mdir = ensure_dir(&module_dir(&name))?;
    let mod_rs = ensure_mod_rs(&mdir)?;

    // 2️⃣ ensure module is published at crate root
    ensure_root_mod(&name)?;

    // 3️⃣ create controller file path ({module}_controller.rs)
    let filename = mdir.join(format!("{}_controller.rs", name));

    // write controller content
    fs::write(&filename, controller_template(&name))?;

    // 4️⃣ ensure `pub mod {module}_controller;` is present in src/{module}/mod.rs
    let mod_name = format!("{}_controller", name);
    ensure_pub_mod_decl(&mod_rs, &mod_name)?;

    // 5️⃣ Add routes() function to mod.rs if not present
    let mut mod_content = String::new();
    fs::File::open(&mod_rs)?.read_to_string(&mut mod_content)?;

    if !mod_content.contains("pub fn routes() -> Router") {
        let routes_fn = format!(
            r#"
use axum::Router;
use axum::routing::{{get, post, patch, delete}};

pub fn routes() -> Router {{
    Router::new()
        .route("/", post({mod_name}::create).get({mod_name}::find_all))
        .route("/{{id}}", get({mod_name}::find_one)
            .patch({mod_name}::update)
            .delete({mod_name}::remove))
}}
"#,
            mod_name = mod_name
        );

        // append the routes function at the end of mod.rs
        mod_content.push_str(&routes_fn);
        fs::File::create(&mod_rs)?.write_all(mod_content.as_bytes())?;
    }

    println!(
        "Controller {} created and routes() added in {}",
        name,
        filename.to_string_lossy()
    );

    Ok(())
}

/// Generate service inside src/{module}/{module}_service.rs and ensure mod declarations.
pub fn generate_service(name: String) -> Result<()> {
    println!("Generating service: {}", name);

    // ensure module dir and mod.rs
    let mdir = ensure_dir(&module_dir(&name))?;
    let mod_rs = ensure_mod_rs(&mdir)?;

    // ensure root has this module
    ensure_root_mod(&name)?;

    // create service file path ({module}_service.rs)
    let filename = mdir.join(format!("{}_service.rs", name));

    fs::write(&filename, service_template(&name))?;

    // ensure `pub mod {module}_service;` in src/{module}/mod.rs
    let mod_name = format!("{}_service", name);
    ensure_pub_mod_decl(&mod_rs, &mod_name)?;

    println!("Service {} created at {}", name, filename.to_string_lossy());
    Ok(())
}

/// Generate DTO file inside src/{module}/dto/{module}_dto.rs.
/// Ensure src/{module}/dto/mod.rs and src/{module}/mod.rs include proper declarations.
pub fn generate_dto(name: String) -> Result<()> {
    println!("Generating dto: {}", name);

    // module dir
    let mdir = ensure_dir(&module_dir(&name))?;
    let mod_rs = ensure_mod_rs(&mdir)?;

    // ensure module declared at crate root
    ensure_root_mod(&name)?;

    // ensure dto subdir and mod.rs
    let dto_dir = ensure_dir(&mdir.join("dto"))?;
    let dto_mod_rs = ensure_mod_rs(&dto_dir)?;

    // file name: {module}_dto.rs
    let dto_filename = format!("{}_dto.rs", name);
    let full_path = dto_dir.join(&dto_filename);

    fs::write(&full_path, dto_template(&name))?;

    // ensure dto mod is included in module mod.rs: `pub mod dto;`
    ensure_pub_mod_decl(&mod_rs, "dto")?;

    // ensure the specific dto file is declared in dto/mod.rs: `pub mod {module}_dto;`
    let dto_child = format!("{}_dto", name);
    ensure_pub_mod_decl(&dto_mod_rs, &dto_child)?;

    println!("DTO {} created at {}", name, full_path.to_string_lossy());
    Ok(())
}

/// Generate entity file inside src/{module}/entities/{module}_entity.rs.
/// Ensure src/{module}/entities/mod.rs and src/{module}/mod.rs include proper declarations.
pub fn generate_entity(name: String) -> Result<()> {
    println!("Generating entity: {}", name);

    // module dir
    let mdir = ensure_dir(&module_dir(&name))?;
    let mod_rs = ensure_mod_rs(&mdir)?;

    // ensure module declared at crate root
    ensure_root_mod(&name)?;

    // ensure entities subdir and mod.rs
    let ent_dir = ensure_dir(&mdir.join("entities"))?;
    let ent_mod_rs = ensure_mod_rs(&ent_dir)?;

    // file name: {module}_entity.rs
    let ent_filename = format!("{}_entity.rs", name);
    let full_path = ent_dir.join(&ent_filename);

    fs::write(&full_path, entity_template(&name))?;

    // ensure entities mod is included in module mod.rs: `pub mod entities;`
    ensure_pub_mod_decl(&mod_rs, "entities")?;

    // ensure the specific entity file is declared in entities/mod.rs: `pub mod {module}_entity;`
    let ent_child = format!("{}_entity", name);
    ensure_pub_mod_decl(&ent_mod_rs, &ent_child)?;

    println!("Entity {} created at {}", name, full_path.to_string_lossy());
    Ok(())
}


pub fn generate_module(name: String) -> Result<()> {
    println!("Generating module: {}", name);

    // 1️⃣ Generate all module components first
    generate_service(name.clone())?;
    generate_dto(name.clone())?;
    generate_entity(name.clone())?;
    generate_controller(name.clone())?;

    // 2️⃣ Update main.rs
    let main_path = Path::new("src/main.rs");
    let mut content = String::new();
    fs::File::open(&main_path)?.read_to_string(&mut content)?;

    // --- Insert `mod {name};` after last `use ...;` ---
    let mod_decl = format!("mod {};", name);
    if !content.contains(&mod_decl) {
        if let Some(pos) = content.rfind("use ") {
            if let Some(end) = content[pos..].find(';') {
                let insert_pos = pos + end + 1;
                content.insert_str(insert_pos, &format!("\n{}", mod_decl));
            }
        } else {
            content = format!("{}\n{}", mod_decl, content);
        }
    }

    // --- Insert `.nest("/name", name::routes())` inside Router::new() chain ---
    let nest_line = format!("        .nest(\"/{0}\", {0}::routes())", name);
    if !content.contains(&nest_line) {
        if let Some(pos) = content.find("Router::new()") {
            if let Some(chain_end) = content[pos..].find(';') {
                let insert_pos = pos + chain_end;
                content.insert_str(insert_pos, &format!("\n{}", nest_line));
            }
        }
    }

    // Write back the updated main.rs
    fs::File::create(&main_path)?.write_all(content.as_bytes())?;

    println!("main.rs updated with module `{}`", name);
    println!("Module {} generated successfully!", name);

    Ok(())
}


//===================================tests==========================================

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::env;

    #[test]
    fn test_generate_controller_creates_file() {
        let dir = env::temp_dir();
        let filename = dir.join("test_controller.rs");
        let filename_str = filename.to_str().unwrap();

        generate_controller("test".into()).unwrap();

        assert!(filename.exists());

        let _ = fs::remove_file(filename_str);
    }

    #[test]
    fn test_generate_service_creates_file() {
        let dir = env::temp_dir();
        let filename = dir.join("test_service.rs");
        let filename_str = filename.to_str().unwrap();

        generate_service("test".into()).unwrap();

        assert!(filename.exists());

        let _ = fs::remove_file(filename_str);
    }

    #[test]
    fn test_generate_dto_creates_file() {
        let dir = env::temp_dir();
        let filename = dir.join("test_dto.rs");
        let filename_str = filename.to_str().unwrap();
        generate_dto("test".into()).unwrap();
        assert!(filename.exists());
        let _ = fs::remove_file(filename_str);
    }

    #[test]
    fn test_generate_entity_creates_file() {
        let dir = env::temp_dir();
        let filename = dir.join("test_entity.rs");
        let filename_str = filename.to_str().unwrap();
        generate_entity("test".into()).unwrap();
        assert!(filename.exists());
        let _ = fs::remove_file(filename_str);
    }
}
