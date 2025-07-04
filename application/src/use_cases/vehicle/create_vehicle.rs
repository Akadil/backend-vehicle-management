use domain::vehicle::{
    entities::vehicle::{NewVehicle, VehicleError, VehicleIdentity},
    repositories::vehicle_repository::{VehicleRepository, VehicleRepositoryError},
};

pub struct CreateVehicleUseCase<'a, VR: VehicleRepository + 'a> {
    vehicle_repository: &'a VR,
}

impl<'a, VR: VehicleRepository + 'a> CreateVehicleUseCase<'a, VR> {
    pub fn new(vehicle_repository: &'a VR) -> Self {
        CreateVehicleUseCase { vehicle_repository }
    }

    pub async fn execute(&self, data: CreateVehicleCmd) -> Result<VehicleIdentity, Error> {
        // Validate input data
        let vehicle: NewVehicle = data.try_into()?;

        // Check if the vehicle already exists (if needed)
        if self
            .vehicle_repository
            .exists_by_vin_or_license_plate(vehicle.vin.value(), vehicle.license_plate.value())
            .await?
        {
            return Err(Error::VehicleAlreadyExists(vehicle.vin.value().to_string()));
        }

        // Create the vehicle
        let vehicle = self.vehicle_repository.create(vehicle).await?;

        Ok(vehicle)
    }
}

pub struct CreateVehicleCmd {
    pub make: String,
    pub model: String,
    pub year: u16,
    pub vin: String,
    pub license_plate: String,
    pub engine_type: String,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Invalid input: {0}")]
    InvalidInput(#[from] VehicleError),
    #[error("Vehicle already exists: {0}")]
    VehicleAlreadyExists(String),
    #[error("Repository error: {0}")]
    RepositoryError(#[from] VehicleRepositoryError),
}

impl TryInto<NewVehicle> for CreateVehicleCmd {
    type Error = VehicleError;

    fn try_into(self) -> Result<NewVehicle, Self::Error> {
        NewVehicle::new(
            self.make,
            self.model,
            self.year,
            self.vin,
            self.license_plate,
            self.engine_type,
        )
    }
}
