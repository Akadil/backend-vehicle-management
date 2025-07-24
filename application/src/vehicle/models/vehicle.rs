#[derive(Debug, Clone)]
pub struct VehicleView {
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