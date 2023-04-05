use axum::{extract::State, http::StatusCode, Extension, Json};
use sea_orm::DatabaseConnection;

use crate::{
    database::{users::Model as UserModel},
    utilities::app_error::AppError, queries::task_queries,
};

use super::{ResponseTask, ResponseDataTask, create_task_extractor::ValidateCreateTask};

pub async fn create_task(
    Extension(user): Extension<UserModel>,
    State(db): State<DatabaseConnection>,
    task: ValidateCreateTask
) -> Result<(StatusCode, Json<ResponseDataTask>), AppError> {
        let task = task_queries::create_task(task, &user, &db).await?;
        
        let response = ResponseTask {
            id: task.id,
            title: task.title,
            description: task.description,
            priority: task.priority,
            completed_at: task.completed_at.map(|time| time.to_string())
        };
    
        Ok((StatusCode::CREATED,Json(ResponseDataTask{data: response})))
}
