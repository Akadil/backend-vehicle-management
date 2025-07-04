use crate::vehicle::entities::vehicle;
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
    /// Create a new vehicle
    fn create(
        &self,
        vehicle: vehicle::NewVehicle,
    ) -> impl Future<Output = Result<vehicle::VehicleIdentity, VehicleRepositoryError>> + Send;

    /// Find a vehicle by its filter
    fn find_by_filter(
        &self,
        filter: vehicle_filter::VehicleFilter,
    ) -> impl Future<Output = Result<Option<vehicle::VehicleIdentity>, VehicleRepositoryError>> + Send;

    /// Find a vehicle by its ID
    fn find_by_id(
        &self,
        id: Uuid,
    ) -> impl Future<Output = Result<Option<vehicle::VehicleIdentity>, VehicleRepositoryError>> + Send;

    /// Find all vehicles
    fn find_all(
        &self,
    ) -> impl Future<Output = Result<Vec<vehicle::VehicleIdentity>, VehicleRepositoryError>> + Send;

    /// Check existence of a vehicle by its VIN
    fn exists_by_vin_or_license_plate(
        &self,
        vin: &str,
        license_plate: &str,
    ) -> impl Future<Output = Result<bool, VehicleRepositoryError>> + Send;

    // /// Update an existing vehicle
    // fn update(
    //     &self,
    //     vehicle: Vehicle,
    // ) -> impl Future<Output = Result<Vehicle, VehicleRepositoryError>> + Send;

    /// Delete a vehicle by its ID
    fn delete(&self, id: Uuid)
    -> impl Future<Output = Result<bool, VehicleRepositoryError>> + Send;
}
