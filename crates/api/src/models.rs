use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    #[serde(skip)]
    pub password_hash: String,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct RegisterPayload {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginPayload {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: User,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct UploadSummary {
    pub id: Uuid,
    pub user_id: Uuid,
    pub filename: String,
    pub total_trips: i32,
    pub cost_actual: rust_decimal::Decimal,
    pub cost_optimized: rust_decimal::Decimal,
    pub savings: rust_decimal::Decimal,
    pub uploaded_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize)]
pub struct AnalysisResponse {
    pub total_trips: usize,
    pub total_cost: f64,
    pub time_based_savings: f64,
    pub distance_based_savings: f64,
    pub time_analysis: serde_json::Value,
    pub distance_analysis: serde_json::Value,
}
