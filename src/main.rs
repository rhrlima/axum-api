use std::{error::Error, net::SocketAddr};

use axum::{
    routing::get,
    Router,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));

    let version = 3.0;
	let app = Router::new().route("/", get(move || hello(version)));

    println!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

async fn hello(version: f32) -> String {
    format!("API v{}", version)
}