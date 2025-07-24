<<<<<<< HEAD
//! Repository for managing maintenance types.

use crate::maintenance::entities::maintenance_type::{MaintenanceType, MaintenanceTypeView};
use std::future::Future;

/// Errors that can occur when interacting with the maintenance type repository
=======
//! Represents the repository for managing maintenance records.
//!
//! ***************************************** 100 chars limit ****************************************
//! # General rules:
//! * the struct is not identity tagged because there is no hydration logic (leave struct)
use crate::maintenance::entities::maintenance_type::MaintenanceType;
use std::future::Future;

>>>>>>> 78a87e9cb86dea90d76570956f1bc7e35f224ca1
#[derive(Debug, thiserror::Error)]
pub enum MaintenanceTypeRepositoryError {
    #[error("database error: {0}")]
    Database(String),
}

<<<<<<< HEAD
/// Repository interface for maintenance type operations
pub trait MaintenanceTypeRepository: Send + Sync {
    /// Creates a new maintenance type
    fn create(
        &self,
        maintenance_type: MaintenanceType,
        user_id: uuid::Uuid,
    ) -> impl Future<Output = Result<MaintenanceTypeView, MaintenanceTypeRepositoryError>> + Send;

    /// Retrieves a maintenance type by ID
=======
pub trait MaintenanceTypeRepository: Send + Sync {
    /* =============================== Maintenance type functions =============================== */
    /// Create a new maintenance type.
    fn create(
        &self,
        maintenance_type: MaintenanceType,
    ) -> impl Future<Output = Result<MaintenanceType, MaintenanceTypeRepositoryError>> + Send;

    /// Get a maintenance type by its ID.
>>>>>>> 78a87e9cb86dea90d76570956f1bc7e35f224ca1
    fn get_by_id(
        &self,
        id: i32,
    ) -> impl Future<Output = Result<Option<MaintenanceType>, MaintenanceTypeRepositoryError>> + Send;

<<<<<<< HEAD
    /// Retrieves a maintenance type view by ID
    fn get_view_by_id(
        &self,
        id: i32,
    ) -> impl Future<Output = Result<Option<MaintenanceTypeView>, MaintenanceTypeRepositoryError>> + Send;

    /// Retrieves all maintenance type views
    fn get_all_view(
        &self,
    ) -> impl Future<Output = Result<Vec<MaintenanceTypeView>, MaintenanceTypeRepositoryError>> + Send;

    /// Checks if a maintenance type exists by name
    fn exists_by_name(
        &self,
        name: &str,
    ) -> impl Future<Output = Result<bool, MaintenanceTypeRepositoryError>> + Send;

    /// Updates an existing maintenance type
    fn update(
        &self,
        maintenance_type: MaintenanceType,
        user_id: uuid::Uuid,
    ) -> impl Future<Output = Result<MaintenanceTypeView, MaintenanceTypeRepositoryError>> + Send;

    /// Deletes a maintenance type
    fn delete(
        &self,
        maintenance_type: MaintenanceType,
        user_id: uuid::Uuid,
    ) -> impl Future<Output = Result<(), MaintenanceTypeRepositoryError>> + Send;
=======
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
>>>>>>> 78a87e9cb86dea90d76570956f1bc7e35f224ca1
}
