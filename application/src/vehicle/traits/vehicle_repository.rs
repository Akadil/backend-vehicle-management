use crate::vehicle::{filters::vehicle_filter::VehicleFilter, models::vehicle::VehicleView};
use std::future::Future;
use uuid::Uuid;

#[derive(Debug, thiserror::Error)]
pub enum VehicleApplicationRepositoryError {
    #[error("vehicle not found: {0}")]
    NotFound(Uuid),
    #[error("vehicle already exists: {0}")]
    AlreadyExists(Uuid),
    #[error("database error: {0}")]
    DatabaseError(String),
}

/// Repository trait for vehicle operations
pub trait VehicleApplicationRepository: Send + Sync {
    /// Find a vehicle by its ID
    fn get_by_filter(
        &self,
        filter: VehicleFilter,
    ) -> impl Future<Output = Result<Vec<VehicleView>, VehicleApplicationRepositoryError>> + Send;
}
