use std::io::Result;
use actix_web::{get, HttpServer, App, Responder, HttpResponse};

mod handlers;
mod middleware;

#[get("/")]
async fn hello() -> impl Responder {
    return HttpResponse::Ok().body("Hello");
}

#[actix_web::main]
async fn main() -> Result<()> {
    println!("Listening at http://localhost:8080");

    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(handlers::user::get_user)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
