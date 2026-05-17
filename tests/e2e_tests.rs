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

    assert!(output.status.success(), "Command failed with status: {:?}\nStderr: {}", output.status, String::from_utf8_lossy(&output.stderr));
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
    
    let json: serde_json::Value = serde_json::from_str(&output).expect("Output should be valid JSON");
    assert_eq!(json["summary"]["total_processed"], 6);
    assert_eq!(json["summary"]["total_skipped"], 3);
    assert!(json["summary"]["unknown_points"].as_array().unwrap().contains(&serde_json::json!("Goober")));
    assert!(json["summary"]["unknown_points"].as_array().unwrap().contains(&serde_json::json!("Doober")));
    assert!(json["summary"]["unknown_vehicle_classes"].as_array().unwrap().contains(&serde_json::json!("Space Shuttle")));
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
