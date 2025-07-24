use super::{
    dto::{DeleteMaintenanceTypeCommand as Input, DeleteMaintenanceTypeResponse as Output},
    error::DeleteMaintenanceTypeError as Error,
};
use crate::auth::AuthenticatedUser;
use domain::maintenance::repositories::maintenance_type_repository::MaintenanceTypeRepository;

pub struct DeleteMaintenanceTypeUseCase<'a, MTR: MaintenanceTypeRepository + 'a> {
    maintenance_type_repository: &'a MTR,
}

impl<'a, MTR: MaintenanceTypeRepository + 'a> DeleteMaintenanceTypeUseCase<'a, MTR> {
    pub fn new(maintenance_type_repository: &'a MTR) -> Self {
        DeleteMaintenanceTypeUseCase {
            maintenance_type_repository,
        }
    }

    pub async fn execute(&self, cmd: Input, user: &AuthenticatedUser) -> Result<Output, Error> {
        // First, check if the maintenance type exists
        let maintenance_type = self
            .maintenance_type_repository
            .get_by_id(cmd.id)
            .await?
            .ok_or(Error::NotFound)?;

        // TODO: Add business logic to check if maintenance type is in use
        // This would typically involve checking if there are any maintenance records
        // or maintenance schedules that reference this maintenance type
        // For now, we'll proceed with deletion

        // Delete the maintenance type
        self.maintenance_type_repository
            .delete(maintenance_type, user.user_id)
            .await?;

        Ok(Output {
            success: true,
            message: "Maintenance type deleted successfully".to_string(),
        })
    }
}
