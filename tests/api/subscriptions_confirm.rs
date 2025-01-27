//! tests/api/subscriptions_confirm.rs

use crate::helpers::spawn_app;
use wiremock::matchers::{method, path};
use wiremock::{Mock, ResponseTemplate};

#[tokio::test]
async fn confirmations_without_token_are_rejected_with_a_400() {
    // Arrange
    let test_app = spawn_app().await;

    // Act
    let response = reqwest::get(&format!("{}/subscriptions/confirm", test_app.address))
        .await
        .unwrap();

    // Assert
    assert_eq!(response.status().as_u16(), 400);
}

#[tokio::test]
async fn the_link_required_by_subscribe_returns_a_200_if_called() {
    // Arrange
    let test_app = spawn_app().await;
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&test_app.email_server)
        .await;

    test_app.post_subscriptions(body.into()).await;
    let email_request = &test_app.email_server.received_requests().await.unwrap()[0];
    let confirmation_links = test_app.get_confirmation_links(&email_request);

    // Act
    let response = reqwest::get(confirmation_links.html).await.unwrap();

    // Assert
    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn clicking_on_the_confirmation_link_confirms_a_subscriber() {
    // Arrange
    let test_app = spawn_app().await;
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";

    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&test_app.email_server)
        .await;

    test_app.post_subscriptions(body.into()).await;
    let email_request = &test_app.email_server.received_requests().await.unwrap()[0];
    let confirmation_links = test_app.get_confirmation_links(&email_request);

    // Act
    reqwest::get(confirmation_links.html)
        .await
        .unwrap()
        .error_for_status()
        .unwrap();

    // Assert
    let saved = sqlx::query!("SELECT email, name, status FROM subscriptions",)
        .fetch_one(&test_app.db_pool)
        .await
        .expect("Failed to fetch saved subscription.");

    assert_eq!(saved.email, "ursula_le_guin@gmail.com");
    assert_eq!(saved.name, "le guin");
    assert_eq!(saved.status, "confirmed");
}

#[tokio::test]
async fn get_subscription_id_from_token_fails_with_incorrect_token() {
    let test_app = spawn_app().await;
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&test_app.email_server)
        .await;
    let bad_token = "1234567890987654321";

    test_app.post_subscriptions(body.into()).await;
    let email_request = &test_app.email_server.received_requests().await.unwrap()[0];
    let confirmation_links = test_app.get_confirmation_links(&email_request);

    // interdict confirmation link, switch token, generate a new reqwest::Url from compromised path
    let interdicted_confirmation_link = confirmation_links.html.as_str();
    let mut link_parts = interdicted_confirmation_link
        .trim()
        .split("=")
        .collect::<Vec<&str>>();
    link_parts.pop();
    link_parts.insert(1, bad_token);
    let compromised_link = reqwest::Url::parse(link_parts.join("=").as_str()).unwrap();

    let response = reqwest::get(compromised_link).await.unwrap();

    assert_eq!(response.status().as_u16(), 401);
}

#[tokio::test]
async fn subscription_confirm_returns_500_when_no_status_field_present() {
    let test_app = spawn_app().await;
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&test_app.email_server)
        .await;

    test_app.post_subscriptions(body.into()).await;
    let email_request = &test_app.email_server.received_requests().await.unwrap()[0];
    let confirmation_links = test_app.get_confirmation_links(&email_request);

    sqlx::query!("ALTER TABLE subscriptions DROP COLUMN status;")
        .execute(&test_app.db_pool)
        .await
        .unwrap();

    let response = reqwest::get(confirmation_links.html).await.unwrap();

    assert_eq!(response.status().as_u16(), 500);
}
