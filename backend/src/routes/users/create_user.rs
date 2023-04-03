use super::ResponseUser;

pub async fn create_user() -> Json<ResponseUser> {
    let response = ResponseUser{data: Some(true)};

    Json(response)
}