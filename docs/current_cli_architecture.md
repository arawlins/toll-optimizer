# CLI Architecture

## Overview
The `toll-optimizer-cli` is a command-line utility designed to analyze toll statement CSVs locally. It leverages the shared `toll-optimizer-core` library to process trip data, calculate potential savings, and suggest optimizations.

## Component Structure

### 1. Entry Point (`crates/cli/src/main.rs`)
-   **Responsibility**: Orchestrates the application flow for local execution.
-   **Key Logic**:
    -   Accepts a CSV file path as a command-line argument.
    -   Reads the file content into memory.
    -   Invokes `csv_parser::parse_trips` to structure the raw data.
    -   Invokes `trip_analyzer::analyze_trips_by_time` and `trip_analyzer::analyze_trips_by_distance`.
    -   Prints formatted text reports to `stdout`, including:
        -   Trip clusters (time-based and distance-based).
        -   Cost comparisons (previous/next timeslots).
        -   Projected 2026 costs.
        -   Distance-based optimizations with suggested route mappings.

### 2. Core Library (`crates/core/`)
The CLI depends on the shared core library for all business logic:
-   **CSV Parser (`src/csv_parser.rs`)**: Converts raw CSV lines into `TripRecord` structs. Handles name normalization and direction determination. Logs unknown entry/exit points via `tracing`.
-   **Trip Analyzer (`src/trip_analyzer.rs`)**: Contains the domain logic for clustering (K-Means) and cost optimization.
-   **Constants (`src/constants.rs`)**: Centralized access point names, distances, zone mappings, and timeslot definitions.

## Data Flow
1.  **Input**: Command-line argument (path to `.csv`) -> `Vec<String>` (Raw Lines).
2.  **Parse**: Raw Lines -> `Vec<((String, Direction), Vec<TripRecord>)>`.
3.  **Analyze**: Parsed Results -> `Vec<TransponderSummaryByTime>` and `Vec<TransponderSummaryByDistance>`.
4.  **Output**: `stdout` (Human-readable text reports).

## Usage
Run the CLI using Cargo, providing the path to a statement:
```bash
cargo run -p toll-optimizer-cli -- <path_to_csv>
```
-   **State**: Entirely stateless run-to-completion script; no persistence.
