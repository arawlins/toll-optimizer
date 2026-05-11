# CLI Architecture (Revised)

## Overview
The **Toll Optimizer CLI** is a standalone Rust command-line utility designed to analyze 407 ETR toll statements locally. It combines CSV parsing, toll calculation, and K-Means clustering logic into a single, high-performance binary for cross-platform distribution.

## Component Structure

### 1. Entry Point (`src/main.rs`)
-   **Responsibility**: Manages the CLI lifecycle using `clap`.
-   **Key Features**:
    -   **Argument Parsing**: Uses `clap` for robust input handling (`--verbose`, `--json`, etc.).
    -   **Error Handling**: Employs `anyhow` for user-friendly error messages and context.
    -   **Reporting**: Generates formatted terminal reports summarizing savings by time and distance.

### 2. Logic Modules (`src/`)
The binary includes the following internal modules:
-   **`csv_parser.rs`**: Converts raw CSV data into structured `TripRecord` entities. It handles entry/exit point normalization and direction detection.
-   **`trip_analyzer.rs`**: The core engine. It performs time-based and distance-based clustering using K-Means and identifies optimization targets (cheaper timeslots or alternate routes).
-   **`constants.rs`**: Contains the 407 ETR network topology, including entry/exit points, zone boundaries, and distance matrices.
-   **`vehicle_class/`**: Specialized modules for calculating tolls based on vehicle type (Light, Medium, Heavy single/multiple, and Motorcycles).

## Data Flow
1.  **Input**: User provides a file path via CLI -> `PathBuf`.
2.  **Ingestion**: `std::fs::File` -> `csv_parser` processes rows into `TripRecord` structs grouped by transponder.
3.  **Analysis**:
    -   **Time Analysis**: Identifies temporal clusters and suggests shifting trips to adjacent cheaper timeslots.
    -   **Distance Analysis**: Identifies geographic clusters and suggests adjusting entry/exit points for savings.
4.  **Output**: `stdout` (Rich text reports with `--verbose` details) or future JSON serialization.

## Distribution & Build
The application is distributed as a pre-compiled, standalone binary via GitHub Actions.
-   **Build Command**: `cargo build --release`
-   **Cross-Compilation**: Supports Linux (musl), Windows (msvc), and macOS (Universal) via target-specific CI/CD jobs.
-   **Target Targets**:
    -   `x86_64-unknown-linux-musl`
    -   `x86_64-pc-windows-msvc`
    -   `x86_64-apple-darwin` & `aarch64-apple-darwin` (Universal)
