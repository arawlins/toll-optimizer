//! Toll statement parsing and optimization logic for the `toll-optimizer` CLI.
//!
//! This crate is primarily designed to support the `toll-optimizer` command-line
//! tool. The public API is focused on supporting the binary and its integration
//! tests and is not intended for general-purpose library use.
//!
//! # Core Functionality
//! - **Parsing**: Loading 407 ETR CSV statements.
//! - **Analysis**: Clustering trips by time and distance to find optimization opportunities.
//! - **Pricing**: Calculating costs for single trips or retrieving live pricing info.
//! - **Reporting**: Generating Markdown or JSON reports.

mod constants;
mod csv_parser;
mod md_output;
mod trip_analyzer;
mod vehicle_class;

// Explicitly re-export only what is required for the CLI and integration tests.

/// Access point names for the 407 ETR.
pub use constants::ACCESS_POINTS;

/// CSV statement parsing logic.
pub use csv_parser::{ParseResult, parse_trips};

/// Markdown report generation and report models.
pub use md_output::{
    print_markdown, print_pricing_markdown, print_single_trip_markdown,
    AnalysisMarkdownReport, SingleTripMarkdownReport,
};

/// Analysis and pricing core.
pub use trip_analyzer::{
    DayType, Direction, TripRecord, VehicleClass,
    analyze_trips_by_distance, analyze_trips_by_time,
    calculate_single_trip_cost, get_pricing,
    TransponderSummaryByDistance, TransponderSummaryByTime,
    CentroidData, CentroidDataByDistance, TripSummary,
    format_minutes_to_time, parse_time_to_minutes,
};
