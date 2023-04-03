use serde::{Deserialize, Serialize};

pub mod create_user;

#[derive(Serialize, Deserialize)]
pub struct ResponseUser {
    data: Option<bool>
}

