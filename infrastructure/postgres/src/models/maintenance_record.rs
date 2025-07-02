//! Represents a log entry for a specific maintenance action taken on a vehicle.

#[derive(Debug, Clone)]
pub struct MaintenanceRecord {
    /// The unique identifier for the maintenance log entry.
    pub id: i32,
    /// The unique identifier for the vehicle associated with this maintenance log.
    pub vehicle_id: i32,
    /// The unique identifier for the maintenance record.
    pub maintenance_id: i32,
    /// The unique identifier for the user who performed the maintenance action.
    pub user_id: i32,
    /// The timestamp when the maintenance action was performed.
    pub performed_at: chrono::DateTime<chrono::Utc>,
    /// Odometer reading at the time of the maintenance action.
    pub odometer: Option<i32>,
    /// The engine hour meter reading at the time of the maintenance action.
    pub engine_hour_meter: Option<i32>,
    /// Next scheduled maintenance date, if applicable.
    pub next_scheduled_date: Option<chrono::NaiveDate>,
    /// Next scheduled maintenance kilometers, if applicable.
    pub next_scheduled_kilometers: Option<i32>,
    /// Next scheduled maintenance engine hours, if applicable.
    pub next_scheduled_engine_hours: Option<i32>,
    /// The details of the maintenance action performed.
    pub details: String,
    // /// Attached files or images related to the maintenance action, if any.
    // pub attachments: Option<Vec<String>>,
}
