use axum::{Extension, extract::{State, Path}, http::StatusCode};
use chrono::Utc;
use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait, IntoActiveModel, Set, ActiveModelTrait};

use crate::database::tasks::{Entity as Tasks, self};

use crate::{database::users::Model, utilities::app_error::AppError};

pub async fn soft_delete_task(
    Extension(user): Extension<Model>,
    State(db): State<DatabaseConnection>,
    Path(task_id): Path<i32>
) -> Result<(), AppError> {
    let task = Tasks::find_by_id(task_id)
        .filter(tasks::Column::UserId.eq(user.id))
        .one(&db)
        .await
        .map_err(|error| {
            eprintln!("Error deleteing task: {:?}", error);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "There was an error deleting the task")
        })?;
    
    // check if task is null
    let mut task = if let Some(task) = task {
        task.into_active_model()
    } else {
        return Err(AppError::new(StatusCode::NOT_FOUND, "not found"));
    };

    let now = Utc::now();

    task.deleted_at = Set(Some(now.into()));
    task.save(&db)
        .await
        .map_err(|error| {
            eprintln!("Error saving after soft-deleting: {:?}", error);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "There was an error deleting the task")
        })?;

    Ok(())
}