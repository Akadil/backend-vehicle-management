//! Represents a maintenance type in the system.
//!
//! *************************************** 100 chars limit ****************************************
//! # General rules:
//! * Maintenance types are used to categorize different maintenance actions (e.g., Oil Change, Tire
//!   Rotation).
//!
//! # Use cases:
//! 1. Show the list of available maintenance types in the system.
//!
use crate::user::entities::user::UserIdentity;

#[derive(Debug, Clone)]
pub struct MaintenanceType {
    /// The name of the maintenance type (e.g., Oil Change, Tire Rotation).
    name: String,
    /// Description of the maintenance type.
    description: String,
}

#[derive(Debug, Clone)]
pub struct MaintenanceTypeView {
    /// The unique identifier for the maintenance type.
    pub id: i32,
    /// The name of the maintenance type (e.g., Oil Change, Tire Rotation).
    pub name: String,
    /// Description of the maintenance type.
    pub description: String,
    /// Created at timestamp.
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Created by user ID.
    pub created_by: UserIdentity,
    /// Updated at timestamp.
    pub updated_at: chrono::DateTime<chrono::Utc>,
    /// Updated by user ID.
    pub updated_by: UserIdentity,
}

impl MaintenanceType {
    /// Creates a new instance of `MaintenanceType`.
    ///
    /// # Arguments
    /// * `name` - The name of the maintenance type.
    /// * `description` - The description of the maintenance type.
    pub fn new(name: String, description: String) -> Result<Self, MaintenanceTypeError> {
        if name.is_empty() {
            return Err(MaintenanceTypeError::EmptyField);
        }
        Ok(MaintenanceType { name, description })
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

#[derive(Debug, thiserror::Error)]
pub enum MaintenanceTypeError {
    #[error("Field is empty")]
    EmptyField,
    #[error("Invalid data: {0}")]
    InvalidData(String),
}
