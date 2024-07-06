use actix_web::{get, post, web, HttpResponse, Result};
use maud::{html, Markup};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct LoginData {
    pub username: String,
    pub password: String,
}

#[get("/login")]
pub async fn get_login() -> HttpResponse {
    let resp = html! {
        html {
            head {
                title { "Login" }
            }
            body {
                h1 { "Login" }
                form action="/login" method="post" {
                    label for="username" { "Username: " }
                    input type="text" id="username" name="username" {}
                    br;
                    label for="password" { "Password: " }
                    input type="password" id="password" name="password" {}
                    br;
                    input type="submit" value="Login" {}
                }
                a href="/" { "Home" }
            }
        }
    };
    HttpResponse::Ok().content_type("text/html; charset=utf-8").body(resp.into_string())
}

#[post("/login")]
pub async fn post_login(form: web::Form<LoginData>) -> HttpResponse {
    let username = &form.username;
    let password = &form.password;

    // Пример простой проверки логина и пароля
    if username == "admin" && password == "password" {
        let resp = html! {
            html {
                head {
                    title { "Dashboard" }
                }
                body {
                    h1 { "Здраствуйте, " (username) "!" }
                    p { "Вы успешно вошли в TazaQala." }
                    a href="/" { "Home" }
                }
            }
        };
        HttpResponse::Ok().content_type("text/html; charset=utf-8").body(resp.into_string())
    } else {
        let resp = html! {
            html {
                head {
                    title { "Login" }
                }
                body {
                    h1 { "Login" }
                    p style="color:red;" { "Invalid username or password." }
                    form action="/login" method="post" {
                        label for="username" { "Username: " }
                        input type="text" id="username" name="username" {}
                        br;
                        label for="password" { "Password: " }
                        input type="password" id="password" name="password" {}
                        br;
                        input type="submit" value="Login" {}
                    }
                    a href="/" { "Home" }
                }
            }
        };
        HttpResponse::Ok().content_type("text/html; charset=utf-8").body(resp.into_string())
    }
}