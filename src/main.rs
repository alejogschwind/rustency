// src/main.rs

// dependencies here
use actix_web::{get, App, web, HttpServer, HttpResponse, Responder};

// module declaration here
mod handlers;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello Word!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    std::env::set_var("RUST_LOG", "actix_web=debug");

    // Start http server
    HttpServer::new(move || {
        App::new()
            .route("/currencies/", web::get().to(handlers::get_currencies))
            .route("/currencies/{id}", web::get().to(handlers::get_currency_by_id))
            .route("/currencies/", web::post().to(handlers::add_currency))
            .route("/currencies/{id}", web::delete().to(handlers::delete_currency))
            .service(hello)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
