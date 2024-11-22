use std::net::TcpListener;
use zero2prod::startup::run;
use zero2prod::configuration::get_configuration;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // panic if we don't read configuration
    let configuration = get_configuration().expect("Failed to read configuration.");
    // Bubble up the error if we failed to bind the address
    // otherwise call .await on our server
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    run(listener)?.await
}
