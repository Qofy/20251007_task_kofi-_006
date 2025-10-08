use actix_web::{web, HttpResponse, Result};
use validator::Validate;
use crate::{
    database::Database,
    models::*,
};

pub async fn export_all_data(db: web::Data<Database>) -> Result<HttpResponse> {
    match db.export_all_data().await {
        Ok(export) => Ok(HttpResponse::Ok().json(ApiResponse::success(export))),
        Err(e) => {
            log::error!("Database error while exporting data: {}", e);
            Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                "Failed to export data".to_string()
            )))
        }
    }
}

pub async fn import_all_data(
    db: web::Data<Database>,
    import_data: web::Json<DatabaseExport>,
) -> Result<HttpResponse> {
    match db.import_all_data(&import_data).await {
        Ok(_) => Ok(HttpResponse::Ok().json(ApiResponse::success(
            "Data imported successfully".to_string()
        ))),
        Err(e) => {
            log::error!("Database error while importing data: {}", e);
            Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                "Failed to import data".to_string()
            )))
        }
    }
}

pub async fn get_data_statistics(db: web::Data<Database>) -> Result<HttpResponse> {
    match db.get_data_statistics().await {
        Ok(stats) => Ok(HttpResponse::Ok().json(ApiResponse::success(stats))),
        Err(e) => {
            log::error!("Database error while getting statistics: {}", e);
            Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                "Failed to get statistics".to_string()
            )))
        }
    }
}

pub async fn clear_all_data(db: web::Data<Database>) -> Result<HttpResponse> {
    match db.clear_all_data().await {
        Ok(_) => Ok(HttpResponse::Ok().json(ApiResponse::success(
            "All data cleared successfully".to_string()
        ))),
        Err(e) => {
            log::error!("Database error while clearing data: {}", e);
            Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                "Failed to clear data".to_string()
            )))
        }
    }
}

// Bulk operations for individual entities
pub async fn bulk_create_events(
    db: web::Data<Database>,
    bulk_request: web::Json<BulkCreateRequest<CreateEventRequest>>,
) -> Result<HttpResponse> {
    let mut success_count = 0;
    let mut error_count = 0;
    let mut errors = Vec::new();

    for event_data in &bulk_request.data {
        // Validate input
        if let Err(validation_errors) = event_data.validate() {
            let validation_error: String = validation_errors
                .field_errors()
                .iter()
                .flat_map(|(field, errors)| {
                    errors.iter().map(move |error| {
                        format!("{}: {}", field, 
                            error.message.as_ref().map(|msg| msg.to_string())
                                .unwrap_or_else(|| "Validation error".to_string())
                        )
                    })
                })
                .collect::<Vec<_>>()
                .join(", ");
            
            errors.push(format!("Event '{}': {}", event_data.title, validation_error));
            error_count += 1;
            continue;
        }

        // Validate dates
        if event_data.start_date >= event_data.end_date {
            errors.push(format!("Event '{}': End date must be after start date", event_data.title));
            error_count += 1;
            continue;
        }

        let event = Event {
            id: uuid::Uuid::new_v4(),
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
            Ok(_) => success_count += 1,
            Err(e) => {
                errors.push(format!("Event '{}': {}", event_data.title, e));
                error_count += 1;
            }
        }
    }

    let response = BulkOperationResponse {
        success_count,
        error_count,
        errors,
    };

    Ok(HttpResponse::Ok().json(ApiResponse::success(response)))
}

pub async fn bulk_create_venues(
    db: web::Data<Database>,
    bulk_request: web::Json<BulkCreateRequest<CreateVenueRequest>>,
) -> Result<HttpResponse> {
    let mut success_count = 0;
    let mut error_count = 0;
    let mut errors = Vec::new();

    for venue_data in &bulk_request.data {
        // Validate input
        if let Err(validation_errors) = venue_data.validate() {
            let validation_error: String = validation_errors
                .field_errors()
                .iter()
                .flat_map(|(field, errors)| {
                    errors.iter().map(move |error| {
                        format!("{}: {}", field, 
                            error.message.as_ref().map(|msg| msg.to_string())
                                .unwrap_or_else(|| "Validation error".to_string())
                        )
                    })
                })
                .collect::<Vec<_>>()
                .join(", ");
            
            errors.push(format!("Venue '{}': {}", venue_data.name, validation_error));
            error_count += 1;
            continue;
        }

        let venue = Venue {
            id: uuid::Uuid::new_v4(),
            name: venue_data.name.clone(),
            address: venue_data.address.clone(),
            capacity: venue_data.capacity,
            description: venue_data.description.clone(),
        };

        match db.create_venue(&venue).await {
            Ok(_) => success_count += 1,
            Err(e) => {
                errors.push(format!("Venue '{}': {}", venue_data.name, e));
                error_count += 1;
            }
        }
    }

    let response = BulkOperationResponse {
        success_count,
        error_count,
        errors,
    };

    Ok(HttpResponse::Ok().json(ApiResponse::success(response)))
}

pub async fn bulk_create_packages(
    db: web::Data<Database>,
    bulk_request: web::Json<BulkCreateRequest<CreatePackageRequest>>,
) -> Result<HttpResponse> {
    let mut success_count = 0;
    let mut error_count = 0;
    let mut errors = Vec::new();

    for package_data in &bulk_request.data {
        // Validate input
        if let Err(validation_errors) = package_data.validate() {
            let validation_error: String = validation_errors
                .field_errors()
                .iter()
                .flat_map(|(field, errors)| {
                    errors.iter().map(move |error| {
                        format!("{}: {}", field, 
                            error.message.as_ref().map(|msg| msg.to_string())
                                .unwrap_or_else(|| "Validation error".to_string())
                        )
                    })
                })
                .collect::<Vec<_>>()
                .join(", ");
            
            errors.push(format!("Package '{}': {}", package_data.name, validation_error));
            error_count += 1;
            continue;
        }

        let package = Package {
            id: uuid::Uuid::new_v4(),
            name: package_data.name.clone(),
            description: package_data.description.clone(),
            price: package_data.price,
            duration_days: package_data.duration_days,
            max_participants: package_data.max_participants,
        };

        match db.create_package(&package).await {
            Ok(_) => success_count += 1,
            Err(e) => {
                errors.push(format!("Package '{}': {}", package_data.name, e));
                error_count += 1;
            }
        }
    }

    let response = BulkOperationResponse {
        success_count,
        error_count,
        errors,
    };

    Ok(HttpResponse::Ok().json(ApiResponse::success(response)))
}