# Toll Optimizer CLI: User Guide

This guide explains how to use the Toll Optimizer CLI to analyze your 407 ETR statements and find ways to save on your monthly tolls.

## Getting Started

### 1. Obtain your 407 ETR Statement
Log in to your 407 ETR account and download your transaction history as a **CSV file**. Ensure you select a date range that covers the trips you want to analyze.

### 2. Run the Analysis
Run the `toll-optimizer` binary (either downloaded or built from source) by passing the path to your CSV file:

```bash
toll-optimizer statement.csv
```

## Command-Line Options

-   `<FILE>`: **(Required)** Path to the CSV file.
-   `-v, --verbose`: Enables detailed output. This includes:
    -   A full breakdown of every trip within each cluster.
    -   Specific optimization notes (e.g., "Exit on Warden to save $1.42").
    -   Projected 2026 cost for every individual trip.
-   `-j, --json`: Outputs the analysis in a machine-readable JSON format (useful for piping into other tools).
-   `-h, --help`: Displays usage information.

## Understanding the Results

### 1. Time-Based Clustering
The tool groups your trips into clusters based on their **entry time**.
-   **Centroid Time**: The average time for trips in that cluster.
-   **Cheaper Prev/Next**: Indicates if leaving a few minutes earlier or later would move the trip into a significantly cheaper timeslot.
-   **Potential Savings**: The sum of all savings if you shifted every trip in that cluster to the recommended timeslot.

### 2. Distance-Based Clustering
The tool identifies routes you take frequently and suggests alternate entry or exit points.
-   **Example**: `Hwy404 -> Kennedy (Avg: 5.04 km)`
-   **Optimization Note**: `[Exit on Warden to save some $$$ (Save $1.42)]`
-   **Rationale**: Sometimes exiting one ramp earlier and driving a short distance on a surface road can save a disproportionate amount in toll charges, especially during peak hours when per-km rates are high.

### 3. 2026 Projections
The CLI automatically calculates the cost of your current trips using the announced **2026 rate schedules**, helping you budget for future price increases.

## Troubleshooting

-   **"File not found"**: Ensure the path to your CSV is correct and that the file is not currently open in another program (like Excel) which might be locking it.
-   **"No trips parsed"**: Check that the CSV file is a standard 407 ETR export. The tool looks for specific headers. If your file is a manual export or modified, it might fail to detect the columns.
