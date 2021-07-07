use rocket_contrib::json::Json;

use crate::database::{projects, DocumentsStorage};
use crate::routes::guards::authorized_user_id::AuthorizedUser;
use crate::routes::route_objects::error_response::{ErrorResponse, ERROR_UNKNOWN};
use crate::routes::route_objects::projects_list_response::{
    ProjectListResponseItem, ProjectsListResponse,
};

#[get("/projects", format = "json")]
pub fn list_projects<'r>(
    user: AuthorizedUser,
    db: DocumentsStorage,
) -> Result<Json<ProjectsListResponse<String>>, ErrorResponse<'r>> {
    match projects::list_projects(&user.user_id, &*db) {
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
