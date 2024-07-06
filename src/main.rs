#[path = "index.rs"]
mod index;
#[path = "handlers/about.rs"]
mod about;
use actix_web::{App, HttpServer, HttpResponse};
use std::io;
#[path = "handlers/auth.rs"]
mod auth;
#[path = "handlers/contact.rs"]
mod contact;


#[actix_web::main]
async fn main() -> io::Result<()> {
    HttpServer::new(move||App::new()
        .service(index::index)
        .service(contact::index)
        .service(contact::about)
        .service(auth::get_login)
        .service(auth::post_login)
)
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}