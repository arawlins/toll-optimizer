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
-   `-j, --json`: Outputs the analysis in a structured JSON format. This is the **recommended mode for LLMs** and automated reporting pipelines as it provides precise metadata (savings, optimization targets, and notes) without visual formatting overhead.
-   `-m, --markdown`: Outputs the analysis in a clean Markdown format with tables, suitable for copying into reports or GitHub issues.
-   `-h, --help`: Displays usage information.

## Live Pricing and Trip Planning
Get real-time 407 ETR rate lookups and optimization advice for your current or planned trip.

```bash
# Get rates for the current time
toll-optimizer --current-price

# Check rates for a specific future date and time
toll-optimizer --current-price --date 2026-05-12 --time "07:30 AM"

# Specify a vehicle class (Default: "Light vehicle")
toll-optimizer --current-price --vehicle-class "Heavy Single Unit"
```

### Single Trip Cost Calculation
Calculate the exact cost of a trip between any two recognized points.

```bash
# Calculate cost for current time
toll-optimizer --entry "McCowan" --exit "Hwy404"

# Calculate cost for a specific time and vehicle class
toll-optimizer --entry "QEW" --exit "Dundas" --date 2026-05-12 --time "08:00 AM" --vehicle-class "Heavy Single Unit"
```

The live pricing check provides:
- **Current Timeslot**: The rate you would pay if you entered the highway right now.
- **Next Timeslot**: The rate for the following timeslot, helping you decide whether to "Leave now" or "Wait".
- **Optimization Tip**: A clear directive (e.g., "Wait for the next timeslot to save money!").

## Understanding the Results

The tool provides three output formats: **Standard (Text)**, **JSON**, and **Markdown**.

### 1. Markdown Output (`--markdown`)
The Markdown output is designed for human readability in contexts like GitHub, GitLab, or any Markdown-supporting editor. It organizes the analysis into a structured report:
- **Tables**: Key metrics like Average Entry Time, Total Distance, and Toll Charges are presented in tables.
- **Detailed Trip Tables**: Individual trips are listed in a table format, including date, route, and specific optimization suggestions.
- **Headers**: Uses standard H1-H4 headers to separate time-based and distance-based analysis.

### 2. Time-Based Clustering (Standard/Markdown)
The tool groups your trips into clusters based on their **entry time**.
-   **Centroid Time**: The average time for trips in that cluster.
-   **Cheaper Prev/Next**: Indicates if leaving a few minutes earlier or later would move the trip into a significantly cheaper timeslot.
-   **Potential Savings**: The sum of all savings if you shifted every trip in that cluster to the recommended timeslot.

### 2. Distance-Based Clustering
The tool identifies routes you take frequently and suggests alternate entry or exit points.
-   **Example**: `Hwy404 -> Kennedy (Avg: 5.04 km)`
-   **Optimization Note**: `[Exit on Warden to save $1.42]`
-   **Rationale**: Sometimes exiting one ramp earlier and driving a short distance on a surface road can save a disproportionate amount in toll charges, especially during peak hours when per-km rates are high.

### 3. 2026 Projections
The CLI automatically calculates the cost of your current trips using the announced **2026 rate schedules**, helping you budget for future price increases.

## Troubleshooting

-   **"File not found"**: Ensure the path to your CSV is correct and that the file is not currently open in another program (like Excel) which might be locking it.
-   **"No trips parsed"**: Check that the CSV file is a standard 407 ETR export. The tool looks for specific headers. If your file is a manual export or modified, it might fail to detect the columns.
