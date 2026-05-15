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
- `--verbose`: Shows detailed trip listings or optimization advice in human-readable output.

**Analysis Options (Requires `<FILE>`):**
- `<FILE>`: Path to the 407 ETR CSV statement file.

**Pricing Options:**
- `--current-price`: Display pricing info for the current timeslot and provide optimization tips.
- `--date <DATE>`: (Optional) Override date for pricing (YYYY-MM-DD).
- `--time <TIME>`: (Optional) Override time for pricing (HH:MM AM/PM or HH:MM).

## Procedures

### 1. Monthly Analysis
To perform a standard monthly review:
1.  Identify the most recent statement in the `csv/` directory.
2.  Run the optimizer with the `--json` flag:
    `toll-optimizer --json "csv/<filename>.csv"`
3.  Parse the `time_based_analysis` to find "Cheaper Prev" or "Cheaper Next" opportunities.
4.  Parse the `distance_based_analysis` to find route optimization advice (e.g., "Exit on Warden to save some $$$").

### 2. Live Pricing & Planning
If a user asks about current rates or planning a trip:
1.  Run the pricing check:
    `toll-optimizer --current-price`
2.  For a specific time:
    `toll-optimizer --current-price --date 2026-05-12 --time "07:30 AM"`
3.  Compare "Current Timeslot" and "Next Timeslot" averages to provide leaving/waiting advice.

### 3. Deep Dive
If a user asks why a specific trip was expensive:
1.  Run with `--verbose` and search for the date or entry point.
2.  Use the `detailed_trip_validation` section to verify if the recorded toll matches the calculated toll (detecting overcharges).

## Success Criteria
- [ ] Correct mode identified (Analysis vs. Pricing).
- [ ] Commands executed successfully.
- [ ] Savings identified or pricing comparisons presented clearly.
- [ ] Actionable advice provided (e.g., "Wait until 9:30 AM to save $2.50").
