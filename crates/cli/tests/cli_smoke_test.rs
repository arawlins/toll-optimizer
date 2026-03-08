use std::env;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;

#[test]
fn test_application_runs_on_sample_csv() {
    // 1. Build the binary first to ensure it's up to date
    let status = Command::new("cargo")
        .arg("build")
        .status()
        .expect("Failed to build project");
    assert!(status.success(), "Cargo build failed");

    // 2. Locate the binary
    // Typically in target/debug/toll-optimizer
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let bin_path = Path::new(&manifest_dir).join("../../target/debug/toll-optimizer-cli");

    // 3. Create a temporary directory for our test execution
    let temp_dir = env::temp_dir().join("toll-optimizer-test-run");
    if temp_dir.exists() {
        fs::remove_dir_all(&temp_dir).expect("Failed to clean up old temp dir");
    }
    fs::create_dir_all(&temp_dir).expect("Failed to create temp dir");

    // 4. Create the 'csv' subdirectory structure expected by the app
    let csv_dir = temp_dir.join("csv");
    fs::create_dir(&csv_dir).expect("Failed to create csv dir");

    // 5. Create the specific hardcoded CSV file required by main.rs
    let csv_file_path = csv_dir.join("2025-08-28 - 573522284 Statement.csv");
    let mut file = fs::File::create(&csv_file_path).expect("Failed to create CSV file");

    let csv_content = r###"NOTE: Test Data

Name: Test User

"Transponder/Plate Number","Vehicle Class","Date of Trip","Entry Time","Entry Point","Exit Point","Distance (km)","Toll Charge ($)","Trip Toll Charge ($)","Camera Charge ($)""Transponder/Plate Number","Vehicle Class","Date of Trip","Entry Time","Entry Point","Exit Point","Distance (km)","Toll Charge ($)","Trip Toll Charge ($)","Camera Charge ($)"
"TEST_PLATE_001","Light vehicle","28 Aug 25","10:00 AM","QEW","Trafalgar","10.0","5.00","0.00","0.00"
""###;
    file.write_all(csv_content.as_bytes())
        .expect("Failed to write CSV content");

    // 6. Execute the binary from the temp directory
    let output = Command::new(&bin_path)
        .arg(&csv_file_path)
        .output()
        .expect("Failed to execute binary");

    // 7. Verify output
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    println!("STDOUT:\n{}", stdout);
    println!("STDERR:\n{}", stderr);

    assert!(
        output.status.success(),
        "Application exited with failure: {}",
        stderr
    );

    // Assert that our test plate was processed and output
    assert!(
        stdout.contains("TEST_PLATE_001"),
        "Output did not contain the test transponder plate"
    );
    assert!(
        stdout.contains("QEW"),
        "Output did not contain entry point QEW"
    );
    assert!(
        stdout.contains("Trafalgar"),
        "Output did not contain exit point Trafalgar"
    );
}
