use actix_web::{web, HttpResponse};

#[allow(dead_code)]
#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

// simple implementation of subscribe function: we always return 200
pub async fn subscribe(_form: web::Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
