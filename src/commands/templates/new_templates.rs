/// Template for the generated main.rs file
pub fn main_template(_name: &str) -> String {
    // you can use `_name` later if you want to customize by project name
    r#"
use axum::{
    routing::get,
    Router
};

#[tokio::main]
async fn main() {
    // Compose the routes
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))

    // Run the app on localhost only
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000") // <- localhost
        .await
        .unwrap();

    println!("🚀 Server running on http://127.0.0.1:3000");

    axum::serve(listener, app).await.unwrap();
}

"#
        .to_string()
}
