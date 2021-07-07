use std::fmt::Debug;

use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct ProjectsListResponse<T: Serialize + Debug> {
    pub projects: Vec<ProjectListResponseItem<T>>,
}

#[derive(Serialize, Debug)]
pub struct ProjectListResponseItem<T: Serialize + Debug> {
    pub id: T,
    pub name: String,
}
