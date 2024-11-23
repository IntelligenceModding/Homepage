use std::cmp::PartialEq;
use std::fmt;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use surrealdb::sql::Id;

impl fmt::Display for IntelliThing {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.id)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IntelliThing {
    pub(crate) id: Id,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    #[serde(serialize_with = "serialize_record_id")]
    pub(crate) id: IntelliThing,
    pub(crate) admin: bool,
    pub(crate) name: String,
    pub(crate) email: String,
    #[serde(skip_serializing)]
    pub(crate) password: String,
    pub(crate) firstname: Option<String>,
    pub(crate) lastname: Option<String>,
}

impl PartialEq for IntelliThing {
    fn eq(&self, other: &Self) -> bool {
        other.id == self.id
    }
}

impl User {
    pub fn validate_password(&self, password: String) -> bool {
        // TODO implement hashing
        self.password == password
    }

    // Checks if one of the unique values is the value to compare
    pub fn compare(&self,to_compare: &String) -> bool {
        self.id == IntelliThing {id: Id::from(to_compare) } || self.name == to_compare.clone() || self.email == to_compare.clone()
    }
}


// Used by the http endpoint to allow patching the user
#[derive(Serialize, Deserialize, Debug)]
pub struct BodyUser {
    #[serde(serialize_with = "serialize_option_record_id", deserialize_with = "deserialize_record_id")]
    pub(crate) id: Option<IntelliThing>,
    pub(crate) name: Option<String>,
    pub(crate) email: Option<String>,
    pub(crate) admin: Option<bool>,
    pub(crate) password: Option<String>,
    pub(crate) firstname: Option<String>,
    pub(crate) lastname: Option<String>,
}


fn serialize_record_id<S>(record_id: &IntelliThing, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
{
    serializer.serialize_str(&record_id.id.to_string())
}

fn serialize_option_record_id<S>(record_id: &Option<IntelliThing>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match record_id {
        None => {
            Ok(serializer.serialize_str("")?) 
        }
        Some(something) => {
            serializer.serialize_str(&something.id.to_string())
        }
    }
}

fn deserialize_record_id<'de, D>(deserializer: D) -> Result<Option<IntelliThing>, D::Error>
where D: Deserializer<'de> {
    let buf = String::deserialize(deserializer)?;

    Ok(Some(IntelliThing { id: Id::String(buf) }))
}