use crate::models::*;
use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sled::{Db, IVec};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct Database {
    db: Arc<Db>,
}

impl Database {
    pub fn new(path: &str) -> Result<Self> {
        let db = sled::open(path)?;
        Ok(Database { db: Arc::new(db) })
    }

    // Helper method to serialize data
    fn serialize<T: Serialize>(&self, data: &T) -> Result<Vec<u8>> {
        Ok(serde_json::to_vec(data)?)
    }

    // Helper method to deserialize data
    fn deserialize<T: for<'de> Deserialize<'de>>(&self, data: IVec) -> Result<T> {
        Ok(serde_json::from_slice(&data)?)
    }

    // Users
    pub async fn create_user(&self, user: &User) -> Result<()> {
        let key = format!("user:{}", user.id);
        let value = self.serialize(user)?;
        self.db.insert(key, value)?;
        
        // Also index by email
        let email_key = format!("user_email:{}", user.email);
        self.db.insert(email_key, user.id.to_string().as_bytes())?;
        
        Ok(())
    }

    pub async fn get_user_by_id(&self, user_id: &Uuid) -> Result<Option<User>> {
        let key = format!("user:{}", user_id);
        if let Some(value) = self.db.get(&key)? {
            Ok(Some(self.deserialize(value)?))
        } else {
            Ok(None)
        }
    }

    pub async fn get_user_by_email(&self, email: &str) -> Result<Option<User>> {
        let email_key = format!("user_email:{}", email);
        if let Some(user_id_bytes) = self.db.get(&email_key)? {
            let user_id_str = String::from_utf8(user_id_bytes.to_vec())?;
            let user_id = Uuid::parse_str(&user_id_str)?;
            self.get_user_by_id(&user_id).await
        } else {
            Ok(None)
        }
    }

    // Events
    pub async fn create_event(&self, event: &Event) -> Result<()> {
        let key = format!("event:{}", event.id);
        let value = self.serialize(event)?;
        self.db.insert(key, value)?;
        Ok(())
    }

    pub async fn get_event_by_id(&self, event_id: &Uuid) -> Result<Option<Event>> {
        let key = format!("event:{}", event_id);
        if let Some(value) = self.db.get(&key)? {
            Ok(Some(self.deserialize(value)?))
        } else {
            Ok(None)
        }
    }

    pub async fn get_all_events(&self) -> Result<Vec<Event>> {
        let mut events = Vec::new();
        let prefix = "event:";
        
        for item in self.db.scan_prefix(prefix) {
            let (_, value) = item?;
            let event: Event = self.deserialize(value)?;
            events.push(event);
        }
        
        // Sort by start_date
        events.sort_by(|a, b| a.start_date.cmp(&b.start_date));
        Ok(events)
    }

    // Packages
    pub async fn create_package(&self, package: &Package) -> Result<()> {
        let key = format!("package:{}", package.id);
        let value = self.serialize(package)?;
        self.db.insert(key, value)?;
        Ok(())
    }

    pub async fn get_all_packages(&self) -> Result<Vec<Package>> {
        let mut packages = Vec::new();
        let prefix = "package:";
        
        for item in self.db.scan_prefix(prefix) {
            let (_, value) = item?;
            let package: Package = self.deserialize(value)?;
            packages.push(package);
        }
        
        packages.sort_by(|a, b| a.price.partial_cmp(&b.price).unwrap_or(std::cmp::Ordering::Equal));
        Ok(packages)
    }

    // Registrations
    pub async fn create_registration(&self, registration: &Registration) -> Result<()> {
        let key = format!("registration:{}", registration.id);
        let value = self.serialize(registration)?;
        self.db.insert(key, value)?;
        Ok(())
    }

    pub async fn get_user_registrations(&self, user_id: &Uuid) -> Result<Vec<Registration>> {
        let mut registrations = Vec::new();
        let prefix = "registration:";
        
        for item in self.db.scan_prefix(prefix) {
            let (_, value) = item?;
            let registration: Registration = self.deserialize(value)?;
            if registration.user_id == *user_id {
                registrations.push(registration);
            }
        }
        
        registrations.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        Ok(registrations)
    }

    // Venues
    pub async fn create_venue(&self, venue: &Venue) -> Result<()> {
        let key = format!("venue:{}", venue.id);
        let value = self.serialize(venue)?;
        self.db.insert(key, value)?;
        Ok(())
    }

    pub async fn get_all_venues(&self) -> Result<Vec<Venue>> {
        let mut venues = Vec::new();
        let prefix = "venue:";
        
        for item in self.db.scan_prefix(prefix) {
            let (_, value) = item?;
            let venue: Venue = self.deserialize(value)?;
            venues.push(venue);
        }
        
        venues.sort_by(|a, b| a.name.cmp(&b.name));
        Ok(venues)
    }

    // Initialize with sample data
    pub async fn init_sample_data(&self) -> Result<()> {
        // Check if data already exists
        if self.db.scan_prefix("event:").next().is_some() {
            return Ok(()); // Data already exists
        }

        // Create sample venues
        let venue1 = Venue {
            id: Uuid::new_v4(),
            name: "Dance Studio Berlin".to_string(),
            address: "Mitte, Berlin, Germany".to_string(),
            capacity: 50,
            description: Some("A beautiful dance studio in the heart of Berlin".to_string()),
        };

        let venue2 = Venue {
            id: Uuid::new_v4(),
            name: "Community Center Kreuzberg".to_string(),
            address: "Kreuzberg, Berlin, Germany".to_string(),
            capacity: 80,
            description: Some("Spacious community center perfect for dance events".to_string()),
        };

        self.create_venue(&venue1).await?;
        self.create_venue(&venue2).await?;

        // Create sample packages
        let packages = vec![
            Package {
                id: Uuid::new_v4(),
                name: "Beginner Blues".to_string(),
                description: "Perfect introduction to Blues dancing".to_string(),
                price: 45.0,
                duration_days: 1,
                max_participants: 20,
            },
            Package {
                id: Uuid::new_v4(),
                name: "Fusion Experience".to_string(),
                description: "Explore the fusion of different dance styles".to_string(),
                price: 85.0,
                duration_days: 2,
                max_participants: 25,
            },
            Package {
                id: Uuid::new_v4(),
                name: "Blues & Fusion Immersion".to_string(),
                description: "Deep dive into Blues and Fusion dancing".to_string(),
                price: 180.0,
                duration_days: 5,
                max_participants: 15,
            },
        ];

        for package in packages {
            self.create_package(&package).await?;
        }

        // Create sample events based on the website content
        let events = vec![
            Event {
                id: Uuid::new_v4(),
                title: "Blues & Fusion Alumni Training".to_string(),
                description: "Advanced training for experienced dancers".to_string(),
                start_date: chrono::NaiveDate::from_ymd_opt(2025, 9, 25)
                    .unwrap()
                    .and_hms_opt(18, 0, 0)
                    .unwrap()
                    .and_utc(),
                end_date: chrono::NaiveDate::from_ymd_opt(2025, 10, 1)
                    .unwrap()
                    .and_hms_opt(21, 0, 0)
                    .unwrap()
                    .and_utc(),
                venue_id: venue1.id,
                max_participants: 15,
                current_participants: 8,
                price: 180.0,
                event_type: EventType::Workshop,
            },
            Event {
                id: Uuid::new_v4(),
                title: "Blues & Fusion Experience".to_string(),
                description: "Weekend festival celebrating Blues and Fusion dance".to_string(),
                start_date: chrono::NaiveDate::from_ymd_opt(2025, 10, 2)
                    .unwrap()
                    .and_hms_opt(19, 0, 0)
                    .unwrap()
                    .and_utc(),
                end_date: chrono::NaiveDate::from_ymd_opt(2025, 10, 5)
                    .unwrap()
                    .and_hms_opt(23, 0, 0)
                    .unwrap()
                    .and_utc(),
                venue_id: venue2.id,
                max_participants: 80,
                current_participants: 45,
                price: 85.0,
                event_type: EventType::Festival,
            },
            Event {
                id: Uuid::new_v4(),
                title: "Movement & Blues Immersion".to_string(),
                description: "Intensive week-long immersion in movement and blues".to_string(),
                start_date: chrono::NaiveDate::from_ymd_opt(2025, 10, 6)
                    .unwrap()
                    .and_hms_opt(10, 0, 0)
                    .unwrap()
                    .and_utc(),
                end_date: chrono::NaiveDate::from_ymd_opt(2025, 10, 12)
                    .unwrap()
                    .and_hms_opt(18, 0, 0)
                    .unwrap()
                    .and_utc(),
                venue_id: venue1.id,
                max_participants: 20,
                current_participants: 12,
                price: 280.0,
                event_type: EventType::Intensive,
            },
        ];

        for event in events {
            self.create_event(&event).await?;
        }

        log::info!("Sample data initialized successfully");
        Ok(())
    }
}