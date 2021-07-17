use rocket::http::HeaderMap;
use rocket::request::{FromRequest, Outcome};
use rocket::Request;

pub mod authorized_user_id;
pub mod client_preferences;

#[derive(Debug)]
pub enum ClientLanguage {
    Ru,
    En,
}

#[derive(Debug)]
pub enum AppName {
    Android,
    Ios,
}

#[derive(Debug)]
pub enum ClientSoftware {
    App {
        app_name: AppName,
        os_version: Option<String>,
        software_version: Option<String>,
        device_name: Option<String>,
    },
    Web {
        user_agent: String,
    },
    Other,
}

#[derive(Debug)]
pub struct ClientPreferences {
    pub client_language: ClientLanguage,
    pub client_software: ClientSoftware,
}

impl<'r> From<&'r HeaderMap<'r>> for ClientLanguage {
    fn from(header_map: &HeaderMap) -> Self {
        let language_header = header_map.get_one("accept-language");
        return if let Some(language_header_value) = language_header {
            if language_header_value.starts_with("ru") {
                ClientLanguage::Ru
            } else {
                ClientLanguage::En
            }
        } else {
            ClientLanguage::En
        };
    }
}

impl<'r> From<&'r HeaderMap<'r>> for ClientSoftware {
    fn from(header_map: &HeaderMap) -> Self {
        let user_agent_header = header_map.get_one("user-agent");
        let client_software = if let Some(user_agent_header_value) = user_agent_header {
            let u_a_lowercase = user_agent_header_value.to_lowercase();
            if u_a_lowercase.starts_with("android") || u_a_lowercase.starts_with("ios") {
                let version = u_a_lowercase
                    .split("/")
                    .collect::<Vec<&str>>()
                    .get(1)
                    .map(|s| s.to_string());
                let app_name = if u_a_lowercase.starts_with("android") {
                    AppName::Android
                } else {
                    AppName::Ios
                };
                ClientSoftware::App {
                    app_name,
                    os_version: header_map.get_one("os-version").map(|s| s.to_string()),
                    software_version: version,
                    device_name: header_map.get_one("device-name").map(|s| s.to_string()),
                }
            } else {
                ClientSoftware::Web {
                    user_agent: user_agent_header_value.to_string(),
                }
            }
        } else {
            ClientSoftware::Other
        };
        return client_software;
    }
}

impl<'r, 'a> FromRequest<'r, 'a> for ClientPreferences {
    type Error = ();

    fn from_request(request: &'r Request<'a>) -> Outcome<Self, Self::Error> {
        let client_language = request.headers().into();
        let client_software = request.headers().into();
        return Outcome::Success(ClientPreferences {
            client_language,
            client_software,
        });
    }
}
