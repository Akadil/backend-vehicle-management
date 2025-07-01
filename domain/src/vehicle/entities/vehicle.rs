/// Represents a vehicle in the system.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Vehicle {
    /// The unique identifier for the vehicle.
    pub id: i32,
    /// Uuid of the vehicle.
    pub uuid: uuid::Uuid,
    /// The make of the vehicle (e.g., Toyota, Ford).
    pub make: String,
    /// The model of the vehicle (e.g., Camry, Focus).
    pub model: String,
    /// The year the vehicle was manufactured.
    pub year: u16,
    /// The Vehicle Identification Number (VIN) of the vehicle.
    pub vin: String,
    /// License plate number of the vehicle.
    pub license_plate: String,
    /// The engine type of the vehicle (e.g., Gasoline, Diesel, Electric).
    pub engine_type: EngineType,
    /// How the maintenance should be tracked: kilometer based or hour based.
    pub tracking_type: TrackingType,
    /// Initial odometer reading of the vehicle.
    pub initial_odometer: i32,
    /// Initial engine hour meter reading of the vehicle.
    pub initial_engine_hour_meter: i32,
    /// created_at timestamp
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// updated_at timestamp
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// Represents the method of maintenance tracking for a vehicle.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TrackingType {
    Kilometers,
    EngineHours,
}

/// Represents the type of engine a vehicle can have.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EngineType {
    Gasoline,
    Diesel,
    Electric,
}

impl Vehicle {
    pub fn full_name(&self) -> String {
        format!("{} {} ({})", self.make, self.model, self.year)
    }
}
