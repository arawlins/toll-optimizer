use axum::{
    Json,
    extract::{Multipart, State, Query},
    http::StatusCode,
    response::IntoResponse,
};
use rust_decimal::Decimal;
use rust_decimal::prelude::FromPrimitive;
use serde::Deserialize;
use sqlx::PgPool;
use toll_optimizer_core::{csv_parser, trip_analyzer};

use crate::{auth::Claims, db::summary::SummaryDb, models::UploadSummary};

#[derive(Deserialize)]
pub struct HistoryQuery {
    #[serde(default = "default_limit")]
    pub limit: i64,
    #[serde(default = "default_offset")]
    pub offset: i64,
}

fn default_limit() -> i64 { 20 }
fn default_offset() -> i64 { 0 }

pub async fn analyze(
    State(pool): State<PgPool>,
    claims: Claims,
    mut multipart: Multipart,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let mut filename = "unknown.csv".to_string();
    let mut file_content = String::new();

    while let Some(field) = multipart.next_field().await.map_err(|e| {
        tracing::error!("Multipart error: {:?}", e);
        (StatusCode::BAD_REQUEST, format!("Multipart error: {}", e))
    })? {
        let name = field.name().unwrap_or("").to_string();
        if name == "file" {
            if let Some(fname) = field.file_name() {
                // Truncate to 255 to match DB schema
                filename = fname.chars().take(255).collect();
            }
            let data = field.bytes().await.map_err(|e| {
                tracing::error!("Error reading field bytes: {:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Error reading bytes: {}", e),
                )
            })?;
            file_content = String::from_utf8(data.to_vec()).map_err(|e| {
                tracing::error!("UTF-8 conversion error: {:?}", e);
                (
                    StatusCode::BAD_REQUEST,
                    "File is not valid UTF-8".to_string(),
                )
            })?;
        }
    }

    if file_content.is_empty() {
        return Err((StatusCode::BAD_REQUEST, "No file provided".to_string()));
    }

    // Parse trips
    let parsed_results = csv_parser::parse_trips(file_content.as_bytes());

    // Analyze trips
    let time_summaries = trip_analyzer::analyze_trips_by_time(&parsed_results);
    let distance_summaries = trip_analyzer::analyze_trips_by_distance(&parsed_results);

    // Calculate totals
    let mut total_trips = 0;
    let mut total_cost_actual = 0.0;
    let mut time_savings = 0.0;
    let mut distance_savings = 0.0;

    for ((_plate, _dir), trips) in &parsed_results {
        total_trips += trips.len();
        for trip in trips {
            total_cost_actual += trip.get_total_recorded_cost();
        }
    }

    for summary in &time_summaries {
        for centroid in &summary.centroids {
            time_savings += centroid.total_optimized_savings;
        }
    }

    for summary in &distance_summaries {
        for centroid in &summary.centroids {
            distance_savings += centroid.total_optimized_savings;
        }
    }

    let final_savings_f64 = time_savings.max(distance_savings);
    let total_cost_optimized_f64 = total_cost_actual - final_savings_f64;

    let cost_actual_dec = Decimal::from_f64(total_cost_actual).unwrap_or_default();
    let cost_optimized_dec = Decimal::from_f64(total_cost_optimized_f64).unwrap_or_default();
    let savings_dec = Decimal::from_f64(final_savings_f64).unwrap_or_default();

    // Save to DB
    let _summary = pool
        .create_summary(
            claims.sub,
            &filename,
            total_trips as i32,
            cost_actual_dec,
            cost_optimized_dec,
            savings_dec,
        )
        .await
        .map_err(|e| {
            tracing::error!("Database error creating summary: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Database error: {}", e),
            )
        })?;

    let time_analysis_json = serde_json::to_value(&time_summaries)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let distance_analysis_json = serde_json::to_value(&distance_summaries)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let response = crate::models::AnalysisResponse {
        total_trips,
        total_cost: total_cost_actual,
        time_based_savings: time_savings,
        distance_based_savings: distance_savings,
        time_analysis: time_analysis_json,
        distance_analysis: distance_analysis_json,
    };

    Ok(Json(response))
}

pub async fn history(
    State(pool): State<PgPool>,
    claims: Claims,
    Query(query): Query<HistoryQuery>,
) -> Result<Json<Vec<UploadSummary>>, (StatusCode, String)> {
    let summaries = pool
        .get_summaries_by_user(claims.sub, query.limit, query.offset)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(summaries))
}
