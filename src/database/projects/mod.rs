use serde::Deserialize;

pub mod add;
pub mod get;

#[derive(Deserialize, Debug)]
pub struct ProjectListEntry {
    #[serde(rename = "_id")]
    pub id: bson::oid::ObjectId,
    pub name: String,
}
