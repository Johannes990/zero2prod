use sqlx::PgPool;
use std::net::TcpListener;
use env_logger::Env;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // 'init' does call 'set_logger', so this is all we need to do
    // we are falling back to printing all logs at info-level time or above
    // if the RUST_LOG environment variable has not been set
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    // panic if we don't read configuration
    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");
    // Bubble up the error if we failed to bind the address
    // otherwise call .await on our server
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    run(listener, connection_pool)?.await
}
