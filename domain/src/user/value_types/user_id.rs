use std::fmt::{Display, Formatter};
use uuid::Uuid;

/// A value type representing a user identifier.
/// Wraps a UUID to ensure type safety and proper identification.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UserId(Uuid);

impl UserId {
    /// Creates a new UserId from a UUID.
    pub fn new(id: Uuid) -> Self {
        UserId(id)
    }

    /// Creates a new UserId with a randomly generated UUID.
    pub fn generate() -> Self {
        UserId(Uuid::new_v4())
    }

    /// Creates a UserId from a string representation of a UUID.
    ///
    /// # Arguments
    /// * `id` - The string representation of a UUID
    ///
    /// # Returns
    /// * `Ok(UserId)` if the string is a valid UUID
    /// * `Err(uuid::Error)` if the string is not a valid UUID
    pub fn from_string(id: &str) -> Result<Self, uuid::Error> {
        let uuid = Uuid::parse_str(id)?;
        Ok(UserId(uuid))
    }

    /// Returns the inner UUID value.
    pub fn value(&self) -> Uuid {
        self.0
    }

    /// Consumes the UserId and returns the inner UUID.
    pub fn into_inner(self) -> Uuid {
        self.0
    }
}

impl Display for UserId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<Uuid> for UserId {
    fn from(uuid: Uuid) -> Self {
        UserId(uuid)
    }
}

impl From<UserId> for Uuid {
    fn from(user_id: UserId) -> Self {
        user_id.0
    }
}

impl AsRef<Uuid> for UserId {
    fn as_ref(&self) -> &Uuid {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_user_id() {
        let uuid = Uuid::new_v4();
        let user_id = UserId::new(uuid);
        assert_eq!(user_id.value(), uuid);
    }

    #[test]
    fn test_generate_user_id() {
        let user_id = UserId::generate();
        assert_ne!(user_id.value(), Uuid::nil());
    }

    #[test]
    fn test_from_string_valid() {
        let uuid_str = "550e8400-e29b-41d4-a716-446655440000";
        let user_id = UserId::from_string(uuid_str).unwrap();
        assert_eq!(user_id.to_string(), uuid_str);
    }

    #[test]
    fn test_from_string_invalid() {
        let result = UserId::from_string("invalid-uuid");
        assert!(result.is_err());
    }

    #[test]
    fn test_display() {
        let uuid = Uuid::new_v4();
        let user_id = UserId::new(uuid);
        assert_eq!(format!("{}", user_id), format!("{}", uuid));
    }
}
