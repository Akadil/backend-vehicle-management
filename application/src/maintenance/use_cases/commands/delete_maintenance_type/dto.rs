pub struct DeleteMaintenanceTypeCommand {
    pub id: i32,
    pub user_id: uuid::Uuid, // user (caller) info
}

pub struct DeleteMaintenanceTypeResponse {
    pub success: bool,
    pub message: String,
}
