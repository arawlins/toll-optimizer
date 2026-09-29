# Architecture and Analysis Guide

This document provides a technical overview of the Toll Optimizer's internal architecture, its analysis algorithms, and the data models used to calculate 407 ETR toll savings.

## Architecture Overview

Toll Optimizer is a modular Rust binary crate organized into several specialized modules:

-   **`csv_parser.rs`**: Handles the ingestion of 407 ETR CSV exports. It maps raw CSV rows to internal `TollTrip` structures, performing initial data cleaning and validation.
-   **`trip_analyzer.rs`**: The core logic engine. It contains the rate schedules, holiday definitions, and the algorithms for time-based and distance-based optimization.
-   **`md_output.rs`**: Formats the analysis results into human-readable Markdown reports.
-   **`constants.rs`**: Stores static data such as access point names, distances, and 2026 rate tables.
-   **`vehicle_class/`**: Handles the logic for different vehicle types (Light, Medium, Heavy Single/Multiple, Motorcycle).

## Analysis Algorithms

### 1. Time-Based Clustering
The goal of time-based analysis is to identify groups of trips that occur at similar times and determine if shifting them would result in a lower "timeslot" rate.

-   **Clustering**: Trips are grouped by entry time using a simple window-based clustering approach. This helps identify "commute patterns."
-   **Optimization**: For each trip, the analyzer lookups the cost for the actual timeslot and compares it with the immediately preceding and following timeslots.
-   **Criteria**: A "Cheaper Prev" or "Cheaper Next" suggestion is generated only if the cost difference exceeds a threshold (typically $0.005) to filter out negligible fluctuations.

### 2. Distance-Based Optimization
This algorithm identifies frequent routes and checks if entering or exiting at an adjacent ramp would save money.

-   **Route Matching**: Trips are grouped by their Entry/Exit point pairs.
-   **Alternate Ramp Check**: The engine simulates the toll cost for the same trip but with the nearest logical entry or exit ramp. 
-   **Savings Logic**: In some cases, exiting one ramp earlier and taking a local road for the final kilometer is significantly cheaper because 407 ETR rates are calculated based on the timeslot of the *entry* point, and shorter distances in high-rate zones can be optimized.

## Data Models

### `TripRecord`
The primary data structure representing a single trip recorded on a statement.
- `entry_point` / `exit_point`: Human-readable ramp names.
- `entry_time`: The exact time the trip began.
- `distance_km`: Recorded distance.
- `toll_charge`: The actual amount billed by 407 ETR.

### `PricingResponse`
Used for live pricing lookups.
- `current`: The rate for the requested timeslot.
- `next`: The rate for the following timeslot.
- `day_type`: Categorized as Weekday, Weekend, or Holiday (which uses Weekend rates).

## Performance Architecture

The toll optimizer has been tuned for zero-cost abstractions, minimal heap allocations, and cache-friendly data structures:

1. **Direct Zone Indexing**: Zone rates are indexed directly via compile-time mapping without linear name scans or runtime hash lookups.
2. **Precomputed Timeslot Ranges**: Timeslots store precomputed integer minutes from midnight (`start_minutes`, `end_minutes`) for instant comparison during trip classification.
3. **Compile-time Holiday Table**: Statutory holidays are defined as a compile-time static `&[(u16, u8, u8)]` slice, eliminating runtime JSON parsing.
4. **Zero-Allocation Parsing & Advice**: Vehicle class matching and optimization advice string formatting reuse string slices and formatters without allocating temporary intermediate strings.
5. **Streamed I/O**: Markdown report generation writes into generic `std::io::Write` buffers wrapped with `BufWriter`, preventing micro-syscall overhead when outputting large statements.
6. **Benchmarks**: Core routines (CSV parsing, rate lookups, K-means clustering) are benchmarked under `benches/trip_benchmarks.rs` via `cargo bench`.