use actix_web::{
    Error, HttpMessage,
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    middleware::Next,
};
use jsonwebtoken::{DecodingKey, Validation, decode};

use crate::{error::CustomError, types::user::Payload};

pub async fn my_custom_middleware(
    req: ServiceRequest,
    next: Next<impl MessageBody + 'static>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    let auth_header = req
        .headers()
        .get("Authorization")
        .ok_or(CustomError::JwtMissing)?;

    let token: Vec<&str> = auth_header
        .to_str()
        .map_err(|_| CustomError::InternalError)?
        .split(" ")
        .collect();

    let jwt_token = token.get(1).copied().ok_or(CustomError::JwtMissing)?;
    println!("jwt token is: {} ", jwt_token);
    let jwt_secret = std::env::var("JWT_SECRET").expect("jwt_secrete is missing");

    match verify_jwt(jwt_token, jwt_secret) {
        Some(payload) => {
            req.extensions_mut().insert(payload);
            next.call(req).await
        }
        None => {
            println!("jwt token is wrong");
            Err(CustomError::WrongJwtToken.into())
        }
    }
}

pub fn verify_jwt(jwt_token: &str, jwt_secret: String) -> Option<Payload> {
    let mut validation = Validation::new(jsonwebtoken::Algorithm::HS256);
    validation.validate_exp = false;
    validation.required_spec_claims.remove("exp");

    let decoding_key = DecodingKey::from_secret(jwt_secret.as_bytes());
    

    match decode::<Payload>(jwt_token, &decoding_key, &validation) {
        Ok(token_data) => {
            Some(token_data.claims)
        }
        Err(err) => {
            println!("jwt error {:?} ", err);
            None
        }
    }
}
