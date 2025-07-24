use domain::maintenance::repositories::maintenance_type_repository::MaintenanceTypeRepositoryError;

#[derive(Debug, thiserror::Error)]
pub enum DeleteMaintenanceTypeError {
    #[error("Maintenance type not found")]
    NotFound,
    #[error("Cannot delete maintenance type: it is currently in use")]
    InUse,
    #[error("Repository error: {0}")]
    Repository(#[from] MaintenanceTypeRepositoryError),
}
