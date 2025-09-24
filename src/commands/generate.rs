use anyhow::Result;
use std::fs;

use super::templates::generate_templates::{
    controller_template,
    service_template,
    dto_template,
    entity_template,
};

pub fn generate_controller(name: String) -> Result<()> {
    println!("Generating controller: {}", name);
    let filename = format!("src/{}_controller.rs", name);
    fs::write(&filename, controller_template(&name))?;
    println!("Controller {} created at {}", name, filename);
    Ok(())
}

pub fn generate_service(name: String) -> Result<()> {
    println!("Generating service: {}", name);
    let filename = format!("src/{}_service.rs", name);
    fs::write(&filename, service_template(&name))?;
    println!("Service {} created at {}", name, filename);
    Ok(())
}

pub fn generate_dto(name: String) -> Result<()> {
    println!("Generating dto: {}", name);
    let filename = format!("src/{}_dto.rs", name);
    fs::write(&filename, dto_template(&name))?;
    println!("DTO {} created at {}", name, filename);
    Ok(())
}

pub fn generate_entity(name: String) -> Result<()> {
    println!("Generating entity: {}", name);
    let filename = format!("src/{}_entity.rs", name);
    fs::write(&filename, entity_template(&name))?;
    println!("Entity {} created at {}", name, filename);
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
