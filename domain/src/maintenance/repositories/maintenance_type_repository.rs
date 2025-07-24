//! Repository for managing maintenance types.

use crate::maintenance::entities::maintenance_type::{MaintenanceType, MaintenanceTypeView};
use std::future::Future;

/// Errors that can occur when interacting with the maintenance type repository
#[derive(Debug, thiserror::Error)]
pub enum MaintenanceTypeRepositoryError {
    #[error("database error: {0}")]
    Database(String),
}

/// Repository interface for maintenance type operations
pub trait MaintenanceTypeRepository: Send + Sync {
    /// Creates a new maintenance type
    fn create(
        &self,
        maintenance_type: MaintenanceType,
        user_id: uuid::Uuid,
    ) -> impl Future<Output = Result<MaintenanceTypeView, MaintenanceTypeRepositoryError>> + Send;

    /// Retrieves a maintenance type by ID
    fn get_by_id(
        &self,
        id: i32,
    ) -> impl Future<Output = Result<Option<MaintenanceType>, MaintenanceTypeRepositoryError>> + Send;

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
}
