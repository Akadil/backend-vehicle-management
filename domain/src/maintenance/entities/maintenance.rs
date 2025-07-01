//! Represents a maintenance details of a certain vehicle and maintenance type in the system.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Maintenance {
    /// The unique identifier for the maintenance record.
    pub id: i32,
    /// The unique identifier for the vehicle associated with this maintenance.
    pub vehicle_id: i32,
    /// The type of maintenance being performed.
    pub maintenance_type_id: i32,
    /// The interval type for the maintenance (e.g., kilometers, hours, by date).
    pub interval_type: MaintenanceIntervalType,
    /// Interval value in kilometers (e.g., 10000 km).
    pub interval_km: Option<i32>,
    /// Interval value in engine hours (e.g., 500 hours).
    pub interval_engine_hours: Option<i32>,
    /// Interval value in years (e.g., 1 year).
    pub interval_years: Option<i32>,
    /// Threshold (Red) value in percentage (e.g., 95%).
    pub red_threshold: i32,
    /// Threshold (Yellow) value in percentage (e.g., 80%).
    pub yellow_threshold: i32,
    /// Created at timestamp.
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Updated at timestamp.
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// Represents the different types of maintenance intervals.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MaintenanceIntervalType {
    Kilometers,
    EngineHours,
    Years,
}
