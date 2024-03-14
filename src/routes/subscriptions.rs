use std::sync::Arc;

use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Form,
};
use chrono::Utc;
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct FormData {
    pub email: String,
    pub name: String,
}

pub async fn subscribe(State(state): State<Arc<PgPool>>, Form(form): Form<FormData>) -> Response {
    match sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at) VALUES ($1,$2,$3,$4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(state.as_ref())
    .await
    {
        Ok(_) => StatusCode::OK.into_response(),
        Err(_) => {
            println!("Failed to insert data into subscriptions table");
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}
