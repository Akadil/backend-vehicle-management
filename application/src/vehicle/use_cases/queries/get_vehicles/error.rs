use crate::vehicle::traits::vehicle_repository::VehicleApplicationRepositoryError;

#[derive(Debug, thiserror::Error)]
pub enum GetVehiclesError {
    // #[error("Invalid pagination parameters: {0}")]
    // InvalidPagination(String),
    #[error("Repository error: {0}")]
    RepositoryError(#[from] VehicleApplicationRepositoryError),
}
