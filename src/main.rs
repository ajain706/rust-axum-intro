use axum::{Router, routing::get};

#[tokio::main]
async fn main() {
    let app = Router::new()
    .route("/", get(root))
    .route("/health", get(check_health));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("Server is running on 3000");
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "This is the root"
}

async fn check_health() -> &'static str {
    "Server is working fine"
}
