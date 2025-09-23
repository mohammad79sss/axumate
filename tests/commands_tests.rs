use std::fs;
use std::env;
use axumate::commands::{ generate::{generate_controller, generate_service}};

/// create_new_project actually runs cargo new and cargo add, which will download dependencies — too heavy for a normal unit test.
#[test]
fn integration_generate_controller() {
    let dir = env::temp_dir();
    let file_path = dir.join("my_controller.rs");
    let file_path_str = file_path.to_str().unwrap();

    generate_controller("my".into()).unwrap();

    assert!(file_path.exists());

    // cleanup
    let _ = fs::remove_file(file_path_str);
}

#[test]
fn integration_generate_service() {
    let dir = env::temp_dir();
    let file_path = dir.join("my_service.rs");
    let file_path_str = file_path.to_str().unwrap();

    generate_service("my".into()).unwrap();

    assert!(file_path.exists());

    // cleanup
    let _ = fs::remove_file(file_path_str);
}
