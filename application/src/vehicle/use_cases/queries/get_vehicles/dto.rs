// use crate::shared::pagination::{SortOrder, DEFAULT_PAGE, DEFAULT_PAGE_SIZE, MAX_PAGE_SIZE};
use crate::vehicle::models::vehicle::VehicleView;

#[derive(Debug, Clone)]
pub struct VehicleResponse {
    pub id: String,
    pub make: String,
    pub model: String,
    pub year: u16,
    pub vin: String,
    pub license_plate: String,
    pub engine_type: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone)]
pub struct GetVehiclesResponse {
    pub vehicles: Vec<VehicleResponse>,
    pub total_count: usize,
    pub page: u32,
    pub page_size: u32,
    pub total_pages: u32,
}

// impl GetVehiclesResponse {
//     pub fn new(vehicles: Vec<VehicleResponse>, total_count: usize, page: u32, page_size: u32) -> Self {
//         let total_pages = if total_count == 0 {
//             1
//         } else {
//             ((total_count as f64) / (page_size as f64)).ceil() as u32
//         };

//         Self {
//             vehicles,
//             total_count,
//             page,
//             page_size,
//             total_pages,
//         }
//     }
// }

impl From<VehicleView> for VehicleResponse {
    fn from(vehicle: VehicleView) -> Self {
        VehicleResponse {
            id: vehicle.id,
            make: vehicle.make,
            model: vehicle.model,
            year: vehicle.year,
            vin: vehicle.vin,
            license_plate: vehicle.license_plate,
            engine_type: vehicle.engine_type,
            created_at: vehicle.created_at.to_rfc3339(),
            updated_at: vehicle.updated_at.to_rfc3339(),
        }
    }
}