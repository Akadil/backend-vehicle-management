use domain::maintenance::repositories::maintenance_type_repository::MaintenanceTypeRepositoryError;

#[derive(Debug, thiserror::Error)]
pub enum SearchMaintenanceTypesError {
    #[error("Invalid search term: must not be empty")]
    EmptySearchTerm,
    #[error("Repository error: {0}")]
    Repository(#[from] MaintenanceTypeRepositoryError),
}
