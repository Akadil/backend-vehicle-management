/// Represents a view of a vehicle with its full details.
#[derive(Debug, Clone)]
pub struct VehicleView {
    /// id and Uuid of the vehicle.
    pub id: String,
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
    pub engine_type: String,
    /// The date and time when the vehicle was created.
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// The date and time when the vehicle was last updated.
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
