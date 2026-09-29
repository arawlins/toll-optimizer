use toll_optimizer::{format_minutes_to_time, parse_time_to_minutes};

#[test]
fn test_parse_time_to_minutes() {
    assert_eq!(parse_time_to_minutes("12:00 AM"), Some(0));
    assert_eq!(parse_time_to_minutes("12:01 AM"), Some(1));
    assert_eq!(parse_time_to_minutes("1:00 AM"), Some(60));
    assert_eq!(parse_time_to_minutes("11:59 AM"), Some(11 * 60 + 59));
    assert_eq!(parse_time_to_minutes("12:00 PM"), Some(12 * 60));
    assert_eq!(parse_time_to_minutes("12:30 PM"), Some(12 * 60 + 30));
    assert_eq!(parse_time_to_minutes("1:00 PM"), Some(13 * 60));
    assert_eq!(parse_time_to_minutes("11:59 PM"), Some(23 * 60 + 59));
}

#[test]
fn test_parse_time_invalid() {
    assert_eq!(parse_time_to_minutes("12:00"), None);
    assert_eq!(parse_time_to_minutes("AM"), None);
    assert_eq!(parse_time_to_minutes("13:00 AM"), None); // Logic handles 13 % 12, but input is weird
    assert_eq!(parse_time_to_minutes("abc"), None);
}

#[test]
fn test_format_minutes_to_time() {
    assert_eq!(format_minutes_to_time(0), "12:00 AM");
    assert_eq!(format_minutes_to_time(1), "12:01 AM");
    assert_eq!(format_minutes_to_time(60), "1:00 AM");
    assert_eq!(format_minutes_to_time(12 * 60), "12:00 PM");
    assert_eq!(format_minutes_to_time(13 * 60), "1:00 PM");
    assert_eq!(format_minutes_to_time(23 * 60 + 59), "11:59 PM");
}

#[test]
fn test_write_single_trip_markdown() {
    use toll_optimizer::{
        DayType, Direction, SingleTripMarkdownReport, write_single_trip_markdown,
    };

    let mut buffer = Vec::new();
    let report = SingleTripMarkdownReport {
        entry: "QEW",
        exit: "Highway 401",
        date: "2025-01-15",
        time: "08:00 AM",
        class: "Regular",
        distance_km: 25.5,
        direction: &Direction::Eastbound,
        day_type: &DayType::Weekday,
        cost: 15.30,
    };
    write_single_trip_markdown(&mut buffer, report).expect("write markdown failed");
    let output = String::from_utf8(buffer).expect("valid utf8");
    assert!(output.contains("# Toll Optimizer Single Trip Report"));
    assert!(output.contains("QEW -> Highway 401"));
    assert!(output.contains("$16.30"));
}

#[test]
fn test_write_pricing_markdown() {
    use toll_optimizer::{PricingResponse, TimeslotPrices, write_pricing_markdown};

    let mut buffer = Vec::new();
    let pricing = PricingResponse {
        current: TimeslotPrices {
            timeslot: "8:00 AM".to_string(),
            average_wb: 55.0,
            average_eb: 57.0,
        },
        next: TimeslotPrices {
            timeslot: "9:00 AM".to_string(),
            average_wb: 50.0,
            average_eb: 52.0,
        },
        day_type: "Weekday (Regular)".to_string(),
    };
    write_pricing_markdown(&mut buffer, &pricing, "2025-01-15", "8:15 AM")
        .expect("write pricing failed");
    let output = String::from_utf8(buffer).expect("valid utf8");
    assert!(output.contains("# Toll Optimizer Live Pricing Report"));
    assert!(output.contains("Weekday (Regular)"));
    assert!(output.contains("55.00¢/km"));
}

#[test]
fn test_write_markdown_processing_summary() {
    use std::collections::HashMap;
    use toll_optimizer::{AnalysisMarkdownReport, write_markdown};

    let mut buffer = Vec::new();
    let camera_charges = HashMap::new();
    let report = AnalysisMarkdownReport {
        summaries_by_time: &[],
        summaries_by_distance: &[],
        total_processed: 5,
        total_skipped: 1,
        total_cost: 42.50,
        total_time_savings: 5.00,
        total_distance_savings: 2.50,
        unknown_points: &[],
        unknown_vehicle_classes: &[],
        camera_charges: &camera_charges,
        show_summary: false,
    };
    write_markdown(&mut buffer, report).expect("write markdown failed");
    let output = String::from_utf8(buffer).expect("valid utf8");
    assert!(output.contains("# Toll Optimizer Analysis Report"));
    assert!(output.contains("| Trips Processed | 5 |"));
    assert!(output.contains("| Total Bill Cost | $42.50 |"));
}
