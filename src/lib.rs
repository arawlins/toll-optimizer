//! Toll statement parsing and optimization logic for the `toll-optimizer` CLI.
//!
//! The crate exposes the same core functionality used by the command-line
//! application: CSV statement parsing, 407 ETR route pricing, trip clustering,
//! and report rendering helpers.
//!
//! # Example
//!
//! ```rust,no_run
//! use std::fs::File;
//!
//! let file = File::open("statement.csv")?;
//! let parsed = toll_optimizer::csv_parser::parse_trips(file);
//! let by_time = toll_optimizer::trip_analyzer::analyze_trips_by_time(&parsed.trips);
//! let by_distance = toll_optimizer::trip_analyzer::analyze_trips_by_distance(&parsed.trips);
//! # Ok::<(), std::io::Error>(())
//! ```

/// 407 ETR topology, timeslot, access-point, and zone constants.
pub mod constants;
/// CSV statement parser for 407 ETR exports.
pub mod csv_parser;
/// Markdown report rendering helpers.
pub mod md_output;
/// Trip pricing, day classification, and clustering analysis.
pub mod trip_analyzer;
/// Vehicle-class-specific toll rate tables.
pub mod vehicle_class;

/// Re-export of topology constants for CLI and downstream consumers.
pub use constants::*;
/// Common trip-analysis domain types.
pub use trip_analyzer::{DayType, Direction, TripRecord, VehicleClass};
/// Re-export of raw vehicle-class rate table modules.
pub use vehicle_class::*;
