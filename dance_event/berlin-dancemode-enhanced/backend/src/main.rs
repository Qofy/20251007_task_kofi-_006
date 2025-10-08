mod database;
mod handlers;
mod models;
mod routes;
mod utils;

use actix_cors::Cors;
use actix_web::{web, App, HttpServer, Result, middleware::Logger};
use database::Database;
use routes::configure_routes;
use std::env;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logger
    env_logger::init();
    
    // Load environment variables or set defaults
    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .expect("PORT must be a valid number");

    // Initialize database
    let db = Database::new("./data/dancemode.sled")?;
    log::info!("Database initialized successfully");

    // Initialize database with some sample data
    db.init_sample_data().await?;
    log::info!("Sample data initialized");

    log::info!("Starting server at http://{}:{}", host, port);

    // Start HTTP server
    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .app_data(web::Data::new(db.clone()))
            .wrap(cors)
            .wrap(Logger::default())
            .configure(configure_routes)
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await?;

    Ok(())
}