//! Struct used to send and receive objects from ArangoDB.
use aragog::Record;
use moon::*;

/// This struct is used to send and receive objects to and from database
/// instead of LocalUser struct in shared folder. This is
/// because of an issue implementing Record for structs in shared folder.
/// Name of struct has to match name of collection in DB. Case sensitive.
#[derive(Debug, Serialize, Deserialize, Clone, Record)]
#[serde(crate = "serde")]
pub struct User {
    pub id: String,
    pub email: String,
    pub username: String,
}

impl User {
    /// Creates a new User object using the User struct.
    pub fn new(id: String, email: String, username: String) -> Self {
        Self {
            id,
            email,
            username,
        }
    }
}
