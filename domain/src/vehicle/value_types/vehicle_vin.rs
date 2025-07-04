//! Represents a vehicle's VIN (Vehicle Identification Number).

use std::fmt;

/// Represents a Vehicle Identification Number (VIN)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VehicleVin {
    value: String,
}

#[derive(Debug, thiserror::Error)]
pub enum VehicleVinError {
    #[error("Invalid VIN format: {0}")]
    InvalidFormat(String),
    #[error("VIN cannot be empty")]
    Empty,
    #[error("VIN must be exactly 17 characters long")]
    InvalidLength,
    #[error("VIN contains invalid characters: {0}")]
    InvalidCharacters(String),
    #[error("Invalid check digit in VIN: {0}")]
    InvalidCheckDigit(String),
}

impl VehicleVin {
    /// Creates a new VehicleVin after validating the format
    pub fn new(value: impl Into<String>) -> Result<Self, VehicleVinError> {
        let value = value.into().trim().to_uppercase();
        Self::validate(&value)?;
        Ok(Self { value })
    }

    /// Validates VIN format and check digit
    fn validate(value: &str) -> Result<(), VehicleVinError> {
        if value.is_empty() {
            return Err(VehicleVinError::Empty);
        }

        if value.len() != 17 {
            return Err(VehicleVinError::InvalidLength);
        }

        // Check for valid characters (no I, O, Q allowed in VIN)
        let invalid_chars: Vec<char> = value
            .chars()
            .filter(|&c| !Self::is_valid_vin_character(c))
            .collect();

        if !invalid_chars.is_empty() {
            return Err(VehicleVinError::InvalidCharacters(
                invalid_chars.iter().collect::<String>(),
            ));
        }

        // Validate check digit (9th position)
        if !Self::is_valid_check_digit(value) {
            return Err(VehicleVinError::InvalidCheckDigit(value.to_string()));
        }

        Ok(())
    }

    /// Checks if character is valid in VIN (alphanumeric except I, O, Q)
    fn is_valid_vin_character(c: char) -> bool {
        c.is_ascii_alphanumeric() && !matches!(c, 'I' | 'O' | 'Q')
    }

    /// Validates the check digit (9th character) using VIN algorithm
    fn is_valid_check_digit(vin: &str) -> bool {
        let chars: Vec<char> = vin.chars().collect();
        let weights = [8, 7, 6, 5, 4, 3, 2, 10, 0, 9, 8, 7, 6, 5, 4, 3, 2];

        let mut sum = 0;
        for (i, &c) in chars.iter().enumerate() {
            if i == 8 {
                continue; // Skip check digit position
            }
            sum += Self::char_to_value(c) * weights[i];
        }

        let remainder = sum % 11;
        let check_digit = if remainder == 10 {
            'X'
        } else {
            char::from_digit(remainder as u32, 10).unwrap()
        };

        chars[8] == check_digit
    }

    /// Converts VIN character to numeric value for check digit calculation
    fn char_to_value(c: char) -> usize {
        match c {
            '0'..='9' => c as usize - '0' as usize,
            'A' | 'J' => 1,
            'B' | 'K' | 'S' => 2,
            'C' | 'L' | 'T' => 3,
            'D' | 'M' | 'U' => 4,
            'E' | 'N' | 'V' => 5,
            'F' | 'W' => 6,
            'G' | 'P' | 'X' => 7,
            'H' | 'Y' => 8,
            'R' | 'Z' => 9,
            _ => 0,
        }
    }

    /// Returns the VIN value as a string
    pub fn value(&self) -> &str {
        &self.value
    }

    /// Returns the VIN value as an owned String
    pub fn into_string(self) -> String {
        self.value
    }

    /// Returns the World Manufacturer Identifier (first 3 characters)
    pub fn wmi(&self) -> &str {
        &self.value[0..3]
    }

    /// Returns the Vehicle Descriptor Section (characters 4-9)
    pub fn vds(&self) -> &str {
        &self.value[3..9]
    }

    /// Returns the Vehicle Identifier Section (characters 10-17)
    pub fn vis(&self) -> &str {
        &self.value[9..17]
    }

    /// Returns the model year character (10th position)
    pub fn model_year_code(&self) -> char {
        self.value.chars().nth(9).unwrap()
    }

    /// Returns the assembly plant code (11th position)
    pub fn assembly_plant_code(&self) -> char {
        self.value.chars().nth(10).unwrap()
    }
}

impl fmt::Display for VehicleVin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl TryFrom<String> for VehicleVin {
    type Error = VehicleVinError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl TryFrom<&str> for VehicleVin {
    type Error = VehicleVinError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_vin() {
        let vin = VehicleVin::new("1HGBH41JXMN109186").unwrap();
        assert_eq!(vin.value(), "1HGBH41JXMN109186");
    }

    #[test]
    fn test_lowercase_conversion() {
        let vin = VehicleVin::new("1hgbh41jxmn109186").unwrap();
        assert_eq!(vin.value(), "1HGBH41JXMN109186");
    }

    #[test]
    fn test_invalid_length() {
        assert!(matches!(
            VehicleVin::new("1HGBH41JXMN10918"),
            Err(VehicleVinError::InvalidLength)
        ));
    }

    #[test]
    fn test_invalid_characters() {
        assert!(matches!(
            VehicleVin::new("1HGBH41JXMN109I86"),
            Err(VehicleVinError::InvalidCharacters(_))
        ));
    }

    #[test]
    fn test_empty_vin() {
        assert!(matches!(VehicleVin::new(""), Err(VehicleVinError::Empty)));
    }

    #[test]
    fn test_vin_sections() {
        let vin = VehicleVin::new("1HGBH41JXMN109186").unwrap();
        assert_eq!(vin.wmi(), "1HG");
        assert_eq!(vin.vds(), "BH41JX");
        assert_eq!(vin.vis(), "MN109186");
        assert_eq!(vin.model_year_code(), 'M');
        assert_eq!(vin.assembly_plant_code(), 'N');
    }

    #[test]
    fn test_display() {
        let vin = VehicleVin::new("1HGBH41JXMN109186").unwrap();
        assert_eq!(format!("{}", vin), "1HGBH41JXMN109186");
    }
}
