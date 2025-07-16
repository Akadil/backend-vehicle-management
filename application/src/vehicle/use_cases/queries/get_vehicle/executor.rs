// use crate::{filters::vehicle_filter, queries::vehicle_query::VehicleQuery};
// use domain::vehicle::{
//     entities::vehicle::{Vehicle, VehicleIdentity},
//     repositories::vehicle_repository::{VehicleRepository, VehicleRepositoryError},
// };
// use uuid::Uuid;

// pub struct GetVehicleUseCase<'a, VR: VehicleRepository + 'a> {
//     vehicle_repository: &'a VR,
// }

// impl<'a, VR: VehicleRepository + 'a> GetVehicleUseCase<'a, VR> {
//     pub fn new(vehicle_repository: &'a VR) -> Self {
//         GetVehicleUseCase { vehicle_repository }
//     }

//     pub async fn execute(&self, query: VehicleQuery) -> Result<Vehicle, Error> {
//         // Validate the query and convert it to a filter
//         let filter = vehicle_filter::VehicleFilter::from_query(query)?;

//         // Find the vehicle by ID
//         let vehicle_identity = self
//             .vehicle_repository
//             .find_by_filter(filter)
//             .await?
//             .ok_or(Error::VehicleNotFound(data.id))?;

//         // Create Vehicle with identity and no latest status for now
//         let vehicle = Vehicle {
//             identity: vehicle_identity,
//             latest_status: None,
//         };

//         Ok(vehicle)
//     }
// }

// pub struct GetVehicleCmd {
//     pub id: Uuid,
// }

// #[derive(Debug, thiserror::Error)]
// pub enum Error {
//     #[error("Invalid filter: {0}")]
//     InvalidFilter(#[from] vehicle_filter::VehicleFilterError),
//     #[error("Vehicle not found: {0}")]
//     VehicleNotFound(Uuid),
//     #[error("Repository error: {0}")]
//     RepositoryError(#[from] VehicleRepositoryError),
// }
