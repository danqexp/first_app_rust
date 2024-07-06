use actix_web::{get, HttpResponse};
use maud::{html, Markup};

#[get("/")]
pub async fn index() -> HttpResponse {
    let resp = html! {
        html {
            head {
                title { "Home" }
            }
            body {
                h1 { "Hello World!" }
                p { "Welcome to the homepage." }
                a href="/about" { "About" }
            }
        }
    };
    HttpResponse::Ok().content_type("text/html; charset=utf-8").body(resp.into_string())
}

#[get("/about")]
pub async fn about() -> HttpResponse {
    let resp = html! {
        html {
            head {
                title { "About" }
            }
            body {
                h1 { "About Us" }
                p { "This is the about page." }
                a href="/" { "Home" }
            }
        }
    };
    HttpResponse::Ok().content_type("text/html; charset=utf-8").body(resp.into_string())
}
