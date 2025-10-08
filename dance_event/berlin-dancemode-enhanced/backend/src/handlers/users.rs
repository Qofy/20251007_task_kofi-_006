use actix_web::{web, HttpResponse, Result, HttpRequest};
use uuid::Uuid;
use crate::{
    database::Database,
    models::*,
    utils::auth::verify_jwt,
};

pub async fn get_profile(
    req: HttpRequest,
    db: web::Data<Database>,
) -> Result<HttpResponse> {
    // Extract token from Authorization header
    let auth_header = req.headers().get("Authorization");
    
    if let Some(auth_value) = auth_header {
        if let Ok(auth_str) = auth_value.to_str() {
            if auth_str.starts_with("Bearer ") {
                let token = &auth_str[7..]; // Remove "Bearer " prefix
                
                // Verify JWT token and extract user ID
                match verify_jwt(token) {
                    Ok(claims) => {
                        // Parse user ID as UUID
                        let user_id = match Uuid::parse_str(&claims.sub) {
                            Ok(id) => id,
                            Err(_) => {
                                return Ok(HttpResponse::BadRequest().json(ApiResponse::<()>::error(
                                    "Invalid user ID format".to_string()
                                )));
                            }
                        };
                        
                        // Get user from database
                        match db.get_user_by_id(&user_id).await {
                            Ok(Some(mut user)) => {
                                // Remove password hash for security
                                user.password_hash = String::new();
                                
                                Ok(HttpResponse::Ok().json(ApiResponse::success(user)))
                            },
                            Ok(None) => {
                                Ok(HttpResponse::NotFound().json(ApiResponse::<()>::error(
                                    "User not found".to_string()
                                )))
                            },
                            Err(err) => {
                                log::error!("Database error getting user profile: {}", err);
                                Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                                    "Failed to retrieve profile".to_string()
                                )))
                            }
                        }
                    },
                    Err(_) => {
                        Ok(HttpResponse::Unauthorized().json(ApiResponse::<()>::error(
                            "Invalid token".to_string()
                        )))
                    }
                }
            } else {
                Ok(HttpResponse::Unauthorized().json(ApiResponse::<()>::error(
                    "Invalid Authorization header format".to_string()
                )))
            }
        } else {
            Ok(HttpResponse::Unauthorized().json(ApiResponse::<()>::error(
                "Invalid Authorization header".to_string()
            )))
        }
    } else {
        Ok(HttpResponse::Unauthorized().json(ApiResponse::<()>::error(
            "Authorization header required".to_string()
        )))
    }
}

pub async fn update_profile(
    req: HttpRequest,
    db: web::Data<Database>,
    user_data: web::Json<serde_json::Value>,
) -> Result<HttpResponse> {
    // Extract token from Authorization header
    let auth_header = req.headers().get("Authorization");
    
    if let Some(auth_value) = auth_header {
        if let Ok(auth_str) = auth_value.to_str() {
            if auth_str.starts_with("Bearer ") {
                let token = &auth_str[7..]; // Remove "Bearer " prefix
                
                // Verify JWT token and extract user ID
                match verify_jwt(token) {
                    Ok(claims) => {
                        // For now, just return success - full implementation would update user data
                        Ok(HttpResponse::Ok().json(ApiResponse::success(serde_json::json!({
                            "message": "Profile update functionality not yet implemented",
                            "user_id": claims.sub
                        }))))
                    },
                    Err(_) => {
                        Ok(HttpResponse::Unauthorized().json(ApiResponse::<()>::error(
                            "Invalid token".to_string()
                        )))
                    }
                }
            } else {
                Ok(HttpResponse::Unauthorized().json(ApiResponse::<()>::error(
                    "Invalid Authorization header format".to_string()
                )))
            }
        } else {
            Ok(HttpResponse::Unauthorized().json(ApiResponse::<()>::error(
                "Invalid Authorization header".to_string()
            )))
        }
    } else {
        Ok(HttpResponse::Unauthorized().json(ApiResponse::<()>::error(
            "Authorization header required".to_string()
        )))
    }
}