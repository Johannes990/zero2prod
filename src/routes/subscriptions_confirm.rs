//! src/routes/subscriptions_confirm.rs

use actix_web::{web, HttpResponse, ResponseError};
use actix_web::http::StatusCode;
use anyhow::Context;
use sqlx::PgPool;
use uuid::Uuid;
use crate::routes::error_chain_fmt;

// This `Parameters` struct defines all the values we expect to see in
// the incoming request.
#[derive(serde::Deserialize)]
pub struct Parameters {
    subscription_token: String,
}

#[derive(thiserror::Error)]
pub enum SubscriptionConfirmationError {
    #[error(transparent)]
    ConfirmationError(#[from] anyhow::Error),
}

impl std::fmt::Debug for SubscriptionConfirmationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl ResponseError for SubscriptionConfirmationError {
    fn status_code(&self) -> StatusCode {
        match self {
            SubscriptionConfirmationError::ConfirmationError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

#[tracing::instrument(name = "Confirm a pending subscriber", skip(parameters, pool))]
// actix-web will only call handler if the extraction was successful
// it is enough to add parameters to our `confirm()` method parameters
// for actix-web to work.
pub async fn confirm(
    parameters: web::Query<Parameters>,
    pool: web::Data<PgPool>
) -> Result<HttpResponse, SubscriptionConfirmationError> {
    let id = get_subscriber_id_from_token(&pool, &parameters.subscription_token)
        .await
        .context("Failed to retrieve subscriber id from the database using given token.")?;
    confirm_subscriber(&pool, id.unwrap())
        .await
        .context("Failed to set subscription status to `confirmed` in the database.")?;
    Ok(HttpResponse::Ok().finish())
}

#[tracing::instrument(name = "Mark subscriber as confirmed", skip(subscriber_id, pool))]
pub async fn confirm_subscriber(pool: &PgPool, subscriber_id: Uuid) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"UPDATE subscriptions SET status = 'confirmed' WHERE id = $1"#,
        subscriber_id,
    )
    .execute(pool)
    .await?;
    Ok(())
}

#[tracing::instrument(name = "Get subscriber id from token", skip(subscription_token, pool))]
pub async fn get_subscriber_id_from_token(
    pool: &PgPool,
    subscription_token: &str,
) -> Result<Option<Uuid>, sqlx::Error> {
    let result = sqlx::query!(
        "SELECT subscription_id FROM subscription_tokens \
        WHERE subscription_token = $1",
        subscription_token,
    )
    .fetch_optional(pool)
    .await?;
    Ok(result.map(|r| r.subscription_id))
}
