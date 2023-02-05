use actix_web::{get, Responder, HttpResponse};

#[get("/user")]
async fn get_user() -> impl Responder {
    return HttpResponse::Ok().body("Got the user");
}
