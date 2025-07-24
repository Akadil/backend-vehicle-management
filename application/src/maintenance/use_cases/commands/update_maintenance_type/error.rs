use domain::maintenance::{
    entities::maintenance_type::MaintenanceTypeError,
    repositories::maintenance_type_repository::MaintenanceTypeRepositoryError,
};

#[derive(Debug, thiserror::Error)]
pub enum UpdateMaintenanceTypeError {
    #[error("Invalid input data: {0}")]
    Validation(#[from] MaintenanceTypeError),
    #[error("Maintenance type not found")]
    NotFound,
    #[error("Maintenance type name already exists")]
    NameAlreadyExists,
    #[error("Repository error: {0}")]
    Repository(#[from] MaintenanceTypeRepositoryError),
}
