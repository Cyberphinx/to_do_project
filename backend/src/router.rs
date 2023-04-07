use axum::{
    http::{HeaderValue, Method},
    middleware,
    routing::{delete, get, patch, post, put},
    Router,
};
use tower_http::cors::{CorsLayer, Any};

use crate::{
    app_state::AppState,
    middleware::require_authentication::require_authentication,
    routes::{
        hello_world::hello_world,
        tasks::{
            create_task::create_task,
            delete_tasks::soft_delete_task,
            get_all_tasks::get_all_tasks,
            get_one_task::get_one_task,
            update_tasks::{mark_completed, mark_uncompleted, update_task},
        },
        users::{create_user::create_user, login::login, logout::logout},
    },
};

// build the router
pub fn create_router(app_state: AppState) -> Router {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin("http://localhost:5173".parse::<HeaderValue>().unwrap());

    Router::new()
        .route("/api/v1/users/logout", post(logout))
        .route("/api/v1/tasks", post(create_task))
        .route("/api/v1/tasks", get(get_all_tasks))
        .route("/api/v1/tasks/:task_id", get(get_one_task))
        .route("/api/v1/tasks/:task_id/completed", put(mark_completed))
        .route("/api/v1/tasks/:task_id/uncompleted", put(mark_uncompleted))
        .route("/api/v1/tasks/:task_id", patch(update_task))
        .route("/api/v1/tasks/:task_id", delete(soft_delete_task))
        .route_layer(middleware::from_fn_with_state(
            app_state.clone(),
            require_authentication,
        ))
        .route("/", get(hello_world))
        .route("/api/v1/users", post(create_user))
        .route("/api/v1/users/login", post(login))
        .with_state(app_state)
        .layer(cors)
}
