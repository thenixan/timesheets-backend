use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct LoginResponse {
    #[serde(rename = "token")]
    token: String,
}

impl From<String> for LoginResponse {
    fn from(s: String) -> Self {
        LoginResponse { token: s }
    }
}
