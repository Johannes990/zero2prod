//! src/routes/subscriptions.rs
use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

#[allow(dead_code)]
#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

// simple implementation of subscribe function: we always return 200
pub async fn subscribe(
    form: web::Form<FormData>,
    // retrieving the connection from the application state
    pool: web::Data<PgPool>,
) -> HttpResponse {
    // request id to correlate requests with log messages
    let request_id = Uuid::new_v4();

    // Spans, like logs, have an associated level
    // 'info_span' creates a span at the info-level
    let request_span = tracing::info_span!(
        "Adding a new subscriber.",
        %request_id,
        subscriber_email = %form.email,
        subscriber_name = %form.name
    );

    // Using 'enter' in an async function is a recipe for disaster
    // don't try at home...
    let _request_info_guard = request_span.enter();

    tracing::info!(
        "request_id: {} - Saving new subscriber info to database",
        request_id
    );
    match sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    // We use get_ref() to get an immutable reference to the 'PgConnection'
    // wrapped by 'web::data'
    .execute(pool.get_ref())
    .await
    {
        Ok(_) => {
            tracing::info!(
                "request_id: {} - New subscriber details have been saved in the database",
                request_id
            );
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            tracing::error!(
                "request_id: {} - Failed to execute query: {:?}",
                request_id,
                e
            );
            HttpResponse::InternalServerError().finish()
        }
    }
}
