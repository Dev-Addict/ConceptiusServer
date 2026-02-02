mod operational_error;

use axum::{Router, routing::get, serve};
use tokio::net::TcpListener;

use crate::operational_error::OperationalError;

#[tokio::main]
async fn main() {
    let v1 = Router::new().route(
        "/",
        get(|| async { Err::<String, OperationalError>(OperationalError::NotFound) }),
    );
    let api_router = Router::new().nest("/v1", v1);

    let app = Router::new().nest("/api", api_router);

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    serve(listener, app).await.unwrap();
}
