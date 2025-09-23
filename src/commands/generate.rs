use anyhow::Result;
use std::fs;

pub fn generate_controller(name: String) -> Result<()> {
    println!("Generating controller: {}", name);
    let filename = format!("src/{}_controller.rs", name);
    fs::write(
        &filename,
        format!(
            r#"
use axum::routing::get;
use axum::Router;

pub fn {}_controller() -> Router {{
    Router::new().route("/{0}", get(handler))
}}

async fn handler() -> &'static str {{
    "Hello from {} controller!"
}}
"#,
            name, name
        ),
    )?;
    println!("Controller {} created at {}", name, filename);
    Ok(())
}

pub fn generate_service(name: String) -> Result<()> {
    println!("Generating service: {}", name);
    let filename = format!("src/{}_service.rs", name);
    fs::write(
        &filename,
        format!(
            r#"
pub struct {}Service;

impl {}Service {{
    pub fn new() -> Self {{
        Self
    }}

    pub fn do_work(&self) {{
        println!("{} service is working!");
    }}
}}
"#,
            capitalize(&name),
            capitalize(&name),
            name
        ),
    )?;
    println!("Service {} created at {}", name, filename);
    Ok(())
}

fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

/// --------------------------------Tests----------------------------------------


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::env;

    #[test]
    fn test_capitalize() {
        assert_eq!(capitalize("hello"), "Hello");
        assert_eq!(capitalize("h"), "H");
        assert_eq!(capitalize(""), "");
    }

    #[test]
    fn test_generate_controller_creates_file() {
        let dir = env::temp_dir();
        let filename = dir.join("test_controller.rs");
        let filename_str = filename.to_str().unwrap();

        // run
        fs::create_dir_all(dir.clone()).unwrap();
        generate_controller("test".into()).unwrap();

        // check existence
        assert!(filename.exists());

        // clean up
        let _ = fs::remove_file(filename_str);
    }

    #[test]
    fn test_generate_service_creates_file() {
        let dir = env::temp_dir();
        let filename = dir.join("test_service.rs");
        let filename_str = filename.to_str().unwrap();

        // run
        fs::create_dir_all(dir.clone()).unwrap();
        generate_service("test".into()).unwrap();

        // check existence
        assert!(filename.exists());

        // clean up
        let _ = fs::remove_file(filename_str);
    }
}






