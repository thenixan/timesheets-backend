use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct NewProjectRequest<'a> {
    #[serde(rename = "name")]
    pub name: &'a str,
}
