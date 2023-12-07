mod api;
mod entities;
use std::net::SocketAddr;

use dotenv::dotenv;
use sea_orm::Database;
use tracing_actix_web::TracingLogger;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

use actix_web::{App, HttpServer};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    tracing_subscriber::registry()
        .with(
            fmt::layer().with_span_events(fmt::format::FmtSpan::NEW | fmt::format::FmtSpan::CLOSE),
        )
        .with(EnvFilter::from_default_env())
        .init();
    let port: u16 = std::env::var("PORT")
        .unwrap_or("8080".into())
        .parse()
        .unwrap_or(8080);
    let socket_addr: SocketAddr = format!("[::]:{}", port).parse()?;
    let db_user = std::env::var("DB_USER")?;
    let db_password = std::env::var("DB_PASSWORD")?;
    let db_host = std::env::var("DB_HOST")?;
    let db_name = std::env::var("DB_NAME")?;
    let db_conn = Database::connect(&format!(
        "postgres://{}:{}@{}/{}",
        db_user, db_password, db_host, db_name
    ))
    .await?;
    let db_conn = actix_web::web::Data::new(db_conn);
    tracing::info!("Starting App Server at: {}", socket_addr);
    let server = HttpServer::new(move || {
        App::new()
            .app_data(db_conn.clone())
            .wrap(actix_cors::Cors::permissive())
            .wrap(TracingLogger::default())
            .configure(api::number::config)
            .service(actix_files::Files::new("/static", "./static").show_files_listing())
    })
    .bind(socket_addr)?;

    Ok(server.run().await?)
}
