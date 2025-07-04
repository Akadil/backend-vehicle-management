use chrono::Datelike;

/// Represents a vehicle in the system.
use crate::vehicle::{
    entities::vehicle_status::VehicleStatusSummary,
    value_types::{engine_type, license_plate, vehicle_vin},
};

#[derive(Debug, Clone)]
pub struct VehicleIdentity {
    /// id and Uuid of the vehicle.
    pub uuid: uuid::Uuid,
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

#[derive(Debug, Clone)]
pub struct Vehicle {
    /// vehicle identity containing all the fields of the vehicle.
    pub identity: VehicleIdentity,
    /// The latest status of the vehicle.
    pub latest_status: Option<VehicleStatusSummary>,
}

/// Represents a new vehicle to be created in the system.
#[derive(Debug, Clone)]
pub struct NewVehicle {
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

impl NewVehicle {
    /// Creates a new instance of `NewVehicle`.
    pub fn new(
        make: String,
        model: String,
        year: u16,
        vin: String,
        license_plate: String,
        engine_type: String,
    ) -> Result<Self, VehicleError> {
        if year < 1950 || year > chrono::Utc::now().year_ce().1 as u16 {
            return Err(VehicleError::InvalidYear(year));
        }
        Ok(NewVehicle {
            make,
            model,
            year,
            vin: vehicle_vin::VehicleVin::new(vin)?,
            license_plate: license_plate::LicensePlate::new(license_plate)?,
            engine_type: engine_type::EngineType::new(engine_type)?,
        })
    }
}

impl Vehicle {
    // access all fields of the vehicle with getters through the identity
    pub fn uuid(&self) -> &uuid::Uuid {
        &self.identity.uuid
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
