//! Represents a vehicle's license plate (in Kazakhstan).

use std::fmt;

/// Represents a Kazakhstan vehicle license plate
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LicensePlate {
    value: String,
}

#[derive(Debug, thiserror::Error)]
pub enum LicensePlateError {}

#[derive(Debug, thiserror::Error)]
pub enum LicensePlateValidationError {
    #[error("Invalid license plate format: {0}")]
    InvalidFormat(String),
    #[error("License plate cannot be empty")]
    Empty,
    #[error("License plate too long: maximum 8 characters")]
    TooLong,
}

impl LicensePlate {
    /// Creates a new LicensePlate without validation
    pub fn new(value: impl Into<String>) -> Result<Self, LicensePlateError> {
        let value = value.into().trim().to_uppercase();
        Ok(Self { value })
    }

    /// Validates Kazakhstan license plate format
    pub fn validate(value: &str) -> Result<(), LicensePlateValidationError> {
        if value.is_empty() {
            return Err(LicensePlateValidationError::Empty);
        }

        if value.len() > 8 {
            return Err(LicensePlateValidationError::TooLong);
        }

        // Kazakhstan license plate formats:
        // - 123ABC45 (3 digits + 3 letters + 2 digits)
        // - A123BCD (1 letter + 3 digits + 3 letters)
        let is_valid = Self::is_standard_format(value) || Self::is_alternative_format(value);

        if !is_valid {
            return Err(LicensePlateValidationError::InvalidFormat(
                value.to_string(),
            ));
        }

        Ok(())
    }

    /// Checks for 123ABC45 format (3 digits + 3 letters + 2 digits)
    fn is_standard_format(value: &str) -> bool {
        if value.len() != 8 {
            return false;
        }

        let chars: Vec<char> = value.chars().collect();
        chars[0..3].iter().all(|c| c.is_ascii_digit())
            && chars[3..6].iter().all(|c| c.is_ascii_alphabetic())
            && chars[6..8].iter().all(|c| c.is_ascii_digit())
    }

    /// Checks for A123BCD format (1 letter + 3 digits + 3 letters)
    fn is_alternative_format(value: &str) -> bool {
        if value.len() != 7 {
            return false;
        }

        let chars: Vec<char> = value.chars().collect();
        chars[0].is_ascii_alphabetic()
            && chars[1..4].iter().all(|c| c.is_ascii_digit())
            && chars[4..7].iter().all(|c| c.is_ascii_alphabetic())
    }

    /// Returns the license plate value as a string
    pub fn value(&self) -> &str {
        &self.value
    }

    /// Returns the license plate value as an owned String
    pub fn into_string(self) -> String {
        self.value
    }
}

impl fmt::Display for LicensePlate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl TryFrom<String> for LicensePlate {
    type Error = LicensePlateError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl TryFrom<&str> for LicensePlate {
    type Error = LicensePlateError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_standard_format() {
        let plate = LicensePlate::new("123ABC45").unwrap();
        assert_eq!(plate.value(), "123ABC45");
        assert!(LicensePlate::validate("123ABC45").is_ok());
    }

    #[test]
    fn test_valid_alternative_format() {
        let plate = LicensePlate::new("A123BCD").unwrap();
        assert_eq!(plate.value(), "A123BCD");
        assert!(LicensePlate::validate("A123BCD").is_ok());
    }

    #[test]
    fn test_lowercase_conversion() {
        let plate = LicensePlate::new("123abc45").unwrap();
        assert_eq!(plate.value(), "123ABC45");
    }

    #[test]
    fn test_invalid_format() {
        assert!(LicensePlate::validate("123456").is_err());
        assert!(LicensePlate::validate("ABCDEF").is_err());
        assert!(LicensePlate::validate("12ABC45").is_err());
    }

    #[test]
    fn test_empty_license_plate() {
        assert!(matches!(
            LicensePlate::validate(""),
            Err(LicensePlateValidationError::Empty)
        ));
    }

    #[test]
    fn test_too_long() {
        assert!(matches!(
            LicensePlate::validate("123ABC456"),
            Err(LicensePlateValidationError::TooLong)
        ));
    }

    #[test]
    fn test_display() {
        let plate = LicensePlate::new("123ABC45").unwrap();
        assert_eq!(format!("{}", plate), "123ABC45");
    }
}
