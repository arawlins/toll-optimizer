# Task Completion: Pricing Logic & Edge Case Tests

## Date
2026-01-26

## Task
Create integration tests for 2025 and 2026 pricing logic, including edge cases.

## Outcome
- Analyzed `src/vehicle_class/light_vehicles.rs` to extract pricing tables for verification.
- Created `tests/pricing_logic_test.rs`:
    - **Methodology**: Black-box testing using `std::process::Command` and a temp directory.
    - **Data Injection**: Created a CSV file named `2025-08-28 - 573522284 Statement.csv` (to bypass filename filter in `main.rs`) but injected content with dates from 2025 and 2026.
    - **Scenarios Covered**:
        - 2025 Weekday (Zone 1 -> Zone 2).
        - 2026 Weekday (Zone 12 -> Zone 12).
        - 2025 Weekend (Zone 1 -> Zone 1).
        - 2026 Weekend (Zone 1 -> Zone 1).
        - Holiday (New Year 2025) treated as Weekend.
    - **Verification**: Parsed stdout for specific calculated costs ("Calc: $X.XX") and confirmed they match manual calculations based on source tables.
- Verified test passes with `cargo test --test pricing_logic_test`.
- Refactored `pricing_logic_test.rs` to separate test cases into individual functions (`test_2025_weekday`, `test_2026_weekday`, etc.) for better isolation and reporting.
- Removed `cargo build` from individual test execution to prevent race conditions during parallel test runs.

## Key Learnings
- Testing 2026 logic was possible even with a hardcoded input filename filter because the logic relies on the *content* dates, not the filename.
- The `csv` directory structure in the temp environment is critical for the binary to function.
