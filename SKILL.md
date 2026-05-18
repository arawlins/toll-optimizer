---
name: toll-optimizer
description: Analyze 407 ETR toll statements to find savings through time-shifting or route adjustments, and check live pricing. Use when analyzing monthly CSV statements or planning trips with live pricing information.
---

# Skill: Toll Optimizer

Analyze 407 ETR toll statements to find savings through time-shifting or route adjustments, and check live pricing.

## Purpose
This skill allows an LLM to autonomously process toll statements and provide actionable financial advice to a user. It can detect when a trip occurred just minutes after a cheaper rate period ended, when an alternate exit would have saved money, or provide live pricing comparisons for trip planning.

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
  Detailed trip listings are included by default.

**Pricing & Trip Options:**
- `--current-price`: Display pricing info for the current timeslot and provide optimization tips.
- `--entry <POINT> --exit <POINT>`: Calculate the cost for a single trip between two points. (Both required if either is specified).
- `--date <DATE>`: (Optional) Override date/time for pricing or single trip (YYYY-MM-DD).
- `--time <TIME>`: (Optional) Override time for pricing or single trip (HH:MM AM/PM or HH:MM).
- `--vehicle-class <CLASS>`: (Optional) Vehicle class for pricing or single trip (e.g., "Light vehicle", "Heavy Single Unit", "Heavy Multiple Unit", "Medium Vehicle", "Motorcycle"). Default: "Light vehicle".

## Procedures

### 1. Monthly Analysis
To perform a standard monthly review:
1.  Run the optimizer with the `--json` flag:
    `toll-optimizer --json "<filename>.csv"`
    NOTE: ALWAYS use the file specified in `<filename>.csv`. If the file cannot be found then DO NOT look for it, just let the user know that the file was not found. NEVER TRUNCATE THE OUTPUT.
2.  Parse the `summary` object for the following fields:
    - `total_potential_distance_savings` (rounded to 2 decimal places) - the total potential distance savings
    - `total_potential_time_savings` (rounded to 2 decimal places) - the total potential time savings
    - `total_cost` (rounded to 2 decimal places) - the total cost of the bill
    - `total_processed` - the number of trips processed
    The report should ALWAYS include the total cost of the bill and the number of trips processed. The savings should ALWAYS be separated into time-based and distance-based savings. NEVER add them together. For details in time-based savings see step 3 and for details in distance-based savings see step 4.
3.  Parse the `time_based_analysis` to find "Cheaper Prev" or "Cheaper Next" opportunities.
4.  Parse the `distance_based_analysis` to find route optimization advice (e.g., "Exit on Warden to save $1.38").

### 2. Live Pricing & Planning
If a user asks about current rates or planning a trip:
1.  Run the pricing check:
    `toll-optimizer --current-price --json`
2.  For a specific time:
    `toll-optimizer --current-price --date 2026-05-12 --time "07:30 AM" --json`
3.  Compare "Current Timeslot" and "Next Timeslot" averages to provide leaving/waiting advice.

### 3. Single Trip Calculation
If a user asks for the cost of a specific trip:
1.  Identify the entry and exit points.
2.  Run the calculation:
    `toll-optimizer --entry "McCowan" --exit "Hwy404" --json`
3.  Add `--date`, `--time`, or `--vehicle-class` if specific context is provided.

### 4. Verification
If a user asks about supported routes or points:
1.  Run the access point list:
    `toll-optimizer --list-access-points --json`
    NOTE: This can help find entry and exit points when calculating a single trip cost.
2.  Run the timeslot list:
    `toll-optimizer --list-timeslots --json`
    NOTE: This can help find other timeslots for live pricing.

## Success Criteria
- [ ] Correct mode identified (Analysis, Pricing, or Listing).
- [ ] Commands executed successfully.
- [ ] Savings identified or pricing comparisons presented clearly.
- [ ] Actionable advice provided (e.g., "Wait until 9:30 AM to save $2.50").
