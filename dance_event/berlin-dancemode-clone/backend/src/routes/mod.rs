use actix_web::web;
use crate::handlers::{
    auth::{login, register},
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
        );
}

async fn health_check() -> actix_web::Result<actix_web::HttpResponse> {
    Ok(actix_web::HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "timestamp": chrono::Utc::now(),
        "service": "berlin-dancemode-backend"
    })))
}