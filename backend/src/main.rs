use backend::{run, utilities::token_wrapper::TokenWrapper, app_state::AppState};
use dotenvy_macro::dotenv;
use sea_orm::Database;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let database_url = dotenv!("DATABASE_URL").to_owned();
    let jwt_secret = dotenv!("JWT_SECRET").to_owned();
    let db = match Database::connect(database_url).await {
        Ok(db) => db,
        Err(error) => {
            eprintln!("Error connecting to the database: {:?}", error);
            panic!();
        },
    };

    let app_state = AppState{
        db,
        jwt_secret: TokenWrapper(jwt_secret)
    };

    run(app_state).await;
}
