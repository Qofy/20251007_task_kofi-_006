use actix_web::{web, HttpResponse, Result};
use validator::Validate;
use uuid::Uuid;
use crate::{
    database::Database,
    models::*,
};

pub async fn get_venues(db: web::Data<Database>) -> Result<HttpResponse> {
    match db.get_all_venues().await {
        Ok(venues) => Ok(HttpResponse::Ok().json(ApiResponse::success(venues))),
        Err(e) => {
            log::error!("Database error while getting venues: {}", e);
            Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                "Internal server error".to_string()
            )))
        }
    }
}

pub async fn create_venue(
    db: web::Data<Database>,
    venue_data: web::Json<CreateVenueRequest>,
) -> Result<HttpResponse> {
    // Validate input
    if let Err(validation_errors) = venue_data.validate() {
        let errors: Vec<String> = validation_errors
            .field_errors()
            .iter()
            .flat_map(|(_, errors)| {
                errors.iter().map(|error| {
                    error.message.as_ref().map(|msg| msg.to_string())
                        .unwrap_or_else(|| "Validation error".to_string())
                })
            })
            .collect();
        
        return Ok(HttpResponse::BadRequest().json(ApiResponse::<()>::validation_error(errors)));
    }

    let venue = Venue {
        id: Uuid::new_v4(),
        name: venue_data.name.clone(),
        address: venue_data.address.clone(),
        capacity: venue_data.capacity,
        description: venue_data.description.clone(),
    };

    match db.create_venue(&venue).await {
        Ok(_) => Ok(HttpResponse::Created().json(ApiResponse::success(venue))),
        Err(e) => {
            log::error!("Database error while creating venue: {}", e);
            Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                "Internal server error".to_string()
            )))
        }
    }
}