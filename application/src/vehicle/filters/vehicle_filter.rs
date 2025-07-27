// application/filter/vehicle_filter.rs
use crate::shared::pagination::SortOrder;
use domain::vehicle::value_types::{engine_type, license_plate, vehicle_vin};
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct VehicleFilter {
    pub uuid: Option<uuid::Uuid>,
    pub make: Option<String>,
    pub model: Option<String>,
    pub year: Option<u16>,
    pub vin: Option<vehicle_vin::VehicleVin>,
    pub license_plate: Option<license_plate::LicensePlate>,
    pub engine_type: Option<engine_type::EngineType>,

    pub page: u32,
    pub page_size: u32,
    pub sort_by: Option<VehicleSortBy>,
    pub sort_order: SortOrder,
}

pub struct NewVehicleFilter {
    pub make: Option<String>,
    pub model: Option<String>,
    pub year: Option<u16>,
    pub vin: Option<vehicle_vin::VehicleVin>,
    pub license_plate: Option<license_plate::LicensePlate>,
    pub engine_type: Option<engine_type::EngineType>,
}

#[derive(Debug, thiserror::Error)]
pub enum VehicleFilterError {
    #[error("Invalid UUID: {0}")]
    InvalidUuid(#[from] uuid::Error),
    #[error("Invalid VIN: {0}")]
    InvalidVin(#[from] vehicle_vin::VehicleVinError),
    #[error("Invalid License Plate: {0}")]
    InvalidLicensePlate(#[from] license_plate::LicensePlateError),
    #[error("Invalid Engine Type: {0}")]
    InvalidEngineType(#[from] engine_type::EngineTypeError),
    #[error("Invalid Sort By: {0}")]
    InvalidSortBy(String),
}

impl VehicleFilter {
    pub fn new(filter: NewVehicleFilter) -> Self {
        Self {
            uuid: None,
            make: filter.make,
            model: filter.model,
            year: filter.year,
            vin: filter.vin,
            license_plate: Some(filter.license_plate),
            engine_type: Some(filter.engine_type),
            page: 1,
            page_size: 10,
            sort_by: None,
            sort_order: SortOrder::Asc,
        }
    }
}

/*  This logic should be in presentation layer, not the application */

// impl VehicleFilter {
//     pub fn from_query(query: VehicleQuery) -> Result<Self, VehicleFilterError> {
//         let page_size = query.page_size.min(MAX_PAGE_SIZE);

//         let uuid = query.uuid.map(|u| uuid::Uuid::parse_str(&u)).transpose()?;
//         let vin = query.vin.map(vehicle_vin::VehicleVin::new).transpose()?;
//         let license_plate = query
//             .license_plate
//             .map(license_plate::LicensePlate::new)
//             .transpose()?;
//         let engine_type = query
//             .engine_type
//             .map(engine_type::EngineType::new)
//             .transpose()?;
//         let sort_by = query
//             .sort_by
//             .map(|s| SortBy::from_str(&s))
//             .transpose()
//             .map_err(|e| VehicleFilterError::InvalidSortBy(e))?;

//         Ok(Self {
//             uuid,
//             make: query.make,
//             model: query.model,
//             year: query.year,
//             vin,
//             license_plate,
//             engine_type,
//             page: query.page,
//             page_size,
//             sort_by,
//             sort_order: query.sort_order.unwrap_or(SortOrder::Asc),
//         })
//     }

//     pub fn offset(&self) -> u32 {
//         (self.page - 1) * self.page_size
//     }
// }

#[derive(Debug, Clone, Copy, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VehicleSortBy {
    Make,
    Model,
    Year,
    Vin,
    LicensePlate,
    EngineType,
    CreatedAt,
    UpdatedAt,
}

impl VehicleSortBy {
    pub fn as_column_name(&self) -> &'static str {
        match self {
            Self::Make => "make",
            Self::Model => "model",
            Self::Year => "year",
            Self::Vin => "vin",
            Self::LicensePlate => "license_plate",
            Self::EngineType => "engine_type",
            Self::CreatedAt => "created_at",
            Self::UpdatedAt => "updated_at",
        }
    }
}

impl FromStr for VehicleSortBy {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "make" => Ok(Self::Make),
            "model" => Ok(Self::Model),
            "year" => Ok(Self::Year),
            "vin" => Ok(Self::Vin),
            "license_plate" => Ok(Self::LicensePlate),
            "engine_type" => Ok(Self::EngineType),
            "created_at" => Ok(Self::CreatedAt),
            "updated_at" => Ok(Self::UpdatedAt),
            _ => Err(format!("Invalid sort field: {}", s)),
        }
    }
}
