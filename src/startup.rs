use std::net::TcpListener;
use actix_web::{App, web, HttpServer};
use actix_web::dev::Server;
use crate::routes::{health_check, subscribe};

// We return Server on the happy path and we drop the `async` keyword.
// We have no need for it since we have no .await call.
pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
    })
        .listen(listener)?
        .run();

    Ok(server)
}
