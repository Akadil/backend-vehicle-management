use super::{
    dto::{GetMaintenanceTypeByIdQuery as Input, GetMaintenanceTypeByIdResponse as Output},
    error::GetMaintenanceTypeByIdError as Error,
};
use domain::maintenance::repositories::maintenance_type_repository::MaintenanceTypeRepository;

pub struct GetMaintenanceTypeByIdUseCase<'a, MTR: MaintenanceTypeRepository + 'a> {
    maintenance_type_repository: &'a MTR,
}

impl<'a, MTR: MaintenanceTypeRepository + 'a> GetMaintenanceTypeByIdUseCase<'a, MTR> {
    pub fn new(maintenance_type_repository: &'a MTR) -> Self {
        GetMaintenanceTypeByIdUseCase {
            maintenance_type_repository,
        }
    }

    pub async fn execute(&self, query: Input) -> Result<Output, Error> {
        let maintenance_type_view = self
            .maintenance_type_repository
            .get_view_by_id(query.id)
            .await?
            .ok_or(Error::NotFound)?;

        Ok(Output::from(maintenance_type_view))
    }
}
