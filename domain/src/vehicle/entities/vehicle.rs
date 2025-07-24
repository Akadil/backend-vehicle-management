//! Represents a vehicle in the system
//! 
//! *************************************** 100 chars limit **************************************** 
//! # General rules:
//! 
//! 
//! # TODO list:
//! * Add validation rules for the make, model, year
//! 
use chrono::Datelike;

use crate::vehicle::{
    entities::vehicle_status::VehicleStatusIdentity,
    value_types::{engine_type, license_plate, vehicle_vin},
};

/// Represents the identity of a vehicle (DB record, non-hydrated).
#[derive(Debug, Clone)]
pub struct VehicleIdentity {
    /// id and Uuid of the vehicle.
    pub id: uuid::Uuid,
    /// The make of the vehicle (e.g., Toyota, Ford).
    pub make: String,
    /// The model of the vehicle (e.g., Camry, Focus).
    pub model: String,
    /// The year the vehicle was manufactured.
    pub year: u16,
    /// The Vehicle Identification Number (VIN) of the vehicle.
    pub vin: vehicle_vin::VehicleVin,
    /// License plate number of the vehicle.
    pub license_plate: license_plate::LicensePlate,
    /// The engine type of the vehicle (e.g., Gasoline, Diesel, Electric).
    pub engine_type: engine_type::EngineType,
    /// The date and time when the vehicle was created.
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// The date and time when the vehicle was last updated.
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

<<<<<<< HEAD
// #[derive(Debug, Clone)]
// pub struct VehicleView {
//     /// id and Uuid of the vehicle.
//     pub id: uuid::Uuid,
//     /// The make of the vehicle (e.g., Toyota, Ford).
//     pub make: String,
//     /// The model of the vehicle (e.g., Camry, Focus).
//     pub model: String,
//     /// The year the vehicle was manufactured.
//     pub year: u16,
//     /// The Vehicle Identification Number (VIN) of the vehicle.
//     pub vin: vehicle_vin::VehicleVin,
//     /// License plate number of the vehicle.
//     pub license_plate: license_plate::LicensePlate,
//     /// The engine type of the vehicle (e.g., Gasoline, Diesel, Electric).
//     pub engine_type: engine_type::EngineType,
//     /// The date and time when the vehicle was created.
//     pub created_at: chrono::DateTime<chrono::Utc>,
//     /// The date and time when the vehicle was last updated.
//     pub updated_at: chrono::DateTime<chrono::Utc>,
// }

=======
/// Represents a hydrated version of a vehicle.
>>>>>>> 78a87e9cb86dea90d76570956f1bc7e35f224ca1
#[derive(Debug, Clone)]
pub struct Vehicle {
    /// vehicle identity containing all the fields of the vehicle.
    pub identity: VehicleIdentity,
    /// The latest status of the vehicle.
    pub latest_status: Option<VehicleStatusIdentity>,
}

impl Vehicle {
    /// Creates a new vehicle with the provided identity and latest status.
    pub fn new(data: NewVehicle) -> Result<Vehicle, VehicleError> {
        let identity = VehicleIdentity {
            id: uuid::Uuid::new_v4(),
            make: data.make,
            model: data.model,
            year: data.year,
            vin: vehicle_vin::VehicleVin::new(data.vin)?,
            license_plate: license_plate::LicensePlate::new(data.license_plate)?,
            engine_type: engine_type::EngineType::new(data.engine_type)?,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };

        Vehicle {
            identity,
            latest_status: None,
        }
    }

    // access all fields of the vehicle with getters through the identity
    pub fn uuid(&self) -> &uuid::Uuid {
        &self.identity.id
    }
    pub fn make(&self) -> &str {
        &self.identity.make
    }
    pub fn model(&self) -> &str {
        &self.identity.model
    }
    pub fn year(&self) -> u16 {
        self.identity.year
    }
    pub fn vin(&self) -> &vehicle_vin::VehicleVin {
        &self.identity.vin
    }
    pub fn license_plate(&self) -> &license_plate::LicensePlate {
        &self.identity.license_plate
    }
    pub fn engine_type(&self) -> &engine_type::EngineType {
        &self.identity.engine_type
    }
    pub fn created_at(&self) -> chrono::DateTime<chrono::Utc> {
        self.identity.created_at
    }
    pub fn updated_at(&self) -> chrono::DateTime<chrono::Utc> {
        self.identity.updated_at
    }
}

#[derive(Debug, thiserror::Error)]
pub enum VehicleError {
    #[error("Invalid year: {0}")]
    InvalidYear(u16),
    #[error("Invalid VIN: {0}")]
    InvalidVin(#[from] vehicle_vin::VehicleVinError),
    #[error("Invalid license plate: {0}")]
    InvalidLicensePlate(#[from] license_plate::LicensePlateError),
    #[error("Invalid engine type: {0}")]
    InvalidEngineType(#[from] engine_type::EngineTypeError),
}

#[derive(Debug, Clone)]
pub struct NewVehicle {
    pub make: String,
    pub model: String,
    pub year: u16,
    pub vin: String,
    pub license_plate: String,
    pub engine_type: String,
}
