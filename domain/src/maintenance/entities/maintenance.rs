//! Represents a maintenance rules of a certain vehicle and maintenance type in the system.
//! 
//! Domain layer entities are used to represent the core business logic and rules.
//! 
//! *************************************** 100 chars limit ****************************************
//! Personal notes:
//! ---
//! * Actually, it's more like maintenance rule than maintenance itself
use crate::{
    maintenance::{
        entities::maintenance_type::MaintenanceType,
        value_types::maintenance_interval_type::MaintenanceIntervalType,
    },
    vehicle::entities::vehicle::VehicleIdentity,
    user::entities::user::UserIdentity,
};

/// Represents the identity of a maintenance (DB record, non-hydrated).
#[derive(Debug, Clone)]
pub struct MaintenanceIdentity {
    /// The unique identifier for the maintenance record.
    pub id: i32,
    /// The unique identifier for the vehicle associated with this maintenance.
    pub vehicle_id: uuid::Uuid,
    /// The type of maintenance being performed.
    pub maintenance_type_id: i32,
    
    /// The interval type for the maintenance (e.g., kilometers, hours, by date).
    pub interval_type: MaintenanceIntervalType,
    /// Interval value (e.g., 10000 km, 500 hours, 1 year).
    pub interval_value: u32,
    /// Threshold (Red) value in percentage (e.g., 95%).
    pub red_threshold: u32,
    /// Threshold (Yellow) value in percentage (e.g., 80%).
    pub yellow_threshold: u32,

    /// Created at timestamp.
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Create by user ID.
    pub created_by: uuid::Uuid,
    /// Updated at timestamp.
    pub updated_at: chrono::DateTime<chrono::Utc>,
    /// Updated by user ID.
    pub updated_by: uuid::Uuid,
}

/// Represents a hydrated version of a maintenance
#[derive(Debug, Clone)]
pub struct Maintenance {
    /// maintenance identity details
    pub identity: MaintenanceIdentity,
    /// The type of maintenance being performed.
    pub maintenance_type: MaintenanceType,
    /// The vehicle associated with this maintenance.
    pub vehicle: VehicleIdentity,
}

impl Maintenance {
    /// Creates a new instance of `Maintenance`.
    pub fn new(
        maintenance_type: MaintenanceType,
        vehicle: VehicleIdentity,
        created_by: UserIdentity,
        data: NewMaintenance,
    ) -> Result<Self, MaintenanceError>  {
        if (data.red_threshold < 0 || data.red_threshold > 100) || (data.yellow_threshold < 0 || data.yellow_threshold > data.red_threshold) {
            return Err(MaintenanceError::InvalidThreshold(
                "Threshold values must be between 0 and 100".to_string(),
            ));
        }

        Ok(Self {
            identity: MaintenanceIdentity {
                id: 0, // This will be set by the database
                vehicle_id: vehicle.id,
                maintenance_type_id: maintenance_type.id(),
                interval_type: MaintenanceIntervalType::from_str(&data.interval_type).ok_or(
                    MaintenanceError::UnknownIntervalType(data.interval_type.clone()),
                )?,
                interval_value: data.interval_value,
                red_threshold: data.red_threshold,
                yellow_threshold: data.yellow_threshold,
                created_at: chrono::Utc::now(),
                created_by: created_by.id,
                updated_at: chrono::Utc::now(),
                updated_by: created_by.id, // Initially set to the creator
            },
            maintenance_type,
            vehicle,
        })
    }
}

pub struct NewMaintenance {
    /// The interval type for the maintenance (e.g., kilometers, hours, by date).
    pub interval_type: String,
    /// Interval value (e.g., 10000 km, 500 hours, 1 year).
    pub interval_value: u32,
    /// Threshold (Red) value in percentage (e.g., 95%).
    pub red_threshold: u32,
    /// Threshold (Yellow) value in percentage (e.g., 80%).
    pub yellow_threshold: u32,
}

#[derive(Debug, thiserror::Error)]
pub enum MaintenanceError {
    #[error("Invalid maintenance data: {0}")]
    InvalidThreshold(String),
    #[error("Unknown maintenance interval type: {0}")]
    UnknownIntervalType(String),
}