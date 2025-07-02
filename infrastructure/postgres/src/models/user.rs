//! Represents an user in the system.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct User {
    /// The unique identifier for the user.
    pub id: String,
    /// Uuid of the user.
    pub uuid: uuid::Uuid,
    /// The username of the user.
    pub username: String,
    /// The email of the user.
    pub email: String,
    /// First name of the user.
    pub first_name: String,
    /// Last name of the user.
    pub last_name: String,
}