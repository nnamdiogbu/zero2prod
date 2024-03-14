use std::sync::Arc;

use sqlx::PgPool;
use tokio::net::TcpListener;

use axum::{
    routing::{get, post},
    serve::Serve,
    Error, Router,
};

use crate::routes::{health_check, subscribe};

pub async fn run(
    listener: TcpListener,
    connection_pool: PgPool,
) -> Result<Serve<Router, Router>, Error> {
    let connection_pool = Arc::new(connection_pool);

    let app = Router::new()
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscribe))
        .with_state(connection_pool.clone());

    let server = axum::serve(listener, app);

    Ok(server)
}
