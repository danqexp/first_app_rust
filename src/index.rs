use actix_web::{get,HttpResponse, web, post};
use maud::html;
use serde::Deserialize;

#[get("/")]
async fn index() -> HttpResponse {
    let resp = html! {
        html {
            body {
                h1 { "Hello World!" }
            }
        }
    };
    HttpResponse::Ok().content_type("text/html; charset=utf-8").body(resp.into_string())
}

#[derive(Deserialize)]
struct FormData {
    name: String,
    message: String,
}

#[get("/contact")]
async fn contact() -> HttpResponse {
    let resp = html! {
        html {
            head {
                title { "Contact" }
            }
            body {
                h1 { "Contact Us" }
                form action="/contact" method="post" {
                    label for="name" { "Name: " }
                    input type="text" id="name" name="name" {}
                    br;
                    label for="message" { "Message: " }
                    textarea id="message" name="message" {}
                    br;
                    input type="submit" value="Submit" {}
                }
                a href="/" { "Home" }
            }
        }
    };
    HttpResponse::Ok().content_type("text/html; charset=utf-8").body(resp.into_string())
}

#[post("/contact")]
pub async fn handle_contact(form: web::Form<FormData>) -> HttpResponse {
    let resp = html! {
        html {
            head {
                title { "Contact" }
            }
            body {
                h1 { "Thank You!" }
                p { "We have received your message." }
                p { "Name: " (form.name) }
                p { "Message: " (form.message) }
                a href="/" { "Home" }
            }
        }
    };
    HttpResponse::Ok().content_type("text/html; charset=utf-8").body(resp.into_string())
}
