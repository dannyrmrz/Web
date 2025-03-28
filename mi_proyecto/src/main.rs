use actix_files::Files;
use actix_web::{web, App, HttpServer};
use dotenvy::dotenv; // Cambia dotenv por dotenvy
use sqlx::postgres::PgPoolOptions;
use std::env;

mod models;
mod handlers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    
    // Configuración de la base de datos
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env file");
    
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create pool");

    // Ejecutar migraciones
    sqlx::query(include_str!("../partidos.sql"))
        .execute(&pool)
        .await
        .expect("Failed to run migrations");

    // Configuración del servidor HTTP
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            // Servir archivos estáticos del frontend
            .service(Files::new("/", "./Frontend").index_file("LaLigaTracker.html"))
            // Configuración de la API
            .route("/api/matches", web::get().to(handlers::get_matches))
            .route("/api/matches/{id}", web::get().to(handlers::get_match_by_id))
            .route("/api/matches", web::post().to(handlers::create_match))
            .route("/api/matches/{id}", web::put().to(handlers::update_match))
            .route("/api/matches/{id}", web::delete().to(handlers::delete_match))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}