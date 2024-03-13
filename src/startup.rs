use tokio::net::TcpListener;

use axum::{
    routing::{get, post},
    serve::Serve,
    Error, Router,
};

use crate::routes::{health_check, subscribe};

pub async fn run(listener: TcpListener) -> Result<Serve<Router, Router>, Error> {
    let app = Router::new()
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscribe));

    let server = axum::serve(listener, app);

    Ok(server)
}
