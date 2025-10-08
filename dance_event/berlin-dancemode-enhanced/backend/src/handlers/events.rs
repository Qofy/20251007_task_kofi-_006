use actix_web::{web, HttpResponse, Result};
use uuid::Uuid;
use validator::Validate;
use crate::{
    database::Database,
    models::*,
};

pub async fn get_events(db: web::Data<Database>) -> Result<HttpResponse> {
    match db.get_all_events().await {
        Ok(events) => Ok(HttpResponse::Ok().json(ApiResponse::success(events))),
        Err(e) => {
            log::error!("Database error while getting events: {}", e);
            Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                "Internal server error".to_string()
            )))
        }
    }
}

pub async fn get_event(
    db: web::Data<Database>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse> {
    let event_id = path.into_inner();
    
    match db.get_event_by_id(&event_id).await {
        Ok(Some(event)) => Ok(HttpResponse::Ok().json(ApiResponse::success(event))),
        Ok(None) => Ok(HttpResponse::NotFound().json(ApiResponse::<()>::error(
            "Event not found".to_string()
        ))),
        Err(e) => {
            log::error!("Database error while getting event: {}", e);
            Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                "Internal server error".to_string()
            )))
        }
    }
}

pub async fn create_event(
    db: web::Data<Database>,
    event_data: web::Json<CreateEventRequest>,
) -> Result<HttpResponse> {
    // Validate input
    if let Err(validation_errors) = event_data.validate() {
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

    // Validate dates
    if event_data.start_date >= event_data.end_date {
        return Ok(HttpResponse::BadRequest().json(ApiResponse::<()>::error(
            "End date must be after start date".to_string()
        )));
    }

    let event = Event {
        id: Uuid::new_v4(),
        title: event_data.title.clone(),
        description: event_data.description.clone(),
        start_date: event_data.start_date,
        end_date: event_data.end_date,
        venue_id: event_data.venue_id,
        max_participants: event_data.max_participants,
        current_participants: 0,
        price: event_data.price,
        event_type: event_data.event_type.clone(),
    };

    match db.create_event(&event).await {
        Ok(_) => Ok(HttpResponse::Created().json(ApiResponse::success(event))),
        Err(e) => {
            log::error!("Database error while creating event: {}", e);
            Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                "Internal server error".to_string()
            )))
        }
    }
}