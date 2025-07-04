//! Represents a vehicle's engine type.

use std::fmt;
use std::str::FromStr;

/// Represents different types of vehicle engines
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EngineType {
    /// Gasoline/Petrol engine
    Gasoline,
    /// Diesel engine
    Diesel,
    /// Electric motor
    Electric,
    // /// Hybrid (combination of gasoline and electric)
    // Hybrid,
    // /// Plug-in hybrid electric vehicle
    // PluginHybrid,
    // /// Compressed Natural Gas
    // Cng,
    // /// Liquefied Petroleum Gas
    // Lpg,
    // /// Hydrogen fuel cell
    // Hydrogen,
    /// Other/Unknown engine type
    Other(String),
}

#[derive(Debug, thiserror::Error)]
pub enum EngineTypeError {
    #[error("Invalid engine type: {0}")]
    InvalidType(String),
    #[error("Engine type cannot be empty")]
    Empty,
}

impl EngineType {
    /// Creates a new EngineType from a string
    pub fn new(value: impl Into<String>) -> Result<Self, EngineTypeError> {
        let temp = value.into();
        let value = temp.trim();
        if value.is_empty() {
            return Err(EngineTypeError::Empty);
        }

        Self::from_str(value)
    }

    /// Returns the engine type as a string
    pub fn as_str(&self) -> &str {
        match self {
            EngineType::Gasoline => "gasoline",
            EngineType::Diesel => "diesel",
            EngineType::Electric => "electric",
            // EngineType::Hybrid => "hybrid",
            // EngineType::PluginHybrid => "plugin_hybrid",
            // EngineType::Cng => "cng",
            // EngineType::Lpg => "lpg",
            // EngineType::Hydrogen => "hydrogen",
            EngineType::Other(value) => value,
        }
    }

    /// Returns the display name of the engine type
    pub fn display_name(&self) -> &str {
        match self {
            EngineType::Gasoline => "Gasoline",
            EngineType::Diesel => "Diesel",
            EngineType::Electric => "Electric",
            // EngineType::Hybrid => "Hybrid",
            // EngineType::PluginHybrid => "Plug-in Hybrid",
            // EngineType::Cng => "CNG",
            // EngineType::Lpg => "LPG",
            // EngineType::Hydrogen => "Hydrogen",
            EngineType::Other(value) => value,
        }
    }

    /// Checks if the engine type is electric or hybrid
    pub fn is_electric_powered(&self) -> bool {
        matches!(
            self,
            EngineType::Electric // | EngineType::Hybrid | EngineType::PluginHybrid
        )
    }

    /// Checks if the engine type uses fossil fuels
    pub fn uses_fossil_fuel(&self) -> bool {
        matches!(
            self,
            EngineType::Gasoline | EngineType::Diesel // | EngineType::Hybrid | EngineType::PluginHybrid
        )
    }

    /// Checks if the engine type is alternative fuel
    pub fn is_alternative_fuel(&self) -> bool {
        matches!(
            self,
            EngineType::Electric // | EngineType::Cng | EngineType::Lpg | EngineType::Hydrogen
        )
    }

    /// Returns all standard engine types
    pub fn standard_types() -> Vec<EngineType> {
        vec![
            EngineType::Gasoline,
            EngineType::Diesel,
            EngineType::Electric,
            // EngineType::Hybrid,
            // EngineType::PluginHybrid,
            // EngineType::Cng,
            // EngineType::Lpg,
            // EngineType::Hydrogen,
        ]
    }
}

impl FromStr for EngineType {
    type Err = EngineTypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let normalized = s.trim().to_lowercase().replace([' ', '-'], "_");

        match normalized.as_str() {
            "gasoline" | "petrol" | "gas" => Ok(EngineType::Gasoline),
            "diesel" => Ok(EngineType::Diesel),
            "electric" | "ev" | "bev" => Ok(EngineType::Electric),
            // "hybrid" | "hev" => Ok(EngineType::Hybrid),
            // "plugin_hybrid" | "plug_in_hybrid" | "phev" => Ok(EngineType::PluginHybrid),
            // "cng" | "compressed_natural_gas" => Ok(EngineType::Cng),
            // "lpg" | "liquefied_petroleum_gas" => Ok(EngineType::Lpg),
            // "hydrogen" | "fuel_cell" | "fcev" => Ok(EngineType::Hydrogen),
            _ => Ok(EngineType::Other(s.trim().to_string())),
        }
    }
}

impl fmt::Display for EngineType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.display_name())
    }
}

impl TryFrom<String> for EngineType {
    type Error = EngineTypeError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl TryFrom<&str> for EngineType {
    type Error = EngineTypeError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_standard_engine_types() {
        assert_eq!(EngineType::new("gasoline").unwrap(), EngineType::Gasoline);
        assert_eq!(EngineType::new("diesel").unwrap(), EngineType::Diesel);
        assert_eq!(EngineType::new("electric").unwrap(), EngineType::Electric);
        // assert_eq!(EngineType::new("hybrid").unwrap(), EngineType::Hybrid);
    }

    #[test]
    fn test_case_insensitive() {
        assert_eq!(EngineType::new("GASOLINE").unwrap(), EngineType::Gasoline);
        assert_eq!(EngineType::new("Diesel").unwrap(), EngineType::Diesel);
        assert_eq!(EngineType::new("Electric").unwrap(), EngineType::Electric);
    }

    #[test]
    fn test_aliases() {
        assert_eq!(EngineType::new("petrol").unwrap(), EngineType::Gasoline);
        assert_eq!(EngineType::new("ev").unwrap(), EngineType::Electric);
        // assert_eq!(EngineType::new("phev").unwrap(), EngineType::PluginHybrid);
    }

    #[test]
    fn test_other_type() {
        let engine = EngineType::new("rotary").unwrap();
        assert_eq!(engine, EngineType::Other("rotary".to_string()));
    }

    #[test]
    fn test_empty_engine_type() {
        assert!(matches!(EngineType::new(""), Err(EngineTypeError::Empty)));
    }

    #[test]
    fn test_electric_powered() {
        assert!(EngineType::Electric.is_electric_powered());
        // assert!(EngineType::Hybrid.is_electric_powered());
        // assert!(EngineType::PluginHybrid.is_electric_powered());
        assert!(!EngineType::Gasoline.is_electric_powered());
    }

    #[test]
    fn test_fossil_fuel() {
        assert!(EngineType::Gasoline.uses_fossil_fuel());
        assert!(EngineType::Diesel.uses_fossil_fuel());
        // assert!(EngineType::Hybrid.uses_fossil_fuel());
        assert!(!EngineType::Electric.uses_fossil_fuel());
    }

    #[test]
    fn test_alternative_fuel() {
        assert!(EngineType::Electric.is_alternative_fuel());
        // assert!(EngineType::Cng.is_alternative_fuel());
        // assert!(EngineType::Hydrogen.is_alternative_fuel());
        assert!(!EngineType::Gasoline.is_alternative_fuel());
    }

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", EngineType::Gasoline), "Gasoline");
        // assert_eq!(format!("{}", EngineType::PluginHybrid), "Plug-in Hybrid");
        assert_eq!(
            format!("{}", EngineType::Other("Rotary".to_string())),
            "Rotary"
        );
    }

    #[test]
    fn test_as_str() {
        assert_eq!(EngineType::Gasoline.as_str(), "gasoline");
        // assert_eq!(EngineType::PluginHybrid.as_str(), "plugin_hybrid");
    }
}
