//! Represents a log entry for a specific maintenance action taken on a vehicle.
//! *************************************** 100 chars limit ****************************************
//! # Business rules:
//! * CRUD operations on maintenance records should be performed by users.
//! 
//! # Personal notes:
//! * Maintenance is always associated with a vehicle, but not mandatorily with a maintenance type
//! * Maintenance record has uuid because in the future it may be used in a distributed system
//!
//! TODO list:
//! * Add validation rules for the details field (e.g., length, content)
use crate::{
    maintenance::entities::maintenance::MaintenanceIdentity,
    user::entities::user::UserIdentity,
    vehicle::entities::{vehicle::VehicleIdentity, vehicle_status::VehicleStatusIdentity},
};

/// Represents the identity of a maintenance log entry (DB record, non-hydrated).
#[derive(Debug, Clone)]
pub struct MaintenanceRecordIdentity {
    /// The unique identifier for the maintenance log entry.
    pub id: uuid::Uuid,

    /// The unique identifier for the vehicle associated with this maintenance log.
    pub vehicle_id: uuid::Uuid,
    /// The unique identifier for the maintenance record.
    pub maintenance_id: i32,
    /// The unique identifier for the user who performed the maintenance action.
    pub user_id: uuid::Uuid,
    /// The vehicle status at the time of the maintenance action.
    pub vehicle_status_id: i32,

    /// The timestamp when the maintenance action was performed.
    pub performed_at: chrono::DateTime<chrono::Utc>,
    /// The details of the maintenance action performed.
    pub details: String,

    /// The timestamp when the maintenance log entry was created.
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// The user who created the maintenance log entry.
    pub created_by: uuid::Uuid,
    /// The timestamp when the maintenance log entry was last updated.
    pub updated_at: chrono::DateTime<chrono::Utc>,
    /// The user who last updated the maintenance log entry.
    pub updated_by: uuid::Uuid,
}

/// Represents a hydrated version of a maintenance log entry.
#[derive(Debug, Clone)]
pub struct MaintenanceRecord {
    /// The identity details of the maintenance log entry.
    pub identity: MaintenanceRecordIdentity,
    /// The vehicle associated with this maintenance log entry.
    pub vehicle: VehicleIdentity,
    /// The maintenance details associated with this log entry.
    pub maintenance: MaintenanceIdentity,
    /// The user who performed the maintenance action.
    pub user: UserIdentity,
    /// The vehicle status at the time of the maintenance action.
    pub vehicle_status: VehicleStatusIdentity,
}

impl MaintenanceRecord {
    /// Creates a new maintenance record with the provided identity and associated entities.
    ///
    /// # Attention:
    /// * for now, no error as I don't have any validation rules
    pub fn new(
        vehicle: VehicleIdentity,
        maintenance: MaintenanceIdentity,
        user: UserIdentity,
        vehicle_status: VehicleStatusIdentity,
        performed_at: chrono::DateTime<chrono::Utc>,
        details: String,
    ) -> Self {
        let identity = MaintenanceRecordIdentity {
            id: uuid::Uuid::new_v4(),
            vehicle_id: vehicle.id,
            maintenance_id: maintenance.id,
            user_id: user.id,
            vehicle_status_id: vehicle_status.id,
            performed_at,
            details,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };

        Self {
            identity,
            vehicle,
            maintenance,
            user,
            vehicle_status,
        }
    }

    /* Getters */
    pub fn id(&self) -> uuid::Uuid {
        self.identity.id
    }
    pub fn performed_at(&self) -> chrono::DateTime<chrono::Utc> {
        self.identity.performed_at
    }
    pub fn details(&self) -> &str {
        &self.identity.details
    }
}
