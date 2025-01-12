//! src/routes/subscriptions_confirm.rs

use actix_web::{web, HttpResponse};

// This `Parameters` struct defines all the values we expect to see in
// the incoming request.
#[derive(serde::Deserialize)]
#[allow(dead_code)]
pub struct Parameters {
    subscription_token: String,
}

#[tracing::instrument(name = "Confirm a pending subscriber", skip(_parameters))]
// actix-web will only call handler if the extraction was successful
// it is enough to add parameters to our `confirm()` method parameters
// for actix-web to work.
pub async fn confirm(_parameters: web::Query<Parameters>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
