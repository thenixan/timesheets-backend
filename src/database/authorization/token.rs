use diesel::RunQueryDsl;
use rand_core::RngCore;
use uuid::Uuid;

use crate::database::authorization::User;
use crate::schema::tokens;

pub enum CreateTokenOutcome {
    Ok(String),
    Err,
}

#[derive(Insertable, PartialEq, Debug)]
#[table_name = "tokens"]
pub struct NewToken<'a> {
    pub token: &'a str,
    pub user_id: &'a Uuid,
}

pub fn create_for_user(db: &diesel::PgConnection, user: &User) -> CreateTokenOutcome {
    let mut token_bytes = [0u8; 32];
    rand_core::OsRng.fill_bytes(&mut token_bytes);
    let token_string = base64::encode(token_bytes);
    let token_entry = NewToken {
        token: &token_string,
        user_id: &user.id,
    };
    match diesel::insert_into(tokens::table)
        .values(token_entry)
        .execute(db)
    {
        Ok(_) => CreateTokenOutcome::Ok(token_string),
        Err(e) => {
            eprintln!("Error inserting token: {}", e);
            CreateTokenOutcome::Err
        }
    }
}
