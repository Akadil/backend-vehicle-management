use super::{
    dto::{CreateVehicleCommand as Input, CreateVehicleResponse as Output},
    error::CreateVehicleError as Error,
};
use domain::vehicle::{
    entities::vehicle::{NewVehicle, VehicleError},
    repositories::vehicle_repository::VehicleRepository,
};

pub struct CreateVehicleUseCase<'a, VR: VehicleRepository + 'a> {
    vehicle_repository: &'a VR,
}

impl<'a, VR: VehicleRepository + 'a> CreateVehicleUseCase<'a, VR> {
    pub fn new(vehicle_repository: &'a VR) -> Self {
        CreateVehicleUseCase { vehicle_repository }
    }

    pub async fn execute(&self, cmd: Input) -> Result<Output, Error> {
        unimplemented!();
        // Validate input data
        // let vehicle: NewVehicle = cmd.try_into()?;

        // Create the vehicle
        // let vehicle = self.vehicle_repository.create(vehicle).await?;

        // Ok(vehicle)
    }
}

impl TryInto<NewVehicle> for Input {
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
