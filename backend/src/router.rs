use crate::routes::{hello_world::hello_world, users::create_user::create_user};

use axum::{Router, routing::{get, post}};

// build the router
pub fn create_router() -> Router {
    Router::new()
        // `GET /` goes to `root`
        .route("/", get(hello_world))
        .route("/api/v1/users", post(create_user))
}
