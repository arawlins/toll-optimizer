use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use sqlx::PgPool;

use crate::{
    auth::Auth,
    db::UserDb,
    models::{AuthResponse, LoginPayload, RegisterPayload},
};

pub async fn register(
    State(pool): State<PgPool>,
    Json(payload): Json<RegisterPayload>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    // Check if user exists
    if let Ok(Some(_)) = pool.get_user_by_email(&payload.email).await {
        return Err((
            StatusCode::BAD_REQUEST,
            "Registration failed. If you already have an account, please try logging in."
                .to_string(),
        ));
    }

    // Hash password
    let password_hash = Auth::hash_password(&payload.password).map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to hash password".to_string(),
        )
    })?;

    // Create user
    let user = pool
        .create_user(&payload.email, &password_hash)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // Create token
    let token = Auth::create_jwt(user.id).map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to generate token".to_string(),
        )
    })?;

    Ok((StatusCode::CREATED, Json(AuthResponse { token, user })))
}

pub async fn login(
    State(pool): State<PgPool>,
    Json(payload): Json<LoginPayload>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    // Find user
    let user = pool
        .get_user_by_email(&payload.email)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::UNAUTHORIZED, "Invalid credentials".to_string()))?;

    // Verify password
    let is_valid = Auth::verify_password(&payload.password, &user.password_hash).map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to verify password".to_string(),
        )
    })?;

    if !is_valid {
        return Err((StatusCode::UNAUTHORIZED, "Invalid credentials".to_string()));
    }

    // Create token
    let token = Auth::create_jwt(user.id).map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to generate token".to_string(),
        )
    })?;

    Ok(Json(AuthResponse { token, user }))
}
