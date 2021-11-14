use actix_web::{get, App, web, HttpServer, HttpResponse, Responder};

extern crate diesel;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

// module declaration here
mod errors;
mod handlers;
mod models;
mod schema;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello Word!")
}

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=debug");
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Create db connection pool
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool: Pool = r2d2::Pool::builder()
     .build(manager)
     .expect("Failed to create pool.");

    // Start http server
    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
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
