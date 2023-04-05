use axum::http::StatusCode;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, TryIntoModel,
};

use crate::{
    database::users::{self, Entity as Users, Model as UserModel},
    utilities::app_error::AppError,
};

pub async fn save_active_user(
    db: &DatabaseConnection,
    user: users::ActiveModel,
) -> Result<UserModel, AppError> {
    let user = user.save(db).await.map_err(|error| {
        let error_message = error.to_string();

        if error_message
            .contains(r#"duplicate key value violates unique constraint "users_username_key""#)
        {
            AppError::new(
                StatusCode::BAD_REQUEST,
                "Username already taken, try again with a different user name",
            )
        } else {
            eprintln!("Error creating user: {:?}", error_message);
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Something went wrong, please try again",
            )
        }
    })?;

    let user = user.save(db).await.map_err(|error| {
        eprintln!("Error saving user: {:?}", error);
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Error saving user")
    })?;

    convert_active_to_model(user)
}

pub async fn find_by_username(
    db: &DatabaseConnection,
    username: String,
) -> Result<UserModel, AppError> {
    Users::find()
        .filter(users::Column::Username.eq(username))
        .one(db)
        .await
        .map_err(|error| {
            eprintln!("Error getting user by username: {:?}", error);
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error logging in, please try again later",
            )
        })?.ok_or_else(|| {
            AppError::new(StatusCode::BAD_REQUEST, "incorrect username and/or password")
        })
}

fn convert_active_to_model(active_user: users::ActiveModel) -> Result<UserModel, AppError> {
    active_user.try_into_model().map_err(|error| {
        eprintln!("Error converting user active model to model: {:?}", error);
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
    })
}
