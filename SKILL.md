---
name: toll-optimizer
description: Analyze 407 ETR toll statements to find savings through time-shifting or route adjustments, and check live pricing. Use when analyzing monthly CSV statements or planning trips with live pricing information.
---

# Skill: Toll Optimizer

Analyze 407 ETR toll statements to find savings through time-shifting or route adjustments, and check live pricing.

## Purpose
This skill allows an LLM to autonomously process toll statements and provide actionable financial advice. It detects optimization opportunities based on timing and route selection. It also provides live pricing comparisons for trip planning.

## Available Tools

### `toll-optimizer` (CLI)
The primary tool for analysis and pricing.

**Usage:**
`toll-optimizer [OPTIONS] [FILE]`

**Global Options:**
- `--json`: (Recommended for LLMs) Outputs structured analysis or pricing data.
- `--list-access-points`: Lists all recognized 407 ETR access points.
- `--list-timeslots`: Lists all 407 ETR pricing timeslots.

**Analysis Options (Requires `<FILE>`):**
- `<FILE>`: Path to the 407 ETR CSV statement file.
- `--show-summary`: Suppresses individual trip details. (Works with JSON outputs).

**Pricing & Trip Options:**
- `--current-price`: Display pricing info for the current timeslot.
- `--entry <POINT> --exit <POINT>`: Calculate the cost for a single trip.
- `--date <DATE>`: (Optional) Override date (YYYY-MM-DD).
- `--time <TIME>`: (Optional) Override time (HH:MM AM/PM).
- `--vehicle-class <CLASS>`: (Optional) Default: "Light vehicle".

## Procedures

### 1. Monthly Analysis
1.  **Run Tool**: `toll-optimizer --json --show-summary "<filename>.csv"`
2.  **Validate**: If file not found, report error and stop. NEVER truncate output.
3.  **Process Data**: Parse `summary`, `time_based_analysis`, and `distance_based_analysis`.
4.  **Format Output**: You MUST use the **Monthly Analysis Report Template** below.

### 2. Live Pricing
1.  **Check Rates**: `toll-optimizer --current-price --json`
2.  **Options**: Override date or time to check pricing for a specific time: `toll-optimizer --current-price --date 2026-05-12 --time "07:30 AM" --json` or `toll-optimizer --current-price --vehicle-class "Medium vehicle" --date 2026-05-12 --time "07:30 AM" --json`.

### 3. Single Trip Calculation
If a user asks for the cost of a specific trip:
1.  **Identify points**: Run `toll-optimizer --list-access-points` if needed.
2.  **Run calculation**: `toll-optimizer --entry "<POINT>" --exit "<POINT>" --date "<DATE>" --time "<TIME>" --json`


## Output Format Patterns

### Standard Monthly Analysis Report Template
When providing a monthly analysis, the response MUST strictly follow this Markdown structure:

**1. Statement Summary**
| Metric | Value |
| :--- | :--- |
| **Total Cost of Bill** | $[total_cost] |
| **Total Trips Processed** | [total_processed] |
| **Potential Distance-Based Savings** | $[total_potential_distance_savings] |
| **Potential Time-Based Savings** | $[total_potential_time_savings] |

**2. Distance-Based Optimization Advice**
(Parse `distance_based_analysis` and list specific location-based advice)
- Example: "Entering at **[Point A]** instead of **[Point B]** would save **$[amount]**."

**3. Time-Based Optimization Advice**
(Parse `time_based_analysis` and list specific timing-based advice)
- Example: "Starting your afternoon commute **after 6:00 PM** would save **$[amount]**."

## Success Criteria
- [ ] Total cost and trips processed are included in a table.
- [ ] Distance and Time savings are NEVER added together; they are listed separately.
- [ ] Advice includes specific entry/exit points or clock times.
- [ ] Final report adheres to the Markdown template.
