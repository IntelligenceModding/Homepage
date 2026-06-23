use serde::{Deserialize, Serialize, Serializer};
use surrealdb::types::{Datetime, RecordId, RecordIdKey, SurrealValue};

#[derive(Serialize, Deserialize, Debug, Clone, SurrealValue)]
pub struct UserData {
    pub(crate) admin: Option<bool>,
    pub(crate) name: Option<String>,
    pub(crate) email: Option<String>,
    pub(crate) password: Option<String>,
    pub(crate) firstname: Option<String>,
    pub(crate) lastname: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, SurrealValue)]
pub struct User {
    pub(crate) id: RecordId,
    pub(crate) admin: bool,
    pub(crate) name: String,
    pub(crate) email: String,
    pub(crate) password: String,
    pub(crate) firstname: Option<String>,
    pub(crate) lastname: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, SurrealValue)]
pub struct CreatePost {
    pub author: RecordId,
    pub likes: i32,
    pub views: i32,
    pub title: String,
    pub posted: Datetime,
}

#[derive(Serialize, Deserialize, Debug, SurrealValue)]
pub struct PostData {
    pub(crate) author: Option<RecordId>,
    pub(crate) likes: Option<i32>,
    pub(crate) views: Option<i32>,
    pub(crate) title: Option<String>,
    pub(crate) posted: Option<Datetime>,
}

#[derive(Serialize, Deserialize, Debug, SurrealValue)]
pub struct Post {
    pub(crate) id: RecordId,
    pub(crate) author: RecordId,
    pub(crate) likes: i32,
    pub(crate) views: i32,
    pub(crate) title: String,
    pub(crate) posted: Datetime,
}

impl User {
    pub fn validate_password(&self, password: String) -> bool {
        // TODO implement hashing
        self.password == password
    }

    pub fn compare(&self, to_compare: &str) -> bool {
        if self.name == to_compare || self.email == to_compare {
            return true;
        }

        match &self.id.key {
            RecordIdKey::String(s) => s == to_compare,
            RecordIdKey::Number(n) => n.to_string() == to_compare,
            RecordIdKey::Uuid(u) => u.to_string() == to_compare,

            _ => false,
        }
    }
}
