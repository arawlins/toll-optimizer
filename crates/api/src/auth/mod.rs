use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use std::env;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

pub mod extractor;

use axum::{extract::Request, http::StatusCode, middleware::Next, response::Response};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Basic},
};

pub async fn basic_auth(
    auth: Option<TypedHeader<Authorization<Basic>>>,
    req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let username = env::var("METRICS_USERNAME").expect("METRICS_USERNAME must be set");
    let password = env::var("METRICS_PASSWORD").expect("METRICS_PASSWORD must be set");

    if let Some(TypedHeader(Authorization(basic))) = auth {
        if basic.username() == username && basic.password() == password {
            return Ok(next.run(req).await);
        }
    }

    Err(StatusCode::UNAUTHORIZED)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid,  // Subject (User ID)
    pub exp: usize, // Expiration
    pub iat: usize, // Issued At
}

pub struct Auth;

impl Auth {
    pub fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password_hash = argon2
            .hash_password(password.as_bytes(), &salt)?
            .to_string();
        Ok(password_hash)
    }

    pub fn verify_password(
        password: &str,
        password_hash: &str,
    ) -> Result<bool, argon2::password_hash::Error> {
        let parsed_hash = PasswordHash::new(password_hash)?;
        Ok(Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok())
    }

    pub fn create_jwt(user_id: Uuid) -> Result<String, jsonwebtoken::errors::Error> {
        let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
        let expiration = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
            + 60 * 60 * 24; // 24 hours

        let claims = Claims {
            sub: user_id,
            iat: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs() as usize,
            exp: expiration as usize,
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret.as_bytes()),
        )
    }

    pub fn decode_jwt(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
        let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(secret.as_bytes()),
            &Validation::default(),
        )?;
        Ok(token_data.claims)
    }
}
