use super::{
    dto::{GetVehiclesResponse as Output, VehicleResponse},
    error::GetVehiclesError as Error,
};
use crate::vehicle::{
    filters::vehicle_filter::VehicleFilter,
    traits::vehicle_repository::VehicleApplicationRepository,
};

pub struct GetVehiclesUseCase<'a, VAR: VehicleApplicationRepository + 'a> {
    repo: &'a VAR,
}

impl<'a, VAR: VehicleApplicationRepository + 'a> GetVehiclesUseCase<'a, VAR> {
    pub fn new(repo: &'a VAR) -> Self {
        GetVehiclesUseCase { repo }
    }

    pub async fn execute(&self, filter: VehicleFilter) -> Result<Output, Error> {
        // validate pagination parameters
        // ?

        let vehicles = self.repo.get_by_filter(filter).await?;

        Ok(Output {
            vehicles: vehicles.into_iter().map(VehicleResponse::from).collect(),
            total_count: vehicles.len(),
            page: filter.page,
            page_size: filter.page_size,
            total_pages: if filter.page_size == 0 {
                1
            } else {
                (total_count as f64 / filter.page_size as f64).ceil() as u32
            },
        })
    }
}
