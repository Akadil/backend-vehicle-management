//! Represents a maintenance type in the system.

#[derive(Debug, Clone)]
pub struct MaintenanceType {
    /// The unique identifier for the maintenance type.
    pub id: i32,
    /// The name of the maintenance type (e.g., Oil Change, Tire Rotation).
    pub name: String,
    /// Description of the maintenance type.
    pub description: String,
    /// Created at timestamp.
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Updated at timestamp.
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone)]
pub struct NewMaintenanceType {
    /// The name of the maintenance type (e.g., Oil Change, Tire Rotation).
    pub name: String,
    /// Description of the maintenance type.
    pub description: String,
}

impl NewMaintenanceType {
    /// Creates a new instance of `NewMaintenanceType`.
    ///
    /// # Arguments
    /// * `name` - The name of the maintenance type.
    /// * `description` - The description of the maintenance type.
    pub fn new(name: String, description: String) -> Self {
        NewMaintenanceType { name, description }
    }
}
