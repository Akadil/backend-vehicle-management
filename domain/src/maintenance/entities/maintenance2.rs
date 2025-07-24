//! Represents a maintenance details of a certain vehicle and maintenance type in the system.
//! 
//! *************************************** 100 chars limit ****************************************
use crate::{
    maintenance::{
        entities::maintenance_type::MaintenanceType,
        value_types::maintenance_interval_type::MaintenanceIntervalType,
    },
    vehicle::entities::vehicle::VehicleIdentity,
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
    pub interval_value: i32,
    /// Threshold (Red) value in percentage (e.g., 95%).
    pub red_threshold: i32,
    /// Threshold (Yellow) value in percentage (e.g., 80%).
    pub yellow_threshold: i32,
    /// Created at timestamp.
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Updated at timestamp.
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// Represents a hydrated version of a maintenance
/// 
/// General rules:
/// * 
/// 
/// Use case: 
/// 1. 
#[derive(Debug, Clone)]
pub struct Maintenance {
    /// maintenance identity details
    pub identity: MaintenanceIdentity,
    /// The type of maintenance being performed.
    pub maintenance_type: MaintenanceType,
    /// The vehicle associated with this maintenance.
    pub vehicle: VehicleIdentity,
}
