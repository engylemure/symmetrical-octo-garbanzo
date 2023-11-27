use std::net::SocketAddr;

use tracing_actix_web::TracingLogger;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let port: u16 = std::env::var("PORT")
        .unwrap_or("8080".into())
        .parse()
        .unwrap_or(8080);
    let socket_addr: SocketAddr = format!("[::]:{}", port).parse()?;
    tracing::info!("Starting App Server at: {}", socket_addr);
    let server = HttpServer::new(|| {
        App::new()
            .wrap(TracingLogger::default())
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(socket_addr)?;

    Ok(server.run().await?)
}
