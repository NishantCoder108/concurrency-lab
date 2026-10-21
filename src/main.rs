use axum::{Router, routing::get};
use std::time::Duration;

async fn fast() -> &'static str {
    "Fast response"
}

async fn async_wait() -> &'static str {
    tokio::time::sleep(Duration::from_millis(100)).await;
    "Async wait completed"
}

async fn blocking_wait() -> &'static str {
    // std::thread::sleep(Duration::from_millis(100));
    tokio::task::spawn_blocking(|| {
        std::thread::sleep(Duration::from_millis(100));
    })
    .await
    .unwrap();
    "Blocking wait completed"
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/fast", get(fast))
        .route("/async-wait", get(async_wait))
        .route("/blocking-wait", get(blocking_wait));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("Server listening on http://127.0.0.1:3000");

    axum::serve(listener, app).await.unwrap();
}
