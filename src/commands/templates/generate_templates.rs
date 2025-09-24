use crate::utils::common::capitalize;

/// Template strings for generated files
pub fn controller_template(name: &str) -> String {
    format!(
        r#"
use axum::routing::get;
use axum::Router;

pub fn {name}_controller() -> Router {{
    Router::new().route("/{name}", get(handler))
}}

async fn handler() -> &'static str {{
    "Hello from {name} controller!"
}}
"#,
        name = name
    )
}

pub fn service_template(name: &str) -> String {
    let cap = capitalize(name);
    format!(
        r#"
pub struct {cap}Service;

impl {cap}Service {{
    pub fn new() -> Self {{
        Self
    }}

    pub fn do_work(&self) {{
        println!("{name} service is working!");
    }}
}}
"#,
        cap = cap,
        name = name
    )
}

pub fn dto_template(name: &str) -> String {
    let cap = capitalize(name);
    format!(
        r#"
use serde::Deserialize;

/// For creating a {cap} via API
#[derive(Debug, Deserialize)]
pub struct Create{cap}Dto {{
    pub name: String,
    pub price: f64,
}}

#[derive(Debug, Deserialize)]
pub struct Update{cap}Dto {{
    pub name: Option<String>,
    pub price: Option<f64>,
}}
"#,
        cap = cap
    )
}

pub fn entity_template(name: &str) -> String {
    let cap = capitalize(name);
    format!(
        r#"
use serde::{{Deserialize, Serialize}};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct {cap} {{
    pub id: i64,          // auto-increment PK
    pub name: String,
    pub price: f64,
}}
"#,
        cap = cap
    )
}
