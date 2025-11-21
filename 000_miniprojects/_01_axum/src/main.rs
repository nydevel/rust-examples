use axum::{routing::get, Router};

async fn hello_world() -> &'static str {
    "Hello, world!"
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(hello_world));
    axum::serve(TcpListener::bind("127.0.0.1:3000").await.unwrap(), app).await.unwrap();
}
