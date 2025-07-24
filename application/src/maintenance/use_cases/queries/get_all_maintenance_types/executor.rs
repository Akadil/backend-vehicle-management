use super::{
    dto::{GetAllMaintenanceTypesQuery as Input, GetAllMaintenanceTypesResponse as Output},
    error::GetAllMaintenanceTypesError as Error,
};
use domain::maintenance::repositories::maintenance_type_repository::MaintenanceTypeRepository;

pub struct GetAllMaintenanceTypesUseCase<'a, MTR: MaintenanceTypeRepository + 'a> {
    maintenance_type_repository: &'a MTR,
}

impl<'a, MTR: MaintenanceTypeRepository + 'a> GetAllMaintenanceTypesUseCase<'a, MTR> {
    pub fn new(maintenance_type_repository: &'a MTR) -> Self {
        GetAllMaintenanceTypesUseCase {
            maintenance_type_repository,
        }
    }

    pub async fn execute(&self, _query: Input) -> Result<Output, Error> {
        let maintenance_type_views = self.maintenance_type_repository.get_all_view().await?;

        Ok(Output::from(maintenance_type_views))
    }
}
