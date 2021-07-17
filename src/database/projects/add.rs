use rocket_contrib::databases::{
    mongodb, mongodb::bson, mongodb::db::ThreadedDatabase, mongodb::doc,
};
use rocket_contrib::databases::mongodb::Bson;
use rocket_contrib::databases::mongodb::spec::BinarySubtype;
use uuid::Uuid;

pub fn one(user_id: &Uuid, name: &str, db: &mongodb::db::Database) -> Result<(), ()> {
    let collection = db.collection("projects");
    let bson_user_id = Bson::Binary(BinarySubtype::Uuid, user_id.as_bytes().to_vec());
    let new_project_doc = doc! {"user_id": bson_user_id, "name": name};
    if let Ok(_) = collection.insert_one(new_project_doc, Option::None) {
        return Result::Ok(());
    }
    return Result::Err(());
}
