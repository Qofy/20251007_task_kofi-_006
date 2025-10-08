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

    pub async fn increment_event_participants(&self, event_id: &Uuid) -> Result<()> {
        if let Some(mut event) = self.get_event_by_id(event_id).await? {
            event.current_participants += 1;
            let key = format!("event:{}", event.id);
            let value = self.serialize(&event)?;
            self.db.insert(key, value)?;
        }
        Ok(())
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

    // Bulk data operations for comprehensive data management
    pub async fn export_all_data(&self) -> Result<DatabaseExport> {
        let users = self.get_all_users().await?;
        let events = self.get_all_events().await?;
        let venues = self.get_all_venues().await?;
        let packages = self.get_all_packages().await?;
        let registrations = self.get_all_registrations().await?;

        Ok(DatabaseExport {
            users,
            events,
            venues,
            packages,
            registrations,
            exported_at: Utc::now(),
            version: "1.0".to_string(),
        })
    }

    pub async fn import_all_data(&self, data: &DatabaseExport) -> Result<()> {
        // Clear existing data first (optional)
        log::info!("Importing {} users, {} events, {} venues, {} packages, {} registrations", 
                   data.users.len(), data.events.len(), data.venues.len(), 
                   data.packages.len(), data.registrations.len());

        // Import venues first (events depend on venues)
        for venue in &data.venues {
            self.create_venue(venue).await?;
        }

        // Import users
        for user in &data.users {
            self.create_user(user).await?;
        }

        // Import packages
        for package in &data.packages {
            self.create_package(package).await?;
        }

        // Import events
        for event in &data.events {
            self.create_event(event).await?;
        }

        // Import registrations
        for registration in &data.registrations {
            self.create_registration(registration).await?;
        }

        log::info!("Data import completed successfully");
        Ok(())
    }

    pub async fn get_all_users(&self) -> Result<Vec<User>> {
        let mut users = Vec::new();
        let prefix = "user:";
        
        for item in self.db.scan_prefix(prefix) {
            let (_, value) = item?;
            let user: User = self.deserialize(value)?;
            users.push(user);
        }
        
        users.sort_by(|a, b| a.created_at.cmp(&b.created_at));
        Ok(users)
    }

    pub async fn get_all_registrations(&self) -> Result<Vec<Registration>> {
        let mut registrations = Vec::new();
        let prefix = "registration:";
        
        for item in self.db.scan_prefix(prefix) {
            let (_, value) = item?;
            let registration: Registration = self.deserialize(value)?;
            registrations.push(registration);
        }
        
        registrations.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        Ok(registrations)
    }

    pub async fn clear_all_data(&self) -> Result<()> {
        self.db.clear()?;
        log::info!("All data cleared successfully");
        Ok(())
    }

    pub async fn get_data_statistics(&self) -> Result<DataStatistics> {
        let users_count = self.db.scan_prefix("user:").count();
        let events_count = self.db.scan_prefix("event:").count();
        let venues_count = self.db.scan_prefix("venue:").count();
        let packages_count = self.db.scan_prefix("package:").count();
        let registrations_count = self.db.scan_prefix("registration:").count();

        Ok(DataStatistics {
            users: users_count,
            events: events_count,
            venues: venues_count,
            packages: packages_count,
            registrations: registrations_count,
            total_records: users_count + events_count + venues_count + packages_count + registrations_count,
            last_updated: Utc::now(),
        })
    }

    // Initialize with sample data
    pub async fn init_sample_data(&self) -> Result<()> {
        // Check if data already exists
        if self.db.scan_prefix("event:").next().is_some() {
            return Ok(()); // Data already exists
        }

        // Create sample venues for electronic music events
        let venue1 = Venue {
            id: Uuid::new_v4(),
            name: "Berghain".to_string(),
            address: "Am Wriezener Bahnhof, 10243 Berlin, Germany".to_string(),
            capacity: 1500,
            description: Some("World-famous techno temple, the mecca of electronic music".to_string()),
        };

        let venue2 = Venue {
            id: Uuid::new_v4(),
            name: "Watergate".to_string(),
            address: "Falckensteinstraße 49, 10997 Berlin, Germany".to_string(),
            capacity: 300,
            description: Some("Sophisticated club with panoramic views of the Spree River".to_string()),
        };

        let venue3 = Venue {
            id: Uuid::new_v4(),
            name: "Tresor".to_string(),
            address: "Köpenicker Str. 70, 10179 Berlin, Germany".to_string(),
            capacity: 800,
            description: Some("Legendary underground techno club in a former bank vault".to_string()),
        };

        let venue4 = Venue {
            id: Uuid::new_v4(),
            name: "Sisyphos".to_string(),
            address: "Hauptstraße 15, 10317 Berlin, Germany".to_string(),
            capacity: 1000,
            description: Some("Outdoor electronic music venue with multiple dance floors".to_string()),
        };

        self.create_venue(&venue1).await?;
        self.create_venue(&venue2).await?;
        self.create_venue(&venue3).await?;
        self.create_venue(&venue4).await?;

        // Create sample packages for electronic music experiences
        let packages = vec![
            Package {
                id: Uuid::new_v4(),
                name: "Underground Initiation".to_string(),
                description: "Your first taste of Berlin's underground electronic scene. Perfect for newcomers who want to experience authentic techno culture.".to_string(),
                price: 75.0,
                duration_days: 1,
                max_participants: 15,
            },
            Package {
                id: Uuid::new_v4(),
                name: "Techno Temple Tour".to_string(),
                description: "Experience the legendary venues that shaped electronic music history. Includes skip-the-line access and expert guide.".to_string(),
                price: 145.0,
                duration_days: 2,
                max_participants: 12,
            },
            Package {
                id: Uuid::new_v4(),
                name: "Berlin Electronic Marathon".to_string(),
                description: "The ultimate electronic music journey. 48 hours of non-stop parties across Berlin's most iconic venues.".to_string(),
                price: 280.0,
                duration_days: 3,
                max_participants: 10,
            },
            Package {
                id: Uuid::new_v4(),
                name: "VIP Scene Access".to_string(),
                description: "Exclusive access to private parties, meet top DJs, and experience Berlin's electronic scene like an insider.".to_string(),
                price: 450.0,
                duration_days: 5,
                max_participants: 8,
            },
        ];

        for package in packages {
            self.create_package(&package).await?;
        }

        // Create sample events for electronic music
        let events = vec![
            Event {
                id: Uuid::new_v4(),
                title: "Berghain Masterclass".to_string(),
                description: "Learn the secrets of getting into Berlin's most exclusive club. Includes dress code guidance, cultural etiquette, and guaranteed entry.".to_string(),
                start_date: chrono::NaiveDate::from_ymd_opt(2025, 10, 15)
                    .unwrap()
                    .and_hms_opt(23, 0, 0)
                    .unwrap()
                    .and_utc(),
                end_date: chrono::NaiveDate::from_ymd_opt(2025, 10, 16)
                    .unwrap()
                    .and_hms_opt(6, 0, 0)
                    .unwrap()
                    .and_utc(),
                venue_id: venue1.id,
                max_participants: 20,
                current_participants: 14,
                price: 120.0,
                event_type: EventType::Workshop,
            },
            Event {
                id: Uuid::new_v4(),
                title: "Warehouse Rave Experience".to_string(),
                description: "Authentic underground warehouse party featuring top international DJs. Raw techno in its purest form.".to_string(),
                start_date: chrono::NaiveDate::from_ymd_opt(2025, 10, 18)
                    .unwrap()
                    .and_hms_opt(22, 0, 0)
                    .unwrap()
                    .and_utc(),
                end_date: chrono::NaiveDate::from_ymd_opt(2025, 10, 19)
                    .unwrap()
                    .and_hms_opt(8, 0, 0)
                    .unwrap()
                    .and_utc(),
                venue_id: venue4.id,
                max_participants: 100,
                current_participants: 67,
                price: 35.0,
                event_type: EventType::Social,
            },
            Event {
                id: Uuid::new_v4(),
                title: "Techno Festival Weekend".to_string(),
                description: "Three-day electronic music festival featuring 50+ artists across multiple venues. The ultimate Berlin electronic experience.".to_string(),
                start_date: chrono::NaiveDate::from_ymd_opt(2025, 10, 25)
                    .unwrap()
                    .and_hms_opt(18, 0, 0)
                    .unwrap()
                    .and_utc(),
                end_date: chrono::NaiveDate::from_ymd_opt(2025, 10, 28)
                    .unwrap()
                    .and_hms_opt(6, 0, 0)
                    .unwrap()
                    .and_utc(),
                venue_id: venue2.id,
                max_participants: 500,
                current_participants: 234,
                price: 185.0,
                event_type: EventType::Festival,
            },
            Event {
                id: Uuid::new_v4(),
                title: "Electronic Music Production Intensive".to_string(),
                description: "Week-long intensive course in electronic music production. Learn from Berlin's top producers in professional studios.".to_string(),
                start_date: chrono::NaiveDate::from_ymd_opt(2025, 11, 1)
                    .unwrap()
                    .and_hms_opt(10, 0, 0)
                    .unwrap()
                    .and_utc(),
                end_date: chrono::NaiveDate::from_ymd_opt(2025, 11, 8)
                    .unwrap()
                    .and_hms_opt(18, 0, 0)
                    .unwrap()
                    .and_utc(),
                venue_id: venue3.id,
                max_participants: 12,
                current_participants: 8,
                price: 680.0,
                event_type: EventType::Intensive,
            },
            Event {
                id: Uuid::new_v4(),
                title: "DJ Competition Finals".to_string(),
                description: "Annual DJ competition finale. Watch emerging talent compete for a chance to play at Berlin's top venues.".to_string(),
                start_date: chrono::NaiveDate::from_ymd_opt(2025, 11, 12)
                    .unwrap()
                    .and_hms_opt(20, 0, 0)
                    .unwrap()
                    .and_utc(),
                end_date: chrono::NaiveDate::from_ymd_opt(2025, 11, 13)
                    .unwrap()
                    .and_hms_opt(4, 0, 0)
                    .unwrap()
                    .and_utc(),
                venue_id: venue2.id,
                max_participants: 300,
                current_participants: 189,
                price: 25.0,
                event_type: EventType::Competition,
            },
            Event {
                id: Uuid::new_v4(),
                title: "Rooftop Sunrise Sessions".to_string(),
                description: "Experience Berlin's famous sunrise sessions on a rooftop venue. House and techno as the city wakes up.".to_string(),
                start_date: chrono::NaiveDate::from_ymd_opt(2025, 11, 15)
                    .unwrap()
                    .and_hms_opt(4, 0, 0)
                    .unwrap()
                    .and_utc(),
                end_date: chrono::NaiveDate::from_ymd_opt(2025, 11, 15)
                    .unwrap()
                    .and_hms_opt(12, 0, 0)
                    .unwrap()
                    .and_utc(),
                venue_id: venue4.id,
                max_participants: 80,
                current_participants: 52,
                price: 45.0,
                event_type: EventType::Social,
            },
        ];

        for event in events {
            self.create_event(&event).await?;
        }

        log::info!("Sample electronic music data initialized successfully");
        Ok(())
    }
}