mod logging;
use actix_web::{App, HttpResponse, HttpServer, Responder, get};
use std::env;
use tracing::info;

#[get("/")]
async fn ping() -> impl Responder {
    HttpResponse::Ok().body("OK")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    logging::init_logging();
    let port = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .expect("PORT must be a valid number");
    info!("Server started on port {}", port);
    HttpServer::new(|| App::new().service(ping))
        .bind(("127.0.0.1", port))?
        .run()
        .await
}
