use crate::vehicle::entities::vehicle::VehicleSummary;
use std::future::Future;
use uuid::Uuid;

#[derive(Debug, thiserror::Error)]
pub enum VehicleRepositoryError {
    #[error("vehicle not found: {0}")]
    NotFound(Uuid),
    #[error("vehicle already exists: {0}")]
    AlreadyExists(Uuid),
}

/// Repository trait for vehicle operations
pub trait VehicleRepository: Send + Sync {
    // /// Create a new vehicle
    // fn create(
    //     &self,
    //     vehicle: Vehicle,
    // ) -> impl Future<Output = Result<Vehicle, VehicleRepositoryError>> + Send;

    // /// Find a vehicle by its ID
    // fn find_by_id(
    //     &self,
    //     id: Uuid,
    // ) -> impl Future<Output = Result<Option<Vehicle>, VehicleRepositoryError>> + Send;

    /// Find all vehicles
    fn find_all(&self)
    -> impl Future<Output = Result<Vec<VehicleSummary>, VehicleRepositoryError>> + Send;

    // /// Update an existing vehicle
    // fn update(
    //     &self,
    //     vehicle: Vehicle,
    // ) -> impl Future<Output = Result<Vehicle, VehicleRepositoryError>> + Send;

    // /// Delete a vehicle by its ID
    // fn delete(&self, id: Uuid)
    // -> impl Future<Output = Result<bool, VehicleRepositoryError>> + Send;
}
