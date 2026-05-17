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

### `TollTrip`
The primary data structure representing a single trip recorded on a statement.
- `entry_point` / `exit_point`: Human-readable ramp names.
- `entry_time`: The exact time the trip began.
- `distance_km`: Calculated or recorded distance.
- `toll_charge`: The actual amount billed by 407 ETR.

### `PricingResult`
Used for live pricing lookups.
- `current`: The rate for the requested timeslot.
- `next`: The rate for the following timeslot.
- `day_type`: Categorized as Weekday, Weekend, or Holiday (which uses Weekend rates).