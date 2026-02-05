use std::env;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::process::Command;

/// Helper function to run a single test case in isolation.
///
/// * `test_id`: A unique identifier for the test (used for temp dir name).
/// * `csv_line`: The specific CSV data line to test.
/// * `expected_snippets`: A list of strings that MUST appear in the output.
fn run_single_trip_test(test_id: &str, csv_line: &str, expected_snippets: &[&str]) {
    // 1. Binary path (Assuming 'cargo test' has already built the binary)
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let bin_path = Path::new(&manifest_dir).join("../../target/debug/toll-optimizer-cli");

    // 2. Setup UNIQUE temp dir for this test case to avoid parallel collisions
    let temp_dir_name = format!("toll-optimizer-test-{}", test_id);
    let temp_dir = env::temp_dir().join(temp_dir_name);
    
    // Clean start
    if temp_dir.exists() {
        fs::remove_dir_all(&temp_dir).expect("Failed to clean up old temp dir");
    }
    fs::create_dir_all(&temp_dir).expect("Failed to create temp dir");

    let csv_dir = temp_dir.join("csv");
    fs::create_dir(&csv_dir).expect("Failed to create csv dir");

    // 3. Create CSV with the REQUIRED filename expected by main.rs
    let csv_file_path = csv_dir.join("2025-08-28 - 573522284 Statement.csv");
    let mut file = fs::File::create(&csv_file_path).expect("Failed to create CSV file");

    let header = r#"NOTE: Test Pricing
Name: Test User

"Transponder/Plate Number","Vehicle Class","Date of Trip","Entry Time","Entry Point","Exit Point","Distance (km)","Toll Charge ($)","Trip Toll Charge ($)","Camera Charge ($)"
"#;
    let content = format!("{}{}\n", header, csv_line);
    file.write_all(content.as_bytes()).expect("Failed to write CSV");

    // 4. Run binary
    let output = Command::new(&bin_path)
        .current_dir(&temp_dir)
        .output()
        .expect("Failed to run binary");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    // 5. Assertions
    if !output.status.success() {
        panic!("Application failed for test '{}' with status: {}.\nSTDERR: {}\nSTDOUT: {}", test_id, output.status, stderr, stdout);
    }

    for snippet in expected_snippets {
        if !stdout.contains(snippet) {
            panic!(
                "Test '{}' failed.\nExpected snippet not found: '{}'\nFull Output:\n{}",
                test_id, snippet, stdout
            );
        }
    }
}

#[test]
fn test_2025_weekday_pricing() {
    // Case 1: 2025 Weekday. 28 Aug 25 (Thursday).
    // Trip: QEW (Zone 1) -> Dundas (Zone 2). Eastbound.
    // Time: 7:00 AM (Index 2 in 2025 weekday slots).
    // Rate: 62.89 cents/km. Dist: 6.062 km. Cost: $3.81.
    let line = r#""CASE_1_2025_WD","Light vehicle","28 Aug 25","07:00 AM","QEW","Dundas","6.062","0.00","0.00","0.00""#;
    
    run_single_trip_test(
        "2025_weekday", 
        line, 
        &["CASE_1_2025_WD", "Calc: $3.81"]
    );
}

#[test]
fn test_2026_weekday_pricing() {
    // Case 2: 2026 Weekday. 28 Aug 26 (Friday).
    // Trip: Whites (Zone 12) -> Brock(Hwy7) (Zone 12). Eastbound.
    // Time: 7:00 AM (Index 1 in 2026 weekday slots).
    // Rate: 95.60 cents/km. Dist: 4.083 km. Cost: $3.90.
    let line = r#""CASE_2_2026_WD","Light vehicle","28 Aug 26","07:00 AM","Whites","Brock(Hwy7)","4.083","0.00","0.00","0.00""#;
    
    run_single_trip_test(
        "2026_weekday", 
        line, 
        &["CASE_2_2026_WD", "Calc: $3.90"]
    );
}

#[test]
fn test_2025_weekend_pricing() {
    // Case 3: 2025 Weekend. 30 Aug 25 (Saturday).
    // Trip: Dundas (Zone 1/2) -> QEW (Zone 1). Westbound.
    // Time: 10:00 AM (Index 2 in 2025 weekend slots).
    // Rate: 45.56 cents/km. Dist: 6.062 km. Cost: $2.76.
    let line = r#""CASE_3_2025_WE","Light vehicle","30 Aug 25","10:00 AM","Dundas","QEW","6.062","0.00","0.00","0.00""#;
    
    run_single_trip_test(
        "2025_weekend", 
        line, 
        &["CASE_3_2025_WE", "Calc: $2.76"]
    );
}

#[test]
fn test_2026_weekend_pricing() {
    // Case 4: 2026 Weekend. 29 Aug 26 (Saturday).
    // Trip: Dundas -> QEW. Westbound.
    // Time: 10:00 AM (Index 1 in 2026 weekend slots).
    // Rate: 63.44 cents/km. Dist: 6.062 km. Cost: $3.85.
    let line = r#""CASE_4_2026_WE","Light vehicle","29 Aug 26","10:00 AM","Dundas","QEW","6.062","0.00","0.00","0.00""#;
    
    run_single_trip_test(
        "2026_weekend", 
        line, 
        &["CASE_4_2026_WE", "Calc: $3.85"]
    );
}

#[test]
fn test_holiday_edge_case() {
    // Case 5: Holiday (New Year). 01 Jan 25 (Wednesday).
    // Should use Weekend pricing ($2.76) instead of Weekday pricing.
    let line = r#""CASE_5_HOLIDAY","Light vehicle","01 Jan 25","10:00 AM","Dundas","QEW","6.062","0.00","0.00","0.00""#;
    
    run_single_trip_test(
        "holiday_edge_case", 
        line, 
        &["CASE_5_HOLIDAY", "Calc: $2.76", "[Holiday]"]
    );
}

