use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SignUpRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SignUpResponse {
    pub msg: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SignInRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SignInResponse {
    pub user_id: String,
    pub jwt_token: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Payload {
    pub user_id: String,
}
