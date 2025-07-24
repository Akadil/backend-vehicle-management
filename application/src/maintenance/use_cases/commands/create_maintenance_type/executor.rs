use super::{
    dto::{CreateMaintenanceTypeCommand as Input, CreateMaintenanceTypeResponse as Output},
    error::CreateMaintenanceTypeError as Error,
};
use crate::auth::AuthenticatedUser;
use domain::maintenance::{
    entities::maintenance_type::MaintenanceType,
    repositories::maintenance_type_repository::MaintenanceTypeRepository,
};

pub struct CreateMaintenanceTypeUseCase<'a, MTR: MaintenanceTypeRepository + 'a> {
    maintenance_type_repository: &'a MTR,
}

impl<'a, MTR: MaintenanceTypeRepository + 'a> CreateMaintenanceTypeUseCase<'a, MTR> {
    pub fn new(maintenance_type_repository: &'a MTR) -> Self {
        CreateMaintenanceTypeUseCase {
            maintenance_type_repository,
        }
    }

    pub async fn execute(&self, cmd: Input, user: &AuthenticatedUser) -> Result<Output, Error> {
        // Create a new MaintenanceType instance
        let maintenance_type = MaintenanceType::new(cmd.name, cmd.description)?;

        // Check if the maintenance type already exists
        if self
            .maintenance_type_repository
            .exists_by_name(maintenance_type.name())
            .await?
        {
            return Err(Error::AlreadyExists);
        }

        // Create the maintenance type in the repository
        let created_maintenance_type = self
            .maintenance_type_repository
            .create(maintenance_type, user.user_id)
            .await?;

        Ok(Output::from(created_maintenance_type))
    }
}
