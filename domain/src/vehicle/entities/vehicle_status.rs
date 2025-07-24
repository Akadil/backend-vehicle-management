//! Represents the status log of a vehicle in the system.
//! 
//! *************************************** 100 chars limit ****************************************
//! # General rules:
//! * Vehicle status is always associated with a vehicle and a user who performed the action.
//! * It should be independent from maintenance logic.
//! 
//! 
use crate::user::entities::user::UserIdentity;
use crate::vehicle::entities::vehicle::VehicleIdentity;

/// Represents the identity of a vehicle status (DB record, non-hydrated).
#[derive(Debug, Clone)]
pub struct VehicleStatusIdentity {
    /// The unique identifier for the vehicle status.
    pub id: i32,
    /// The unique identifier for the vehicle.
    pub vehicle_id: uuid::Uuid,
    /// The unique identifier for the user who created this status.
    pub performed_by: uuid::Uuid,

    /// The timestamp when the status was performed.
    pub performed_at: chrono::DateTime<chrono::Utc>,
    /// Odometer reading at the time of the status.
    pub odometer: i32,
    /// The engine hour meter reading at the time of the status.
    pub engine_hour_meter: Option<i32>,
    /// The current fuel level of the vehicle.
    pub fuel_level: Option<i32>,
    /// Some notes or comments about the vehicle status.
    pub notes: String,

    /// Created at timestamp.
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Updated at timestamp.
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// Represents a hydrated version of a vehicle status.
#[derive(Debug, Clone)]
pub struct VehicleStatus {
    /// The identity details of the vehicle status.
    pub identity: VehicleStatusIdentity,
    /// The unique identifier for the vehicle.
    pub vehicle: VehicleIdentity,
    /// The unique identifier for the user who created this status.
    pub performed_by: UserIdentity,
}
