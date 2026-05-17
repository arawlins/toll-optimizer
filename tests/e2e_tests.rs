use std::process::Command;
use std::str;

fn run_optimizer(args: &[&str]) -> String {
    let output = Command::new("cargo")
        .arg("run")
        .arg("--quiet")
        .arg("--")
        .args(args)
        .output()
        .expect("Failed to execute command");

    assert!(
        output.status.success(),
        "Command failed with status: {:?}\nStderr: {}",
        output.status,
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8_lossy(&output.stdout).to_string()
}

#[test]
fn test_e2e_light_vehicles() {
    let output = run_optimizer(&["tests/csv/2025-08-28 - light vehicles.csv"]);

    assert!(output.contains("--- Processing Summary ---"));
    assert!(output.contains("Trips Processed: 62"));
    assert!(output.contains("Trips Skipped:   0"));
    assert!(output.contains("Total Bill Cost: $438.35"));
    assert!(output.contains("--- Time-Based Analysis ---"));
    assert!(output.contains("--- Distance-Based Analysis ---"));
}

#[test]
fn test_e2e_invalid_entry() {
    let output = run_optimizer(&["tests/csv/2026-02-28 - invalid entry.csv"]);

    assert!(output.contains("--- Processing Summary ---"));
    assert!(output.contains("Trips Processed: 6"));
    assert!(output.contains("Trips Skipped:   3"));
    assert!(output.contains("Unrecognized Access Points:"));
    assert!(output.contains("- Doober | NOT RECOGNIZED"));
    assert!(output.contains("- Goober | NOT RECOGNIZED"));
    assert!(output.contains("Unrecognized Vehicle Classes:"));
    assert!(output.contains("- Space Shuttle | NOT RECOGNIZED"));
}

#[test]
fn test_e2e_mixed_case() {
    let output = run_optimizer(&["tests/csv/2026-04-28 - mixed case.csv"]);

    assert!(output.contains("--- Processing Summary ---"));
    assert!(output.contains("Trips Processed: 17"));
    assert!(output.contains("Trips Skipped:   0"));
    assert!(output.contains("Total Bill Cost: $389.23"));
}

#[test]
fn test_e2e_no_preamble() {
    let output = run_optimizer(&["tests/csv/2026-02-28 - no preamble.csv"]);

    assert!(output.contains("--- Processing Summary ---"));
    assert!(output.contains("Trips Processed: 8"));
    assert!(output.contains("Trips Skipped:   0"));
}

#[test]
fn test_e2e_json_output() {
    let output = run_optimizer(&["tests/csv/2026-02-28 - invalid entry.csv", "--json"]);

    let json: serde_json::Value =
        serde_json::from_str(&output).expect("Output should be valid JSON");
    assert_eq!(json["summary"]["total_processed"], 6);
    assert_eq!(json["summary"]["total_skipped"], 3);
    assert!(
        json["summary"]["unknown_points"]
            .as_array()
            .unwrap()
            .contains(&serde_json::json!("Goober"))
    );
    assert!(
        json["summary"]["unknown_points"]
            .as_array()
            .unwrap()
            .contains(&serde_json::json!("Doober"))
    );
    assert!(
        json["summary"]["unknown_vehicle_classes"]
            .as_array()
            .unwrap()
            .contains(&serde_json::json!("Space Shuttle"))
    );
}

#[test]
fn test_e2e_markdown_output() {
    let output = run_optimizer(&["tests/csv/2026-02-28 - invalid entry.csv", "--markdown"]);

    assert!(output.contains("# Toll Optimizer Analysis Report"));
    assert!(output.contains("## Processing Summary"));
    assert!(output.contains("| Trips Processed | 6 |"));
    assert!(output.contains("| Trips Skipped | 3 |"));
    assert!(output.contains("### Unrecognized Vehicle Classes"));
    assert!(output.contains("- Space Shuttle | NOT RECOGNIZED"));
}

#[test]
fn test_e2e_single_trip() {
    let output = run_optimizer(&[
        "--entry",
        "McCowan",
        "--exit",
        "Hwy404",
        "--date",
        "2026-05-12",
        "--time",
        "08:00 AM",
    ]);

    assert!(output.contains("--- Single Trip Cost Analysis ---"));
    assert!(output.contains("Route: McCowan -> Hwy404"));
    assert!(output.contains("Base Toll: $4.52"));
    assert!(output.contains("Trip Charge: $1.00"));
    assert!(output.contains("Total Estimated Cost: $5.52"));
}

#[test]
fn test_e2e_single_trip_json() {
    let output = run_optimizer(&[
        "--entry",
        "McCowan",
        "--exit",
        "Hwy404",
        "--date",
        "2026-05-12",
        "--time",
        "08:00 AM",
        "--json",
    ]);

    let json: serde_json::Value =
        serde_json::from_str(&output).expect("Output should be valid JSON");
    assert_eq!(json["entry"], "McCowan");
    assert_eq!(json["exit"], "Hwy404");

    let base_toll = json["base_toll"].as_f64().unwrap();
    assert!((base_toll - 4.52).abs() < 0.01);
    assert_eq!(json["trip_charge"], 1.0);
    assert_eq!(json["total_estimated_cost"], base_toll + 1.0);
}

#[test]
fn test_e2e_camera_charges() {
    let output = run_optimizer(&["tests/csv/2025-08-28 - camera charges.csv"]);

    assert!(output.contains("Camera Charges by Transponder/Plate:"));
    assert!(output.contains("  - A12345K0: $15.90"));
    assert!(output.contains("  - A12345K1: $37.10"));

    // A12345K2 has $0.00, so it should NOT be in the camera charges list
    // We check for the specific list item format to avoid matching other sections
    assert!(!output.contains("  - A12345K2:"));

    assert!(output.contains("Tip: Leasing a transponder for $31.50 (plus applicable taxes) per year will save you money on the camera charges."));
}
