# Current CLI Architecture

## Overview
The `toll-optimizer` is currently a monolithic command-line interface (CLI) tool designed to analyze toll statement CSVs. It processes trip data to calculate potential savings by optimizing entry/exit points and times based on historical and projected pricing.

## Component Structure

### 1. Entry Point (`src/main.rs`)
-   **Responsibility**: Orchestrates the application flow.
-   **Key Logic**:
    -   Scans the `csv/` directory for a *hardcoded* filename (`2025-08-28 - 573522284 Statement.csv`).
    -   Reads the file content into memory.
    -   Invokes `csv_parser::parse_trips` to structure the raw data.
    -   Invokes `trip_analyzer::analyze_trips_by_time` and `trip_analyzer::analyze_trips_by_distance`.
    -   Prints formatted text reports to `stdout`, including:
        -   Trip clusters (time-based).
        -   Cost comparisons (previous/next timeslots).
        -   Projected 2026 costs.
        -   Distance-based optimizations.

### 2. CSV Parser (`src/csv_parser.rs`)
-   **Responsibility**: Converts raw CSV lines into `TripRecord` structs.
-   **Logic**:
    -   Iterates through lines, skipping headers and metadata.
    -   Filters for "Light vehicle" class.
    -   Normalizes access point names using `ACCESS_POINT_SYNONYMS`.
    -   Determines trip direction (Eastbound/Westbound) based on entry/exit indices in `ACCESS_POINTS`.
    -   Groups trips by Transponder Plate and Direction.

### 3. Trip Analyzer (`src/trip_analyzer.rs`)
-   **Responsibility**: The core domain logic for clustering and cost calculation.
-   **Data Structures**:
    -   `TripRecord`: Represents a single trip.
    -   `TripSummary`: Enriched trip data with optimization contexts.
    -   `CentroidData`: Aggregated statistics for a cluster of trips.
-   **Algorithms**:
    -   **K-Means Clustering (1D)**: Groups trips by entry time (minutes from midnight) or distance to identify patterns.
    -   **Elbow Method**: Determines the optimal number of clusters (`k`).
    -   **Cost Calculation**:
        -   Uses complex pricing tables (2025 vs 2026, Weekday vs Weekend).
        -   Calculates "Previous Timeslot" and "Next Timeslot" costs to suggest timing adjustments (e.g., "Wait 5 mins to save $3.00").
    -   **Distance Optimization**: Checks if moving the Entry or Exit point by one interchange (within valid logic) reduces the cost.

### 4. Vehicle Class & Constants (`src/vehicle_class/`, `src/constants.rs`)
-   **Responsibility**: specific configuration data.
-   **Content**:
    -   `light_vehicles.rs`, `heavy_single_unit.rs`, etc.: Huge static arrays defining toll prices by Zone and Timeslot for different years.
    -   `constants.rs`: Access point names, distances, zone mappings, and timeslot definitions.

## Data Flow
1.  **Input**: Local File System (`csv/*.csv`) -> `Vec<String>` (Raw Lines).
2.  **Parse**: Raw Lines -> `HashMap<(Plate, Direction), Vec<TripRecord>>`.
3.  **Analyze**: Map -> `Vec<TransponderSummary>`.
4.  **Output**: `stdout` (Human-readable text).

## Limitations (Refactoring Targets)
-   **Hardcoded Input**: The input file name is hardcoded in `main.rs`.
-   **Tight Coupling**: `main.rs` directly depends on file system I/O *and* specific business logic.
-   **Output Format**: Only prints text to console; no structured output (JSON) for API consumption.
-   **State**: Entirely stateless run-to-completion script; no persistence.
