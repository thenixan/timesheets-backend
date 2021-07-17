use rocket_contrib::databases::mongodb::coll::options::FindOptions;
use rocket_contrib::databases::mongodb::db::ThreadedDatabase;
use rocket_contrib::databases::mongodb::spec::BinarySubtype;
use rocket_contrib::databases::mongodb::{self, bson, doc, from_bson, Bson};
use uuid::Uuid;

use crate::database::projects::ProjectListEntry;

pub fn list(user_id: &Uuid, db: &mongodb::db::Database) -> Result<Vec<ProjectListEntry>, ()> {
    let collection = db.collection("projects");
    let bson_user_id = Bson::Binary(BinarySubtype::Uuid, user_id.as_bytes().to_vec());
    let filter = doc! {"user_id": bson_user_id};
    let mut find_options = FindOptions::default();
    find_options.projection = Some(doc! {"_id": 1, "name": 1});
    match collection.find(Some(filter), Some(find_options)) {
        Ok(mut cursor) => {
            let mut result = Vec::new();
            while let Some(document) = cursor.next().and_then(|r| r.ok()) {
                match from_bson::<ProjectListEntry>(Bson::Document(document)) {
                    Ok(project_entry) => {
                        result.push(project_entry);
                    }
                    Err(e) => {
                        eprintln!("Error deserializing: {:?}", e);
                    }
                }
            }
            Result::Ok(result)
        }
        Err(_) => Result::Err(()),
    }
}
