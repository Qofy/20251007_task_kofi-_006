use actix_web::{web, HttpResponse, Result};
use validator::Validate;
use crate::{
    database::Database,
    models::*,
    utils::auth::{hash_password, verify_password, generate_jwt},
};
use chrono::Utc;
use uuid::Uuid;

pub async fn register(
    db: web::Data<Database>,
    user_data: web::Json<SimpleRegisterRequest>,
) -> Result<HttpResponse> {
    // Validate input
    if let Err(validation_errors) = user_data.validate() {
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

    // Check if user already exists
    match db.get_user_by_email(&user_data.email).await {
        Ok(Some(_)) => {
            return Ok(HttpResponse::Conflict().json(ApiResponse::<()>::error(
                "User with this email already exists".to_string()
            )));
        }
        Err(e) => {
            log::error!("Database error while checking user existence: {}", e);
            return Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                "Internal server error".to_string()
            )));
        }
        _ => {}
    }

    // Hash password
    let password_hash = match hash_password(&user_data.password) {
        Ok(hash) => hash,
        Err(e) => {
            log::error!("Password hashing error: {}", e);
            return Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                "Internal server error".to_string()
            )));
        }
    };

    // Create user with username as first_name for simplicity
    let user = User {
        id: Uuid::new_v4(),
        email: user_data.email.clone(),
        password_hash,
        first_name: user_data.username.clone(), // Use username as first name
        last_name: "User".to_string(), // Default last name
        phone: None,
        dance_experience: DanceExperience::Beginner, // Default to beginner
        created_at: Utc::now(),
        updated_at: Utc::now(),
        is_active: true,
    };

    // Save user to database
    match db.create_user(&user).await {
        Ok(_) => {
            // Generate JWT token
            match generate_jwt(&user.id.to_string(), &user.email) {
                Ok(token) => {
                    let auth_response = AuthResponse {
                        token,
                        user: user.into(),
                    };
                    Ok(HttpResponse::Created().json(ApiResponse::success(auth_response)))
                }
                Err(e) => {
                    log::error!("JWT generation error: {}", e);
                    Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                        "Internal server error".to_string()
                    )))
                }
            }
        }
        Err(e) => {
            log::error!("Database error while creating user: {}", e);
            Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                "Internal server error".to_string()
            )))
        }
    }
}

pub async fn login(
    db: web::Data<Database>,
    login_data: web::Json<LoginRequest>,
) -> Result<HttpResponse> {
    // Validate input
    if let Err(validation_errors) = login_data.validate() {
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

    // Get user by email
    let user = match db.get_user_by_email(&login_data.email).await {
        Ok(Some(user)) => user,
        Ok(None) => {
            return Ok(HttpResponse::Unauthorized().json(ApiResponse::<()>::error(
                "Invalid email or password".to_string()
            )));
        }
        Err(e) => {
            log::error!("Database error while getting user: {}", e);
            return Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                "Internal server error".to_string()
            )));
        }
    };

    // Verify password
    if !user.is_active {
        return Ok(HttpResponse::Unauthorized().json(ApiResponse::<()>::error(
            "Account is disabled".to_string()
        )));
    }

    match verify_password(&login_data.password, &user.password_hash) {
        Ok(true) => {
            // Generate JWT token
            match generate_jwt(&user.id.to_string(), &user.email) {
                Ok(token) => {
                    let auth_response = AuthResponse {
                        token,
                        user: user.into(),
                    };
                    Ok(HttpResponse::Ok().json(ApiResponse::success(auth_response)))
                }
                Err(e) => {
                    log::error!("JWT generation error: {}", e);
                    Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                        "Internal server error".to_string()
                    )))
                }
            }
        }
        Ok(false) => {
            Ok(HttpResponse::Unauthorized().json(ApiResponse::<()>::error(
                "Invalid email or password".to_string()
            )))
        }
        Err(e) => {
            log::error!("Password verification error: {}", e);
            Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                "Internal server error".to_string()
            )))
        }
    }
}