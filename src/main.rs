use std::net::TcpListener;
use zero2prod::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let port = 8000;
    let listener = TcpListener::bind(format!("localhost:{}", port))
        .expect(&format!("failed to bind port {}", port));
    run(listener)?.await
}
