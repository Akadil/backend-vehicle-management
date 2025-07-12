//! Represents an user in the system.
use crate::user::value_types::{Email, UserId};
use crate::vehicle::entities::vehicle::VehicleIdentity;

#[derive(Debug, Clone)]
pub struct UserIdentity {
    /// The unique identifier for the user.
    pub id: uuid::Uuid,
    /// Uuid of the user.
    pub uuid: UserId,
    /// The username of the user.
    pub username: String,
    /// The email of the user.
    pub email: Email,
    /// First name of the user.
    pub first_name: String,
    /// Last name of the user.
    pub last_name: String,
}

#[derive(Debug, Clone)]
pub struct UserDriver {
    /// The unique identifier for the driver.
    pub id: uuid::Uuid,
    /// Uuid of the driver.
    pub uuid: uuid::Uuid,
    /// The vehicle assigned to the driver.
    pub vehicle: Vec<VehicleIdentity>,
}
