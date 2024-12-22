//! src/routes/subscriptions.rs
use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use tracing::Instrument;
use uuid::Uuid;

#[allow(dead_code)]
#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(
    form: web::Form<FormData>,
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

    // we don't call enter on query_span, instrument takes
    // care of that for us in the right moments in the futures lifetime
    let query_span = tracing::info_span!("Saving new subscriber details in the database");

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
    .instrument(query_span)
    // first we attach the instrumentation, then we `await` it
    .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            // this currently falls outside of `query_span`
            // we'll rectify it later
            tracing::error!("Failed to execute query: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
