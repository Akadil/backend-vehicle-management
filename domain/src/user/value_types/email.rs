use std::fmt::{Display, Formatter};

/// A value type representing an email address.
/// Ensures that email addresses are valid and properly formatted.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Email(String);

impl Email {
    /// Creates a new Email after validation.
    ///
    /// # Arguments
    /// * `email` - The string representation of an email address
    ///
    /// # Returns
    /// * `Ok(Email)` if the email is valid
    /// * `Err(String)` if the email is invalid
    pub fn new(email: String) -> Result<Self, String> {
        Self::is_valid_email(&email)?;
        Ok(Email(email.to_lowercase()))
    }

    /// Returns the inner string value of the Email.
    pub fn value(&self) -> &str {
        &self.0
    }

    /// Consumes the Email and returns the inner string.
    pub fn into_inner(self) -> String {
        self.0
    }

    /// Basic email validation.
    /// Returns specific error messages for validation failures.
    fn is_valid_email(email: &str) -> Result<(), String> {
        let email = email.trim();

        if email.is_empty() {
            return Err("Email cannot be empty".to_string());
        }

        // Basic validation: must contain exactly one @ symbol
        let parts: Vec<&str> = email.split('@').collect();
        if parts.len() != 2 {
            return Err("Email must contain exactly one @ symbol".to_string());
        }

        let local = parts[0];
        let domain = parts[1];

        // Local part validation
        if local.is_empty() {
            return Err("Email local part cannot be empty".to_string());
        }
        if local.len() > 64 {
            return Err("Email local part cannot exceed 64 characters".to_string());
        }

        // Domain part validation
        if domain.is_empty() {
            return Err("Email domain cannot be empty".to_string());
        }
        if domain.len() > 253 {
            return Err("Email domain cannot exceed 253 characters".to_string());
        }

        // Domain must contain at least one dot
        if !domain.contains('.') {
            return Err("Email domain must contain at least one dot".to_string());
        }

        // Basic character validation
        let valid_local_chars = |c: char| c.is_alphanumeric() || ".-_+".contains(c);
        if !local.chars().all(valid_local_chars) {
            return Err("Email local part contains invalid characters".to_string());
        }

        let valid_domain_chars = |c: char| c.is_alphanumeric() || ".-".contains(c);
        if !domain.chars().all(valid_domain_chars) {
            return Err("Email domain contains invalid characters".to_string());
        }

        Ok(())
    }
}

impl Display for Email {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<String> for Email {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Email::new(value)
    }
}

impl From<Email> for String {
    fn from(email: Email) -> Self {
        email.0
    }
}

impl AsRef<str> for Email {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_email() {
        let email = Email::new("user@example.com".to_string()).unwrap();
        assert_eq!(email.value(), "user@example.com");
    }

    #[test]
    fn test_email_case_normalization() {
        let email = Email::new("User@EXAMPLE.COM".to_string()).unwrap();
        assert_eq!(email.value(), "user@example.com");
    }

    #[test]
    fn test_invalid_email_no_at() {
        let result = Email::new("userexample.com".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_email_multiple_at() {
        let result = Email::new("user@example@com".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_email_no_domain() {
        let result = Email::new("user@".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_email_no_local() {
        let result = Email::new("@example.com".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_email_no_dot_in_domain() {
        let result = Email::new("user@example".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_empty_email() {
        let result = Email::new("".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_display() {
        let email = Email::new("test@example.com".to_string()).unwrap();
        assert_eq!(format!("{}", email), "test@example.com");
    }
}
