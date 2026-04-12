use axum::{Json, http::StatusCode};
use chrono::{NaiveDate, Datelike};
use toll_optimizer_core::{
    constants::{WEEKDAY_TIMESLOTS_2026, WEEKEND_TIMESLOTS_2026},
    light_vehicles::{
        WEEKDAY_EB_AVERAGE_TOLL_PRICES_2026, WEEKDAY_WB_AVERAGE_TOLL_PRICES_2026,
        WEEKEND_EB_AVERAGE_TOLL_PRICES_2026, WEEKEND_WB_AVERAGE_TOLL_PRICES_2026,
    },
    trip_analyzer::{self, DayType},
};

use crate::models::{PricingRequest, PricingResponse, TimeslotPrices};

pub async fn get_current_and_next_prices(
    Json(payload): Json<PricingRequest>,
) -> Result<Json<PricingResponse>, (StatusCode, String)> {
    // 1. Parse date (YYYY-MM-DD)
    let date = NaiveDate::parse_from_str(&payload.date, "%Y-%m-%d").map_err(|e| {
        (
            StatusCode::BAD_REQUEST,
            format!("Invalid date format. Expected YYYY-MM-DD: {}", e),
        )
    })?;

    let day = date.day();
    let month = date.month();
    let year = date.year() as u32;

    // 2. Classify day (Holiday, Weekend, or Weekday)
    let day_type = if trip_analyzer::is_holiday(day, month, year) {
        DayType::Holiday
    } else if trip_analyzer::is_weekend(day, month, year) {
        DayType::Weekend
    } else {
        DayType::Weekday
    };

    // 3. Parse time to minutes
    // Try "HH:MM AM/PM" first, then "HH:MM:SS" (24h)
    let minutes = if let Some(m) = trip_analyzer::parse_time_to_minutes(&payload.time) {
        m
    } else {
        // Try parsing as HH:MM:SS or HH:MM
        let time_parts: Vec<&str> = payload.time.split(':').collect();
        if time_parts.len() >= 2 {
            let h: u32 = time_parts[0].parse().map_err(|_| {
                (
                    StatusCode::BAD_REQUEST,
                    "Invalid time format. Expected HH:MM AM/PM or HH:MM[:SS]".to_string(),
                )
            })?;
            let m: u32 = time_parts[1].parse().map_err(|_| {
                (
                    StatusCode::BAD_REQUEST,
                    "Invalid time format. Expected HH:MM AM/PM or HH:MM[:SS]".to_string(),
                )
            })?;
            if h < 24 && m < 60 {
                h * 60 + m
            } else {
                return Err((StatusCode::BAD_REQUEST, "Invalid time values".to_string()));
            }
        } else {
            return Err((
                StatusCode::BAD_REQUEST,
                "Invalid time format. Expected HH:MM AM/PM or HH:MM[:SS]".to_string(),
            ));
        }
    };

    // 4. Determine timeslot indices
    let (slots, eb_averages, wb_averages) = match day_type {
        DayType::Weekday => (
            &WEEKDAY_TIMESLOTS_2026[..],
            &WEEKDAY_EB_AVERAGE_TOLL_PRICES_2026[..],
            &WEEKDAY_WB_AVERAGE_TOLL_PRICES_2026[..],
        ),
        DayType::Weekend | DayType::Holiday => (
            &WEEKEND_TIMESLOTS_2026[..],
            &WEEKEND_EB_AVERAGE_TOLL_PRICES_2026[..],
            &WEEKEND_WB_AVERAGE_TOLL_PRICES_2026[..],
        ),
    };

    let slot_minutes: Vec<u32> = slots
        .iter()
        .filter_map(|&t| trip_analyzer::parse_time_to_minutes(t))
        .collect();

    if slot_minutes.is_empty() {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            "No timeslots defined".to_string(),
        ));
    }

    let mut current_idx = slot_minutes.len() - 1;
    for (i, &slot_time) in slot_minutes.iter().enumerate() {
        if minutes < slot_time {
            if i == 0 {
                current_idx = slot_minutes.len() - 1;
            } else {
                current_idx = i - 1;
            }
            break;
        }
        current_idx = i;
    }

    let next_idx = (current_idx + 1) % slot_minutes.len();

    let day_type_str = match day_type {
        DayType::Weekday => "Weekday",
        DayType::Weekend => "Weekend",
        DayType::Holiday => "Holiday",
    };

    Ok(Json(PricingResponse {
        current: TimeslotPrices {
            timeslot: slots[current_idx].to_string(),
            average_eb: eb_averages[current_idx],
            average_wb: wb_averages[current_idx],
        },
        next: TimeslotPrices {
            timeslot: slots[next_idx].to_string(),
            average_eb: eb_averages[next_idx],
            average_wb: wb_averages[next_idx],
        },
        day_type: day_type_str.to_string(),
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_pricing_weekday_morning() {
        let req = PricingRequest {
            date: "2026-04-13".to_string(), // Monday
            time: "06:30 AM".to_string(),
        };
        let response = get_current_and_next_prices(Json(req)).await.unwrap();
        let PricingResponse { current, next, day_type } = response.0;

        assert_eq!(day_type, "Weekday");
        assert_eq!(current.timeslot, "5:00 AM");
        assert_eq!(next.timeslot, "7:00 AM");
        // Weekday WB 2026 5am average is 69.10, 7am is 89.35
        assert_eq!(current.average_wb, 69.10);
        assert_eq!(next.average_wb, 89.35);
    }

    #[tokio::test]
    async fn test_get_pricing_weekend_wrap_around() {
        let req = PricingRequest {
            date: "2026-04-12".to_string(), // Sunday
            time: "10:00 PM".to_string(),
        };
        let response = get_current_and_next_prices(Json(req)).await.unwrap();
        let PricingResponse { current, next, day_type } = response.0;

        assert_eq!(day_type, "Weekend");
        assert_eq!(current.timeslot, "9:00 PM");
        assert_eq!(next.timeslot, "8:30 AM");
        // Weekend WB 2026 9pm is 50.56, 8:30am is 58.15
        assert_eq!(current.average_wb, 50.56);
        assert_eq!(next.average_wb, 58.15);
    }
}

// Simple macro-like replacement for json! since we don't need full serde_json::json! in this file's test scope
// actually, let's just use the real models to construct the tests.
