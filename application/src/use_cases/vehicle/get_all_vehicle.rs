use domain::vehicle::{
    entities::vehicle::Vehicle,
    repositories::vehicle_repository::{VehicleRepository, VehicleRepositoryError},
};

pub struct GetAllVehicleUseCase<'a, VR: VehicleRepository + 'a> {
    vehicle_repository: &'a VR,
}

impl<'a, VR: VehicleRepository + 'a> GetAllVehicleUseCase<'a, VR> {
    pub fn new(vehicle_repository: &'a VR) -> Self {
        GetAllVehicleUseCase { vehicle_repository }
    }

    pub async fn execute(&self) -> Result<Vec<Vehicle>, Error> {
        // Find all vehicles
        let vehicle_identities = self.vehicle_repository.find_all().await?;

        // Convert VehicleIdentity to Vehicle entities
        let vehicles = vehicle_identities
            .into_iter()
            .map(|identity| Vehicle {
                identity,
                latest_status: None,
            })
            .collect();

        Ok(vehicles)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Repository error: {0}")]
    RepositoryError(#[from] VehicleRepositoryError),
}
