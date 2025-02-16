use std::{fmt::format, net::TcpListener};
use serde;
use actix_web::{
    dev::Server, http::StatusCode, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder,
};

async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

#[derive(serde::Deserialize)]
struct FormData{
    email: String,
    name: String,
}

async fn subscriptions(form : web::Form<FormData>) -> String {
    
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| App::new()
        .route("/health_check", web::get().to(health_check))
        .route("/subscriptions", web::post().to(subscriptions))
        )
        .listen(listener)?
        .run();

    Ok(server)
}
