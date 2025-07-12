//! Represents the repository for managing maintenance records.
//!
//! ***************************************** 100 chars limit ****************************************
//! # General rules:
//! * the struct is not identity tagged because there is no hydration logic (leave struct)
use crate::maintenance::entities::maintenance_type::MaintenanceType;
use std::future::Future;

#[derive(Debug, thiserror::Error)]
pub enum MaintenanceTypeRepositoryError {
    #[error("database error: {0}")]
    Database(String),
}

pub trait MaintenanceTypeRepository: Send + Sync {
    /* =============================== Maintenance type functions =============================== */
    /// Create a new maintenance type.
    fn create(
        &self,
        maintenance_type: MaintenanceType,
    ) -> impl Future<Output = Result<MaintenanceType, MaintenanceTypeRepositoryError>> + Send;

    /// Get a maintenance type by its ID.
    fn get_by_id(
        &self,
        id: i32,
    ) -> impl Future<Output = Result<Option<MaintenanceType>, MaintenanceTypeRepositoryError>> + Send;

    /// Get all maintenance types.
    fn get_all(
        &self,
    ) -> impl Future<Output = Result<Vec<MaintenanceType>, MaintenanceTypeRepositoryError>> + Send;

    /// Update an existing maintenance type.
    fn update(
        &self,
        maintenance_type: MaintenanceType,
    ) -> impl Future<Output = Result<MaintenanceType, MaintenanceTypeRepositoryError>> + Send;

    /// Delete a maintenance type.
    fn delete(
        &self,
        maintenance_type: MaintenanceType,
    ) -> impl Future<Output = Result<bool, MaintenanceTypeRepositoryError>> + Send;
}
