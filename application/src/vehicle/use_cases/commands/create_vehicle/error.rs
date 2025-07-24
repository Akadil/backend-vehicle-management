use domain::vehicle::repositories::vehicle_repository::VehicleRepositoryError;

#[derive(Debug, thiserror::Error)]
pub enum CreateVehicleError {
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    #[error("Vehicle already exists: {0}")]
    VehicleAlreadyExists(String),
    #[error("Repository error: {0}")]
    RepositoryError(#[from] VehicleRepositoryError),
}
