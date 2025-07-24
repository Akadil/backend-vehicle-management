use domain::maintenance::repositories::maintenance_type_repository::MaintenanceTypeRepositoryError;

#[derive(Debug, thiserror::Error)]
pub enum GetMaintenanceTypeByIdError {
    #[error("Maintenance type not found")]
    NotFound,
    #[error("Repository error: {0}")]
    Repository(#[from] MaintenanceTypeRepositoryError),
}
