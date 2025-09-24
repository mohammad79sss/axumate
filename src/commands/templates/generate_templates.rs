use crate::utils::common::{capitalize, uppercase};

/// Template strings for generated files
pub fn controller_template(name: &str) -> String {
    let cap = capitalize(name);
    format!(
        r#"
use axum::{{
    extract::Path,
    http::StatusCode,
    Json,
}};
use crate::{name}::dto::{name}_dto::{{Create{cap}Dto, Update{cap}Dto}};
use crate::{name}::{name}_service;

// Create a {name}
pub async fn create(Json(dto): Json<Create{cap}Dto>) -> (StatusCode, String) {{
    let message = {name}_service::create(dto);
    (StatusCode::CREATED, message)
}}

// Get all {name}s
pub async fn find_all() -> (StatusCode, String) {{
    let message = {name}_service::find_all();
    (StatusCode::OK, message)
}}

// Get one {name} by ID
pub async fn find_one(Path(id): Path<u32>) -> (StatusCode, String) {{
    let message = {name}_service::find_one(id);
    (StatusCode::OK, message)
}}

// Update a {name} by ID
pub async fn update(Path(id): Path<u32>, Json(dto): Json<Update{cap}Dto>) -> (StatusCode, String) {{
    let message = {name}_service::update(id, dto);
    (StatusCode::OK, message)
}}

// Remove a {name} by ID
pub async fn remove(Path(id): Path<u32>) -> (StatusCode, String) {{
    let message = {name}_service::remove(id);
    (StatusCode::OK, message)
}}
"#,
        name = name,
        cap = cap
    )
}


pub fn service_template(name: &str) -> String {
    let cap = capitalize(name);
    let upp = uppercase(name);
    format!(
        r#"
use crate::{name}::dto::{name}_dto::{{Create{cap}Dto, Update{cap}Dto}};
use crate::{name}::entities::{name}_entity::{cap};
use std::sync::Mutex;
use once_cell::sync::Lazy;

static {upp}S: Lazy<Mutex<Vec<{cap}>>> = Lazy::new(|| Mutex::new(Vec::new()));

pub fn create(dto: Create{cap}Dto) -> String {{
    let mut {name}s = {upp}S.lock().unwrap();
    let id = ({name}s.len() + 1) as u32;
    let {name} = {cap} {{ id, field1: dto.field1, field2: dto.field2 }};
    {name}s.push({name});
    format!("{cap} created with id {{}}", id)
}}

pub fn find_all() -> String {{
    let {name}s = {upp}S.lock().unwrap();
    format!("There are {{}} {name}s in total", {name}s.len())
}}

pub fn find_one(id: u32) -> String {{
    let {name}s = {upp}S.lock().unwrap();
    if {name}s.iter().any(|b| b.id == id) {{
        format!("Found {name} with id {{}}", id)
    }} else {{
        format!("{cap} with id {{}} not found", id)
    }}
}}

pub fn update(id: u32, dto: Update{cap}Dto) -> String {{
    let mut {name}s = {upp}S.lock().unwrap();
    if let Some({name}) = {name}s.iter_mut().find(|b| b.id == id) {{
        if let Some(field1) = dto.field1 {{
            {name}.field1 = field1;
        }}
        if let Some(field2) = dto.field2 {{
            {name}.field2 = field2;
        }}
        return format!("{cap} with id {{}} updated", id);
    }}
    format!("{cap} with id {{}} not found", id)
}}

pub fn remove(id: u32) -> String {{
    let mut {name}s = {upp}S.lock().unwrap();
    let len_before = {name}s.len();
    {name}s.retain(|b| b.id != id);
    if {name}s.len() < len_before {{
        format!("{cap} with id {{}} removed", id)
    }} else {{
        format!("{cap} with id {{}} not found", id)
    }}
}}
"#,
        name = name,
        cap = cap,
        upp = upp
    )
}



pub fn dto_template(name: &str) -> String {
    let cap = capitalize(name);
    format!(
        r#"
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Create{cap}Dto {{
    pub field1: String,
    pub field2: String,
}}

#[derive(Deserialize)]
pub struct Update{cap}Dto {{
    pub field1: Option<String>,
    pub field2: Option<String>,
}}
"#,
        cap = cap
    )
}


pub fn entity_template(name: &str) -> String {
    let cap = capitalize(name);
    format!(
        r#"
use serde::{{Serialize, Deserialize}};

#[derive(Clone, Serialize, Deserialize)]
pub struct {cap} {{
    pub id: u32,
    pub field1: String,
    pub field2: String,
}}
"#,
        cap = cap
    )
}

