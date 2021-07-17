use rocket_contrib::json::Json;

use crate::database::{projects, DocumentsStorage};
use crate::routes::guards::authorized_user_id::AuthorizedUser;
use crate::routes::route_objects::error_response::{
    ErrorResponse, ERROR_UNKNOWN, ERROR_WRONG_REQUEST,
};
use crate::routes::route_objects::new_project_request::NewProjectRequest;
use crate::routes::route_objects::projects_list_response::{
    ProjectListResponseItem, ProjectsListResponse,
};

#[get("/projects", format = "json")]
pub fn list_projects<'r>(
    user: AuthorizedUser,
    db: DocumentsStorage,
) -> Result<Json<ProjectsListResponse<String>>, ErrorResponse<'r>> {
    match projects::get::list(&user.user_id, &*db) {
        Ok(rows) => {
            let result = ProjectsListResponse {
                projects: rows
                    .into_iter()
                    .map(|r| ProjectListResponseItem {
                        id: r.id.to_hex(),
                        name: r.name,
                    })
                    .collect(),
            };
            Result::Ok(Json(result))
        }
        Err(_) => Result::Err(ERROR_UNKNOWN),
    }
}

#[post("/projects", format = "json", data = "<maybe_new_project_request>")]
pub fn new_project<'r>(
    maybe_new_project_request: Option<Json<NewProjectRequest>>,
    user: AuthorizedUser,
    db: DocumentsStorage,
) -> Result<(), ErrorResponse<'r>> {
    if let Some(new_project_request) = maybe_new_project_request {
        match projects::add::one(&user.user_id, new_project_request.name, &*db) {
            Ok(_) => Result::Ok(()),
            Err(_) => Result::Err(ERROR_UNKNOWN),
        }
    } else {
        Result::Err(ERROR_WRONG_REQUEST)
    }
}
