use actix_web::{
    HttpResponse, post,
    web::{self, Data},
};
use bcrypt::{DEFAULT_COST, hash, verify};
use jsonwebtoken::{EncodingKey, Header, encode};

use crate::{
    error::CustomError,
    types::{
        app::AppState,
        user::{Payload, SignInRequest, SignInResponse, SignUpRequest, SignUpResponse},
    },
};

#[post("/sign-in")]
pub async fn sign_in(
    body: web::Json<SignInRequest>,
    app_state: Data<AppState>,
) -> Result<HttpResponse, CustomError> {
    let data: SignInRequest = body.into_inner();

    let user = sqlx::query!(
        "SELECT id, username, password FROM users WHERE username = $1",
        &data.username
    )
    .fetch_optional(&app_state.pool)
    .await
    .map_err(|_| CustomError::InternalError)?;

    let user = match user {
        Some(user) => user,
        None => return Err(CustomError::UserNotFound),
    };

    let is_correct =
        verify(data.password, &user.password).map_err(|_| CustomError::InternalError)?;

    if !is_correct {
        return Err(CustomError::WrongPassword);
    };
    let jwt_secret = std::env::var("JWT_SECRET").expect("jwt secret is missing");
    let header = Header::default();
    let encoding_key = EncodingKey::from_secret(jwt_secret.as_bytes());
    let jwt_token = encode(
        &header,
        &Payload {
            user_id: user.id.to_string(),
        },
        &encoding_key,
    )
    .map_err(|_| CustomError::InternalError)?;

    Ok(HttpResponse::Ok().json(SignInResponse {
        user_id: user.id.to_string(),
        jwt_token,
    }))
}

#[post("/sign-up")]
pub async fn sign_up(
    body: web::Json<SignUpRequest>,
    app_state: Data<AppState>,
) -> Result<HttpResponse, CustomError> {
    let data: SignUpRequest = body.into_inner();

    let user_exist = sqlx::query!("SELECT id FROM users WHERE username = $1", &data.username)
        .fetch_optional(&app_state.pool)
        .await
        .map_err(|_| CustomError::InternalError)?;

    if user_exist.is_some() {
        return Err(CustomError::UserExists);
    }
    let hashed_pass = hash(data.password, DEFAULT_COST).map_err(|_| CustomError::InternalError)?;
    sqlx::query!(
        "INSERT INTO users (username, password) VALUES ($1, $2)",
        data.username,
        hashed_pass
    )
    .execute(&app_state.pool)
    .await
    .map_err(|_| CustomError::InternalError)?;

    Ok(HttpResponse::Created().json(SignUpResponse {
        msg: "user created succefully".to_string(),
    }))
}
