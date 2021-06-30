use rocket::serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
#[serde(crate = "rocket::serde")]
pub struct RegistrationRequest<'a> {
    #[serde(rename = "login")]
    pub login: &'a str,
    #[serde(rename = "password")]
    pub password: &'a str,
}