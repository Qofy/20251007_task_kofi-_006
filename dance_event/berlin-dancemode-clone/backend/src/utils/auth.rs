use anyhow::Result;
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use std::env;
use crate::models::Claims;

const JWT_SECRET: &str = "your-secret-key-change-in-production";

pub fn hash_password(password: &str) -> Result<String> {
    let hashed = hash(password, DEFAULT_COST)?;
    Ok(hashed)
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool> {
    let is_valid = verify(password, hash)?;
    Ok(is_valid)
}

pub fn generate_jwt(user_id: &str, email: &str) -> Result<String> {
    let secret = env::var("JWT_SECRET").unwrap_or_else(|_| JWT_SECRET.to_string());
    
    let expiration = Utc::now()
        .checked_add_signed(Duration::days(30))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: user_id.to_string(),
        email: email.to_string(),
        exp: expiration,
        iat: Utc::now().timestamp() as usize,
    };

    let header = Header::default();
    let key = EncodingKey::from_secret(secret.as_ref());
    
    let token = encode(&header, &claims, &key)?;
    Ok(token)
}

pub fn verify_jwt(token: &str) -> Result<Claims> {
    let secret = env::var("JWT_SECRET").unwrap_or_else(|_| JWT_SECRET.to_string());
    
    let key = DecodingKey::from_secret(secret.as_ref());
    let validation = Validation::default();
    
    let token_data = decode::<Claims>(token, &key, &validation)?;
    Ok(token_data.claims)
}

// Middleware for JWT authentication
use actix_web::{dev::ServiceRequest, Error, HttpMessage};
use actix_web_httpauth::extractors::bearer::BearerAuth;

pub async fn jwt_middleware(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    match verify_jwt(credentials.token()) {
        Ok(claims) => {
            req.extensions_mut().insert(claims);
            Ok(req)
        }
        Err(_) => {
            let response = actix_web::HttpResponse::Unauthorized()
                .json(crate::models::ApiResponse::<()>::error("Invalid token".to_string()));
            Err((actix_web::error::ErrorUnauthorized(response), req))
        }
    }
}