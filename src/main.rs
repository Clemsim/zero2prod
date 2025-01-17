use zero2prod::run;
use actix_web;

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    run().await
}