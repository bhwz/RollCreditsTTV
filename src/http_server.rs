use axum::{Router};
use axum::http::{HeaderValue, Method};
use axum::routing::get;
use tower_http::cors::CorsLayer;
use crate::api::{credits, sys_status};

pub async fn serve_api() {
    println!("Starting API server at http://localhost:8080");
    let app = Router::new()
        .route("/status", get(sys_status))
        .route("/credits", get(credits)).layer(
        CorsLayer::new()
            .allow_origin([
                "https://ttv.greatinter.net".parse::<HeaderValue>().unwrap(),
                "https://ttv-credits.apis.greatinter.net".parse::<HeaderValue>().unwrap()
            ])
            .allow_methods([Method::OPTIONS, Method::GET, Method::POST]),
    );

    let listener = tokio::net::TcpListener::bind(("127.0.0.1", 8080)).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
