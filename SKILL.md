# Skill: Toll Optimizer

Analyze 407 ETR toll statements to find savings through time-shifting or route adjustments.

## Purpose
This skill allows an LLM to autonomously process toll statements and provide actionable financial advice to a user. It can detect when a trip occurred just minutes after a cheaper rate period ended or when an alternate exit would have saved money.

## Available Tools

### `toll-optimizer` (CLI)
The primary tool for analysis.

**Usage:**
`cargo run -- [OPTIONS] <FILE>`

**Arguments:**
- `<FILE>`: Path to the 407 ETR CSV statement file.
- `--json`: (Recommended for LLMs) Outputs structured analysis data.
- `--verbose`: Shows detailed trip listings in human-readable output.

## Procedures

### 1. Monthly Analysis
To perform a standard monthly review:
1.  Identify the most recent statement in the `csv/` directory.
2.  Run the optimizer with the `--json` flag:
    `cargo run -- --json "csv/<filename>.csv"`
3.  Parse the `time_based_analysis` to find "Cheaper Prev" or "Cheaper Next" opportunities.
4.  Parse the `distance_based_analysis` to find route optimization advice (e.g., "Exit on Warden to save some $$$").

### 2. Deep Dive
If a user asks why a specific trip was expensive:
1.  Run with `--verbose` and grep for the date or entry point.
2.  Use the `detailed_trip_validation` section to verify if the recorded toll matches the calculated toll (detecting overcharges).

## Success Criteria
- [ ] Correct CSV file identified.
- [ ] Analysis run successfully.
- [ ] Savings identified and presented in a clear, summarized format.
- [ ] (Optional) Actionable advice provided for future trips (e.g., "Start your commute 5 minutes earlier").
