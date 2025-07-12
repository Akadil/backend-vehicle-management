//! Represents a maintenance type in the system.
//! 
//! Maintenance types are used to categorize different maintenance actions (e.g., Oil Change, Tire 
//!   Rotation).
//! 
//! *************************************** 100 chars limit ****************************************
//! # Personal notes:
//! * 
//! 
//! # Business logic:
//! 1. Show the list of available maintenance types in the system.
//!

#[derive(Debug, Clone)]
pub struct MaintenanceType {
    /// The unique identifier for the maintenance type.
    id: i32,
    /// The name of the maintenance type (e.g., Oil Change, Tire Rotation).
    name: String,
    /// Description of the maintenance type.
    description: String,
    /// Created at timestamp.
    created_at: chrono::DateTime<chrono::Utc>,
    /// Updated at timestamp.
    updated_at: chrono::DateTime<chrono::Utc>,
}

impl MaintenanceType {
    /// Creates a new instance of `MaintenanceType`.
    ///
    /// # Arguments
    /// * `name` - The name of the maintenance type.
    /// * `description` - The description of the maintenance type.
    pub fn new(name: String, description: String) -> Self {
        MaintenanceType {
            id: 0, // ID will be set by the database
            name,
            description,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        }
    }

    /* Getters */
    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    /* Setters */
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn set_description(&mut self, description: String) {
        self.description = description;
    }
}
