use domain::maintenance::entities::maintenance_type::MaintenanceTypeView;

pub struct SearchMaintenanceTypesQuery {
    pub search_term: String,
    pub limit: Option<usize>,
}

pub struct MaintenanceTypeSearchResult {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

pub struct SearchMaintenanceTypesResponse {
    pub results: Vec<MaintenanceTypeSearchResult>,
    pub total_found: usize,
    pub search_term: String,
}

impl From<MaintenanceTypeView> for MaintenanceTypeSearchResult {
    fn from(view: MaintenanceTypeView) -> Self {
        MaintenanceTypeSearchResult {
            id: view.id,
            name: view.name,
            description: view.description,
            created_at: view.created_at,
            updated_at: view.updated_at,
        }
    }
}
