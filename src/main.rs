#[path = "index.rs"]
mod index;
use actix_web::{App, HttpServer, HttpResponse};
use std::io;
use crate::handlers::some_function;

#[actix_web::main]
async fn main() -> io::Result<()> {
    HttpServer::new(|| App::new()
        .service(index::index))
        .service(handlers::about)
        .service(handlers::contact::get_contact)
        .service(handlers::contact::post_contact)
        .service(handlers::auth::get_login)
        .service(handlers::auth::post_login)
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}