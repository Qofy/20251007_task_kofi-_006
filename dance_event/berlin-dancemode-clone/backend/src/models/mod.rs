use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
    pub first_name: String,
    pub last_name: String,
    pub phone: Option<String>,
    pub dance_experience: DanceExperience,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DanceExperience {
    Beginner,
    Intermediate,
    Advanced,
    Professional,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateUserRequest {
    #[validate(email(message = "Invalid email address"))]
    pub email: String,
    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub password: String,
    #[validate(length(min = 2, message = "First name must be at least 2 characters"))]
    pub first_name: String,
    #[validate(length(min = 2, message = "Last name must be at least 2 characters"))]
    pub last_name: String,
    pub phone: Option<String>,
    pub dance_experience: DanceExperience,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(email(message = "Invalid email address"))]
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: UserProfile,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfile {
    pub id: Uuid,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub phone: Option<String>,
    pub dance_experience: DanceExperience,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub venue_id: Uuid,
    pub max_participants: u32,
    pub current_participants: u32,
    pub price: f64,
    pub event_type: EventType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventType {
    Workshop,
    Festival,
    Intensive,
    Social,
    Competition,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateEventRequest {
    #[validate(length(min = 3, message = "Title must be at least 3 characters"))]
    pub title: String,
    #[validate(length(min = 10, message = "Description must be at least 10 characters"))]
    pub description: String,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub venue_id: Uuid,
    #[validate(range(min = 1, max = 1000, message = "Max participants must be between 1 and 1000"))]
    pub max_participants: u32,
    #[validate(range(min = 0.0, message = "Price must be non-negative"))]
    pub price: f64,
    pub event_type: EventType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Venue {
    pub id: Uuid,
    pub name: String,
    pub address: String,
    pub capacity: u32,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateVenueRequest {
    #[validate(length(min = 2, message = "Venue name must be at least 2 characters"))]
    pub name: String,
    #[validate(length(min = 5, message = "Address must be at least 5 characters"))]
    pub address: String,
    #[validate(range(min = 1, max = 10000, message = "Capacity must be between 1 and 10000"))]
    pub capacity: u32,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Package {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub price: f64,
    pub duration_days: u32,
    pub max_participants: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreatePackageRequest {
    #[validate(length(min = 3, message = "Package name must be at least 3 characters"))]
    pub name: String,
    #[validate(length(min = 10, message = "Description must be at least 10 characters"))]
    pub description: String,
    #[validate(range(min = 0.0, message = "Price must be non-negative"))]
    pub price: f64,
    #[validate(range(min = 1, max = 365, message = "Duration must be between 1 and 365 days"))]
    pub duration_days: u32,
    #[validate(range(min = 1, max = 1000, message = "Max participants must be between 1 and 1000"))]
    pub max_participants: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Registration {
    pub id: Uuid,
    pub user_id: Uuid,
    pub event_id: Option<Uuid>,
    pub package_id: Option<Uuid>,
    pub status: RegistrationStatus,
    pub payment_status: PaymentStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub amount_paid: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RegistrationStatus {
    Pending,
    Confirmed,
    Cancelled,
    WaitingList,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PaymentStatus {
    Pending,
    Completed,
    Failed,
    Refunded,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateRegistrationRequest {
    pub event_id: Option<Uuid>,
    pub package_id: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentRequest {
    pub registration_id: Uuid,
    pub payment_method: String, // For future integration with payment processors
    pub amount: f64,
}

// API Response types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub message: Option<String>,
    pub errors: Option<Vec<String>>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            message: None,
            errors: None,
        }
    }

    pub fn error(message: String) -> Self {
        Self {
            success: false,
            data: None,
            message: Some(message),
            errors: None,
        }
    }

    pub fn validation_error(errors: Vec<String>) -> Self {
        Self {
            success: false,
            data: None,
            message: Some("Validation failed".to_string()),
            errors: Some(errors),
        }
    }
}

// Helper function to convert User to UserProfile (without sensitive data)
impl From<User> for UserProfile {
    fn from(user: User) -> Self {
        UserProfile {
            id: user.id,
            email: user.email,
            first_name: user.first_name,
            last_name: user.last_name,
            phone: user.phone,
            dance_experience: user.dance_experience,
            created_at: user.created_at,
        }
    }
}

// JWT Claims
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // User ID
    pub email: String,
    pub exp: usize, // Expiration time
    pub iat: usize, // Issued at
}