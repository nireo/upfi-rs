use std::net::TcpListener;

use upfi::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:0").expect("failed to bind random port.");
    run(listener)?.await
}
