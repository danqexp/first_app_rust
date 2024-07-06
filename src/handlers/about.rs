use actix_web::{get, HttpResponse, Responder};

#[get("/about_test")]
pub async fn about_function() -> HttpResponse {
    HttpResponse::Ok().content_type("text/html; charset=utf-8").body("About Page")
}
