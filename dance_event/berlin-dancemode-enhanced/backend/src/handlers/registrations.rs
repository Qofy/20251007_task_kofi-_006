use actix_web::{web, HttpResponse, Result, HttpRequest};
use validator::Validate;
use uuid::Uuid;
use chrono::Utc;
use crate::{
    database::Database,
    models::*,
};

pub async fn create_registration(
    _req: HttpRequest,
    db: web::Data<Database>,
    registration_data: web::Json<CreateRegistrationRequest>,
) -> Result<HttpResponse> {
    // Validate the request
    if let Err(errors) = registration_data.validate() {
        let error_messages: Vec<String> = errors
            .field_errors()
            .into_iter()
            .flat_map(|(field, errors)| {
                errors.iter().map(move |error| {
                    format!("{}: {}", field, error.message.as_ref().unwrap_or(&"Invalid value".into()))
                })
            })
            .collect();
        return Ok(HttpResponse::BadRequest().json(ApiResponse::<()>::validation_error(error_messages)));
    }

    // For demo purposes, use a default user ID
    // In production, this would come from JWT token
    let user_id = Uuid::new_v4();
    
    // Create the registration
    let registration = Registration {
        id: Uuid::new_v4(),
        user_id,
        event_id: registration_data.event_id,
        package_id: registration_data.package_id,
        status: RegistrationStatus::Pending,
        payment_status: PaymentStatus::Pending,
        created_at: Utc::now(),
        updated_at: Utc::now(),
        amount_paid: 0.0,
    };

    // Try to save to database
    match db.create_registration(&registration).await {
        Ok(_) => {
            log::info!("Registration created successfully: {}", registration.id);
            
            // If this is an event registration, increment participant count
            if let Some(event_id) = registration.event_id {
                if let Err(e) = db.increment_event_participants(&event_id).await {
                    log::warn!("Failed to increment event participants: {}", e);
                }
            }
            
            Ok(HttpResponse::Ok().json(ApiResponse::success(registration)))
        }
        Err(e) => {
            log::error!("Failed to create registration: {}", e);
            Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                "Failed to create registration".to_string()
            )))
        }
    }
}

pub async fn get_user_registrations(
    _req: HttpRequest,
    _db: web::Data<Database>,
) -> Result<HttpResponse> {
    // This would typically extract user ID from JWT token
    // For now, return a placeholder response
    Ok(HttpResponse::Ok().json(ApiResponse::<()>::error(
        "User registrations endpoint - JWT middleware needed".to_string()
    )))
}

pub async fn process_payment(
    _req: HttpRequest,
    _db: web::Data<Database>,
    _path: web::Path<Uuid>,
    _payment_data: web::Json<PaymentRequest>,
) -> Result<HttpResponse> {
    // This would integrate with payment processors like Stripe
    // For now, return a placeholder response
    Ok(HttpResponse::Ok().json(ApiResponse::<()>::error(
        "Payment processing endpoint - Payment integration needed".to_string()
    )))
}