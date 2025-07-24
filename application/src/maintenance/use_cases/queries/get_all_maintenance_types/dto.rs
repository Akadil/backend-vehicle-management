use domain::maintenance::entities::maintenance_type::MaintenanceTypeView;

pub struct GetAllMaintenanceTypesQuery;

pub struct MaintenanceTypeSummary {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

pub struct GetAllMaintenanceTypesResponse {
    pub maintenance_types: Vec<MaintenanceTypeSummary>,
    pub total_count: usize,
}

impl From<MaintenanceTypeView> for MaintenanceTypeSummary {
    fn from(view: MaintenanceTypeView) -> Self {
        MaintenanceTypeSummary {
            id: view.id,
            name: view.name,
            description: view.description,
            created_at: view.created_at,
            updated_at: view.updated_at,
        }
    }
}

impl From<Vec<MaintenanceTypeView>> for GetAllMaintenanceTypesResponse {
    fn from(views: Vec<MaintenanceTypeView>) -> Self {
        let total_count = views.len();
        let maintenance_types = views
            .into_iter()
            .map(MaintenanceTypeSummary::from)
            .collect();

        GetAllMaintenanceTypesResponse {
            maintenance_types,
            total_count,
        }
    }
}
