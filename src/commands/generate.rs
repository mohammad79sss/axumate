use anyhow::Result;
use std::fs;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use crate::utils::file::*;

use super::templates::generate_templates::{
    controller_template, dto_template, entity_template, service_template,
};

/// Generate controller inside src/{module}/{module}_controller.rs and ensure mod declarations.
pub fn generate_controller(name: String) -> Result<()> {
    println!("Generating controller: {}", name);

    // 1. ensure module dir and mod.rs
    let mdir = ensure_dir(&module_dir(&name))?;
    let mod_rs = ensure_mod_rs(&mdir)?;

    // 2. ensure module is published at crate root
    ensure_root_mod(&name)?;

    // 3. create controller file path ({module}_controller.rs)
    let filename = mdir.join(format!("{}_controller.rs", name));

    // write content
    fs::write(&filename, controller_template(&name))?;

    // 4. ensure `pub mod {module}_controller;` is present in src/{module}/mod.rs
    let mod_name = format!("{}_controller", name);
    ensure_pub_mod_decl(&mod_rs, &mod_name)?;

    println!(
        "Controller {} created at {}",
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


/*pub fn generate_module(name: String) -> Result<()> {
    println!("Generating module: {}", name);
}*/

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
