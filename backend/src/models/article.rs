//! Struct used to send and receive objects from ArangoDB.
use aragog::Record;
use moon::*;

/// This struct is used to send and receive objects to and from database
/// instead of LocalArticle struct in shared folder. This is
/// because of an issue implementing Record for structs in shared folder.
/// Name of struct has to match name of collection in DB. Case sensitive.
#[derive(Debug, Serialize, Deserialize, Clone, Record)]
#[serde(crate = "serde")]
pub struct Article {
    pub id: u32,
    pub title: String,
    pub content: String,
    pub contributors: Vec<String>,
    pub author: String,
    pub tags: Vec<String>,
    pub created_time: String,
    pub updated_time: String,
}

impl Article {
    /// Creates a new Article object using the Article struct.
    pub fn new(
        id: u32,
        title: String,
        content: String,
        contributors: Vec<String>,
        author: String,
        tags: Vec<String>,
        created_time: String,
        updated_time: String,
    ) -> Self {
        Self {
            id,
            title,
            content,
            contributors,
            author,
            tags,
            created_time,
            updated_time,
        }
    }
}
