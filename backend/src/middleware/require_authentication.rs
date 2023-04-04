use axum::{
    http::{HeaderMap, Request, StatusCode},
    middleware::Next,
    response::Response, extract::State,
};
use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait};

use crate::{utilities::{app_error::AppError, jwt::validate_token, token_wrapper::TokenWrapper}, database::users};
use crate::database::users::Entity as Users;

pub async fn require_authentication<T>(
    headers: HeaderMap,
    State(db): State<DatabaseConnection>,
    State(token_secret): State<TokenWrapper>,
    mut request: Request<T>,
    next: Next<T>,
) -> Result<Response, AppError> {
    let header_token = if let Some(token) = headers.get("x-auth-token") {
        token.to_str()
            .map_err(|error| {
                eprintln!("Error extracting token from hearder: {:?}", error);
                AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Error reading token")
            })?
    } else {
        return Err(AppError::new(StatusCode::UNAUTHORIZED, "not authenticated!"))
    };

    // return early if token is not valid
    validate_token(&token_secret.0, header_token)?;

    // get the user as a model
    let user = Users::find()
        .filter(users::Column::Token.eq(Some(header_token.to_owned())))
        .one(&db)
        .await
        .map_err(|error| {
            eprintln!("Error getting user by token: {:?}", error);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "There was a problem getting your account")
        })?;

    // if token doesn't exist, you are logged out
    if let Some(user) = user {
        // if user is logged in, add the user to the request in an extension
        request.extensions_mut().insert(user);
    } else {
        // if user is not logged in, return early with unauthorized
        return Err(AppError::new(StatusCode::UNAUTHORIZED, "You are not authorized for this"))
    }

    Ok(next.run(request).await)
}
