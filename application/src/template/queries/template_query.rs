// application/query/vehicle_query.rs
use crate::pagination::{DEFAULT_PAGE, DEFAULT_PAGE_SIZE, SortOrder};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct VehicleQuery {
    pub uuid: Option<String>,
    pub make: Option<String>,
    pub model: Option<String>,
    pub year: Option<u16>,
    pub vin: Option<String>,
    pub license_plate: Option<String>,

    pub engine_type: Option<String>,

    #[serde(default = "default_page")]
    pub page: u32,

    #[serde(default = "default_page_size")]
    pub page_size: u32,

    pub sort_by: Option<String>,
    pub sort_order: Option<SortOrder>,
}

fn default_page() -> u32 {
    DEFAULT_PAGE
}

fn default_page_size() -> u32 {
    DEFAULT_PAGE_SIZE
}
