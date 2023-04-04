use axum::{
    routing::{get, post},
    Router, middleware,
};

use crate::{
    app_state::AppState,
    routes::{
        hello_world::hello_world,
        users::{create_user::create_user, login::login, logout::logout},
    }, middleware::require_authentication::require_authentication,
};

// build the router
pub fn create_router(app_state: AppState) -> Router {
    Router::new()
        .route("/api/v1/users/logout", post(logout))
        .route_layer(middleware::from_fn_with_state(app_state.clone(), require_authentication))
        .route("/", get(hello_world))
        .route("/api/v1/users", post(create_user))
        .route("/api/v1/users/login", post(login))
        .with_state(app_state)
}
