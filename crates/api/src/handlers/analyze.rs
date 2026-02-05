use axum::{
    extract::{Multipart, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use rust_decimal::Decimal;
use rust_decimal::prelude::FromPrimitive;
use sqlx::PgPool;
use toll_optimizer_core::{csv_parser, trip_analyzer};

use crate::{auth::Claims, db::summary::SummaryDb, models::UploadSummary};

pub async fn analyze(
    State(pool): State<PgPool>,
    _claims: Claims,
    mut multipart: Multipart,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let mut filename = "unknown.csv".to_string();
    let mut file_content = String::new();

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?
    {
        let name = field.name().unwrap_or("").to_string();
        if name == "file" {
            if let Some(fname) = field.file_name() {
                filename = fname.to_string();
            }
            let data = field
                .bytes()
                .await
                .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
            file_content = String::from_utf8(data.to_vec())
                .map_err(|_| (StatusCode::BAD_REQUEST, "File is not valid UTF-8".to_string()))?;
        }
    }

    if file_content.is_empty() {
        return Err((StatusCode::BAD_REQUEST, "No file provided".to_string()));
    }

    // Parse trips
    let lines = file_content.lines().map(|s| s.to_string());
    let parsed_results = csv_parser::parse_trips(lines);

    // Analyze trips
    let summaries = trip_analyzer::analyze_trips_by_time(&parsed_results);

    // Calculate totals for DB summary
    let mut total_trips = 0;
    let mut total_cost_actual = Decimal::new(0, 2);
    let mut total_cost_optimized = Decimal::new(0, 2);
    
    // Iterate through summaries to calculate aggregate stats
    // Note: The structure is hierarchical: Summary -> Centroids -> Trips
    // We need to be careful not to double count if logic changes, but typically we iterate the results.
    
    // Simple aggregation based on the returned structure
    for summary in &summaries {
        for centroid in &summary.centroids {
            for trip_summary in &centroid.trips {
                total_trips += 1;
                
                let actual = Decimal::from_f64(trip_summary.trip.get_total_recorded_cost()).unwrap_or_default();
                total_cost_actual += actual;

                // Determine optimized cost for this trip
                // If optimized_cost is present, use it. Otherwise use actual.
                // Wait, logic in CLI:
                // if let Some(prev_cost) = trip_summary.total_cost_previous_timeslot ...
                // But trip_summary struct has `optimized_cost` field? 
                // Let's check `TripSummary` definition in `core`.
                
                // In `core/trip_analyzer.rs`:
                // pub struct TripSummary<'a> { ... pub optimized_cost: Option<f64>, ... }
                // Use that if available.
                
                let optimized = if let Some(opt) = trip_summary.optimized_cost {
                     Decimal::from_f64(opt).unwrap_or(actual)
                } else if let Some(prev) = trip_summary.total_cost_previous_timeslot {
                    // CLI logic checked if prev < current - 0.005
                    let prev_dec = Decimal::from_f64(prev).unwrap_or(actual);
                    if prev_dec < actual {
                        prev_dec
                    } else {
                        actual
                    }
                } else if let Some(next) = trip_summary.total_cost_next_timeslot {
                     let next_dec = Decimal::from_f64(next).unwrap_or(actual);
                     if next_dec < actual {
                         next_dec
                     } else {
                         actual
                     }
                } else {
                    actual
                };
                
                total_cost_optimized += optimized;
            }
        }
    }

    let savings = total_cost_actual - total_cost_optimized;

    // Save to DB
    let _summary = pool
        .create_summary(
            _claims.sub,
            &filename,
            total_trips,
            total_cost_actual,
            total_cost_optimized,
            savings,
        )
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(summaries))
}

pub async fn history(
    State(pool): State<PgPool>,
    claims: Claims,
) -> Result<Json<Vec<UploadSummary>>, (StatusCode, String)> {
    let summaries = pool
        .get_summaries_by_user(claims.sub)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(summaries))
}
