pub struct CreateVehicleCommand {
    pub make: String,
    pub model: String,
    pub year: u16,
    pub vin: String,
    pub license_plate: String,
    pub engine_type: String,
    // user (caller) info
    pub user_id: String,
}

pub struct CreateVehicleResponse {
    pub id: String,
    pub make: String,
    pub model: String,
    pub year: u16,
    pub vin: String,
    pub license_plate: String,
    pub engine_type: String,
    pub created_at: String,
}
