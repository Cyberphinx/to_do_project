use axum::{ Extension, extract::State, http::StatusCode};
use sea_orm::{DatabaseConnection, IntoActiveModel, Set};

use crate::{database::users, utilities::app_error::AppError, queries::user_queries::save_active_user};

pub async fn logout(
    Extension(user): Extension<users::Model>,
    State(db): State<DatabaseConnection>
) -> Result<StatusCode, AppError> {
    let mut user = user.into_active_model();

    user.token = Set(None);
    
    save_active_user(&db, user).await?;

    Ok(StatusCode::OK)
}