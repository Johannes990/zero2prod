use std::net::TcpListener;
use zero2prod::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // Bubble up the error if we failed to bind the address
    // otherwise call .await on our server
    let listener = TcpListener::bind("127.0.0.1:0")?;
    run(listener)?.await
}
