use crate::utils::common::{capitalize, uppercase};



//-----------------------------------------controller--------------------------------------------
pub fn simple_controller_template(name: &str) -> String {
    let cap = capitalize(name);
    format!(
        r#"use axum::{{extract::Path, http::StatusCode, Json}};

// Create a {name}
pub async fn create() -> (StatusCode, String) {{
    (StatusCode::CREATED, "Create {cap}".to_string())
}}

// Get all {name}s
pub async fn find_all() -> (StatusCode, String) {{
    (StatusCode::OK, "Find all {cap}s".to_string())
}}

// Get one {name} by ID
pub async fn find_one(Path(id): Path<u32>) -> (StatusCode, String) {{
    (StatusCode::OK, format!("Find {cap} with id {{}}", id))
}}

// Update a {name} by ID
pub async fn update(Path(id): Path<u32>) -> (StatusCode, String) {{
    (StatusCode::OK, format!("Update {cap} with id {{}}", id))
}}

// Remove a {name} by ID
pub async fn remove(Path(id): Path<u32>) -> (StatusCode, String) {{
    (StatusCode::OK, format!("Remove {cap} with id {{}}", id))
}}
"#,
        name = name,
        cap = cap
    )
}



//-----------------------------------------service--------------------------------------------
pub fn simple_service_template(name: &str) -> String {
    let cap = capitalize(name);
    format!(
        r#"// Service functions for {cap}

pub fn create() -> String {{
    "Create {cap}".to_string()
}}

pub fn find_all() -> String {{
    "Find all {cap}s".to_string()
}}

pub fn find_one(id: u32) -> String {{
    format!("Find {cap} with id {{}}", id)
}}

pub fn update(id: u32) -> String {{
    format!("Update {cap} with id {{}}", id)
}}

pub fn remove(id: u32) -> String {{
    format!("Remove {cap} with id {{}}", id)
}}
"#,
        cap = cap
    )
}

//-----------------------------------------dto--------------------------------------------
pub fn simple_dto_template(name: &str) -> String {
    let cap = capitalize(name);
    format!(
        r#"use serde::Deserialize;

#[derive(Deserialize)]
pub struct Create{cap}Dto {{
    // TODO: define fields
}}

#[derive(Deserialize)]
pub struct Update{cap}Dto {{
    // TODO: define fields
}}
"#,
        cap = cap
    )
}



//-----------------------------------------entity--------------------------------------------
pub fn simple_entity_template(name: &str) -> String {
    let cap = capitalize(name);
    format!(
        r#"use serde::{{Serialize, Deserialize}};

#[derive(Clone, Serialize, Deserialize)]
pub struct {cap} {{
    pub id: u32,
    // TODO: add fields
}}
"#,
        cap = cap
    )
}


//-----------------------------------------middleware--------------------------------------------
pub fn middleware_dependencies_template() -> String {
    format!(
        r#"use axum::extract::Request;
use axum::middleware::Next;
use axum::response::Response;
"#
    )
}

pub fn middleware_function_template(name: &str) -> String {
    format!(
        r#"pub async fn {name}_middleware(req: Request, next: Next) -> Response {{
    println!("➡️ [{name}_middleware] {{}} {{}}", req.method(), req.uri().path());
    let res = next.run(req).await;
    println!("⬅️ [{name}_middleware] {{}}", res.status());
    res
}}
"#,
        name = name
    )
}
