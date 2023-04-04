use std::net::SocketAddr;

use router::create_router;
use app_state::AppState;

pub mod app_state;
mod router;
mod routes;
mod database;
pub mod utilities;


pub async fn run(app_state: AppState) {
    let app = create_router(app_state);
    let address = SocketAddr::from(([0, 0, 0, 0], 6006));

    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await
        .unwrap();
}