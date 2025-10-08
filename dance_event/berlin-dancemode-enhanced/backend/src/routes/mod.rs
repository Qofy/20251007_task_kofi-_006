use actix_web::web;
use crate::handlers::{
    auth::{login, register},
    data::{export_all_data, import_all_data, get_data_statistics, clear_all_data, 
           bulk_create_events, bulk_create_venues, bulk_create_packages},
    events::{get_events, get_event, create_event},
    users::{get_profile, update_profile},
    venues::{get_venues, create_venue},
    packages::{get_packages, create_package},
    registrations::{create_registration, get_user_registrations, process_payment},
};

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg
        // Health check
        .route("/health", web::get().to(health_check))
        
        // Auth routes
        .service(
            web::scope("/api/auth")
                .route("/register", web::post().to(register))
                .route("/login", web::post().to(login))
        )
        
        // User routes
        .service(
            web::scope("/api/users")
                .route("/profile", web::get().to(get_profile))
                .route("/profile", web::put().to(update_profile))
                .route("/registrations", web::get().to(get_user_registrations))
        )
        
        // Event routes
        .service(
            web::scope("/api/events")
                .route("", web::get().to(get_events))
                .route("", web::post().to(create_event))
                .route("/{event_id}", web::get().to(get_event))
        )
        
        // Venue routes
        .service(
            web::scope("/api/venues")
                .route("", web::get().to(get_venues))
                .route("", web::post().to(create_venue))
        )
        
        // Package routes
        .service(
            web::scope("/api/packages")
                .route("", web::get().to(get_packages))
                .route("", web::post().to(create_package))
        )
        
        // Registration routes
        .service(
            web::scope("/api/registrations")
                .route("", web::post().to(create_registration))
                .route("/{registration_id}/payment", web::post().to(process_payment))
        )
        
        // Data management routes
        .service(
            web::scope("/api/data")
                .route("/export", web::get().to(export_all_data))
                .route("/import", web::post().to(import_all_data))
                .route("/statistics", web::get().to(get_data_statistics))
                .route("/clear", web::delete().to(clear_all_data))
                .route("/bulk/events", web::post().to(bulk_create_events))
                .route("/bulk/venues", web::post().to(bulk_create_venues))
                .route("/bulk/packages", web::post().to(bulk_create_packages))
        );
}

async fn health_check() -> actix_web::Result<actix_web::HttpResponse> {
    Ok(actix_web::HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "timestamp": chrono::Utc::now(),
        "service": "berlin-dancemode-backend"
    })))
}