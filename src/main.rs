use std::net::TcpListener;

use zero2prod::run;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8090").expect("Failed to bind to address");
    run(listener)?.await
}
