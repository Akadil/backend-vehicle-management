use domain::maintenance::entities::maintenance_type::MaintenanceTypeView;

pub struct CreateMaintenanceTypeCommand {
    pub name: String,
    pub description: String,
    pub user_id: uuid::Uuid, // user (caller) info
}

pub struct CreateMaintenanceTypeResponse {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub created_by: uuid::Uuid, // user ID
    pub created_by_email: String,
}

impl From<MaintenanceTypeView> for CreateMaintenanceTypeResponse {
    fn from(view: MaintenanceTypeView) -> Self {
        CreateMaintenanceTypeResponse {
            id: view.id,
            name: view.name,
            description: view.description,
            created_at: view.created_at,
            created_by: view.created_by.id,
            created_by_email: view.created_by.email.into(),
        }
    }
}
