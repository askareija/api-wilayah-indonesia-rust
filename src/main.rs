use actix_web::{web, App, HttpServer};
use env_logger::Env;
use log::info;

mod db;
mod handlers;
mod models;
mod routes;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // Setup logging
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    // Ensure the data directory exists
    std::fs::create_dir_all("data").expect("Failed to create data directory");

    // Initialize database
    let db = web::Data::new(
        db::Database::new("data/wilayah_indonesia.db").expect("Failed to create database connection"),
    );

    // Start HTTP server
    info!("Starting server at http://localhost:8080");
    HttpServer::new(move || {
        App::new()
            .app_data(db.clone())
            .configure(routes::configure_province_routes)
            .configure(routes::configure_regency_routes)
            .configure(routes::configure_district_routes)
            .configure(routes::configure_village_routes)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
