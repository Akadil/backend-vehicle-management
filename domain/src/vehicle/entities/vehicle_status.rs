//! Represents the status log of a vehicle in the system.
use crate::user::entities::user::UserSummary;
use crate::vehicle::entities::vehicle::VehicleIdentity;

#[derive(Debug, Clone)]
pub struct VehicleStatus {
    /// The unique identifier for the vehicle status.
    pub id: i32,
    /// The unique identifier for the vehicle.
    pub vehicle: VehicleIdentity,
    /// The unique identifier for the user who created this status.
    pub user: UserSummary,
    /// Odometer reading at the time of the status.
    pub odometer: Option<i32>,
    /// The engine hour meter reading at the time of the status.
    pub engine_hour_meter: Option<i32>,
    /// The current fuel level of the vehicle.
    pub fuel_level: Option<i32>,
    /// Created at timestamp.
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Some notes or comments about the vehicle status.
    pub notes: Option<String>,
}

#[derive(Debug, Clone)]
pub struct VehicleStatusSummary {
    /// The unique identifier for the vehicle status.
    pub id: i32,
    /// The unique identifier for the vehicle.
    pub vehicle_id: i32,
    /// The unique identifier for the user who created this status.
    pub user_id: i32,
}
