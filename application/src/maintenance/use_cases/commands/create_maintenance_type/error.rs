use domain::maintenance::{
    entities::maintenance_type::MaintenanceTypeError,
    repositories::maintenance_type_repository::MaintenanceTypeRepositoryError,
};

#[derive(Debug, thiserror::Error)]
pub enum CreateMaintenanceTypeError {
    #[error("Invalid input data: {0}")]
    Validation(#[from] MaintenanceTypeError),
    #[error("Maintenance type already exists")]
    AlreadyExists,
    #[error("Repository error: {0}")]
    Repository(#[from] MaintenanceTypeRepositoryError),
}
