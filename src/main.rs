use sqlx::PgPool;
use zero2prod::{config::get_configuration, startup::run};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let config = get_configuration().expect("Failed to read configuration file!");
    let connection_pool = PgPool::connect(&config.database.connection_string())
        .await
        .expect("Failed to connect to Postgres");
    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{}", config.application_port))
        .await
        .unwrap();
    match run(listener, connection_pool).await {
        Ok(s) => s.await,
        Err(_) => panic!("Error occured while launching server"),
    }
}
