mod handler;
mod routes;
mod models;

use routes::create_router;

#[tokio::main]
async fn main() {
    let app = create_router();

    println!("🚀 Server started successfully");
    let listener = tokio::net::TcpListener::bind("127.0.0.1:5000").await.unwrap();
    axum::serve(listener, app.into_make_service()).await.unwrap();
}