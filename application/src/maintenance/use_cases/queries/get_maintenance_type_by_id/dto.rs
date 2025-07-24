use domain::maintenance::entities::maintenance_type::MaintenanceTypeView;

pub struct GetMaintenanceTypeByIdQuery {
    pub id: i32,
}

pub struct GetMaintenanceTypeByIdResponse {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub created_by: uuid::Uuid,
    pub created_by_email: String,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub updated_by: uuid::Uuid,
    pub updated_by_email: String,
}

impl From<MaintenanceTypeView> for GetMaintenanceTypeByIdResponse {
    fn from(view: MaintenanceTypeView) -> Self {
        GetMaintenanceTypeByIdResponse {
            id: view.id,
            name: view.name,
            description: view.description,
            created_at: view.created_at,
            created_by: view.created_by.id,
            created_by_email: view.created_by.email.into(),
            updated_at: view.updated_at,
            updated_by: view.updated_by.id,
            updated_by_email: view.updated_by.email.into(),
        }
    }
}
