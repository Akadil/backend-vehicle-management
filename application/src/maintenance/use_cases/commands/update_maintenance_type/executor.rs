use super::{
    dto::{UpdateMaintenanceTypeCommand as Input, UpdateMaintenanceTypeResponse as Output},
    error::UpdateMaintenanceTypeError as Error,
};
use crate::auth::AuthenticatedUser;
use domain::maintenance::{
    entities::maintenance_type::MaintenanceType,
    repositories::maintenance_type_repository::MaintenanceTypeRepository,
};

pub struct UpdateMaintenanceTypeUseCase<'a, MTR: MaintenanceTypeRepository + 'a> {
    maintenance_type_repository: &'a MTR,
}

impl<'a, MTR: MaintenanceTypeRepository + 'a> UpdateMaintenanceTypeUseCase<'a, MTR> {
    pub fn new(maintenance_type_repository: &'a MTR) -> Self {
        UpdateMaintenanceTypeUseCase {
            maintenance_type_repository,
        }
    }

    pub async fn execute(&self, cmd: Input, user: &AuthenticatedUser) -> Result<Output, Error> {
        // First, check if the maintenance type exists
        let existing_maintenance_type = self
            .maintenance_type_repository
            .get_by_id(cmd.id)
            .await?
            .ok_or(Error::NotFound)?;

        // Check if the new name already exists (only if name is changing)
        if existing_maintenance_type.name() != cmd.name {
            if self
                .maintenance_type_repository
                .exists_by_name(&cmd.name)
                .await?
            {
                return Err(Error::NameAlreadyExists);
            }
        }

        // Create a new MaintenanceType instance with updated data
        let mut updated_maintenance_type = MaintenanceType::new(cmd.name, cmd.description)?;

        // Update the maintenance type in the repository
        let updated_maintenance_type_view = self
            .maintenance_type_repository
            .update(updated_maintenance_type, user.user_id)
            .await?;

        Ok(Output::from(updated_maintenance_type_view))
    }
}
