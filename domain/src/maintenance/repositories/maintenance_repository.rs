//! Represents the repository for managing maintenance records.
//!
//! ***************************************** 100 chars limit ****************************************
use crate::maintenance::entities::maintenance_type::MaintenanceType;
use std::future::Future;

#[derive(Debug, thiserror::Error)]
pub enum MaintenanceRepositoryError {
    #[error("database error: {0}")]
    Database(String),
}

pub trait MaintenanceRepository: Send + Sync {
    /* =============================== Maintenance type functions =============================== */
    /// Create a new maintenance type.
    fn create(
        &self,
        maintenance_type: MaintenanceType,
    ) -> impl Future<Output = Result<MaintenanceType, MaintenanceRepositoryError>> + Send;

    /// Get a maintenance type by its ID.
    fn get_by_id(
        &self,
        id: i32,
    ) -> impl Future<Output = Result<Option<MaintenanceType>, MaintenanceRepositoryError>> + Send;

    /// Get all maintenance types.
    fn get_all(
        &self,
    ) -> impl Future<Output = Result<Vec<MaintenanceType>, MaintenanceRepositoryError>> + Send;

    /// Update an existing maintenance type.
    fn update(
        &self,
        maintenance_type: MaintenanceType,
    ) -> impl Future<Output = Result<MaintenanceType, MaintenanceRepositoryError>> + Send;

    /// Delete a maintenance type.
    fn delete(
        &self,
        maintenance_type: MaintenanceType,
    ) -> impl Future<Output = Result<bool, MaintenanceRepositoryError>> + Send;
}
