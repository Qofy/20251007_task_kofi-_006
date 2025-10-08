use actix_web::{web, HttpResponse, Result};
use validator::Validate;
use uuid::Uuid;
use crate::{
    database::Database,
    models::*,
};

pub async fn get_packages(db: web::Data<Database>) -> Result<HttpResponse> {
    match db.get_all_packages().await {
        Ok(packages) => Ok(HttpResponse::Ok().json(ApiResponse::success(packages))),
        Err(e) => {
            log::error!("Database error while getting packages: {}", e);
            Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                "Internal server error".to_string()
            )))
        }
    }
}

pub async fn create_package(
    db: web::Data<Database>,
    package_data: web::Json<CreatePackageRequest>,
) -> Result<HttpResponse> {
    // Validate input
    if let Err(validation_errors) = package_data.validate() {
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

    let package = Package {
        id: Uuid::new_v4(),
        name: package_data.name.clone(),
        description: package_data.description.clone(),
        price: package_data.price,
        duration_days: package_data.duration_days,
        max_participants: package_data.max_participants,
    };

    match db.create_package(&package).await {
        Ok(_) => Ok(HttpResponse::Created().json(ApiResponse::success(package))),
        Err(e) => {
            log::error!("Database error while creating package: {}", e);
            Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                "Internal server error".to_string()
            )))
        }
    }
}