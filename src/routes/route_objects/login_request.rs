use rocket::serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
#[serde(crate = "rocket::serde")]
pub struct LoginRequest<'a> {
    pub login: &'a str,
    pub password: &'a str,
}
