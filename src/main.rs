use std::net::TcpListener;

use actix_web;
use zero2prod::run;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    let adress = TcpListener::bind(("127.0.0.1", 8080)).unwrap();
    run(adress)?.await
}
