/// Represents the different types of maintenance intervals.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MaintenanceIntervalType {
    Kilometers,
    EngineHours,
    Years,
}

impl MaintenanceIntervalType {
    /// Returns the string representation of the interval type.
    pub fn as_str(&self) -> &str {
        match self {
            MaintenanceIntervalType::Kilometers => "Kilometers",
            MaintenanceIntervalType::EngineHours => "Engine Hours",
            MaintenanceIntervalType::Years => "Years",
        }
    }

    /// Returns the interval type from a string representation.
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "Kilometers" => Some(MaintenanceIntervalType::Kilometers),
            "Engine Hours" => Some(MaintenanceIntervalType::EngineHours),
            "Years" => Some(MaintenanceIntervalType::Years),
            _ => None,
        }
    }

    /// Returns the interval type from a string representation, or returns an error if the string is invalid.
    pub fn from_str_or_err(s: &str) -> Result<Self, String> {
        match s {
            "Kilometers" => Ok(MaintenanceIntervalType::Kilometers),
            "Engine Hours" => Ok(MaintenanceIntervalType::EngineHours),
            "Years" => Ok(MaintenanceIntervalType::Years),
            _ => Err(format!("Invalid maintenance interval type: {}", s)),
        }
    }
}

impl std::fmt::Display for MaintenanceIntervalType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
