//! src/routes/subscriptions.rs
use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

#[allow(dead_code)]
#[allow(unused_imports)]
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
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            println!("Failed to execute query: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
