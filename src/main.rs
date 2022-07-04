use actix_web::{get, App, HttpResponse, HttpServer, Responder};

mod unsigned_integer;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("FurDB")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(unsigned_integer::encode)
            .service(unsigned_integer::decode)
    })
    .bind(("127.0.0.1", 6000))?
    .run()
    .await
}
