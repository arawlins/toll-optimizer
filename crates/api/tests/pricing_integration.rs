use reqwest::blocking::Client;
use serde_json::json;
use std::io::ErrorKind;
use std::net::TcpStream;
use std::process::{Child, Command, Stdio};
use std::time::{Duration, Instant};

struct KillOnDrop(Child);

impl Drop for KillOnDrop {
    fn drop(&mut self) {
        let _ = self.0.kill();
    }
}

fn setup_api() -> KillOnDrop {
    // Build the binary
    let status = Command::new("cargo")
        .arg("build")
        .status()
        .expect("Failed to build API");
    assert!(status.success());

    // Start the process
    let mut db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://admin:password@localhost:5433/toll_optimizer".to_string());
    if db_url.contains("@localhost/") || db_url.contains("@127.0.0.1/") {
        db_url = db_url.replace("@localhost/", "@localhost:5433/");
        db_url = db_url.replace("@127.0.0.1/", "@127.0.0.1:5433/");
    }

    let child = Command::new("../../target/debug/toll-optimizer-api")
        .env("DATABASE_URL", db_url)
        .env("PORT", "3006")
        .env("DISABLE_RATE_LIMIT", "true")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to start API");

    let guard = KillOnDrop(child);

    // Wait for the server to start (polling port 3006)
    let start = Instant::now();
    let timeout = Duration::from_secs(15);
    let mut success = false;

    while start.elapsed() < timeout {
        match TcpStream::connect("127.0.0.1:3006") {
            Ok(_) => {
                success = true;
                break;
            }
            Err(e) if e.kind() == ErrorKind::ConnectionRefused => {
                std::thread::sleep(Duration::from_millis(500));
            }
            Err(e) => {
                eprintln!("Unexpected error connecting to API: {:?}", e);
                break;
            }
        }
    }

    assert!(
        success,
        "API did not start listening on port 3006 within timeout"
    );

    guard
}

#[test]
fn test_pricing_weekday_normal() {
    let _guard = setup_api();
    let client = Client::new();

    let res = client
        .post("http://127.0.0.1:3006/api/pricing")
        .json(&json!({
            "date": "2026-04-13", // Monday
            "time": "08:15 AM"
        }))
        .send()
        .expect("Failed to send request");

    assert!(res.status().is_success());
    let data: serde_json::Value = res.json().unwrap();
    
    assert_eq!(data["day_type"], "Weekday");
    assert_eq!(data["current"]["timeslot"], "7:00 AM");
    assert_eq!(data["next"]["timeslot"], "9:30 AM");
    // Weekday 2026 rates (from constants)
    // 7am: EB 87.63, WB 89.35
    // 9:30am: EB 72.14, WB 69.98
    assert_eq!(data["current"]["average_eb"], 87.63);
    assert_eq!(data["next"]["average_eb"], 72.14);
}

#[test]
fn test_pricing_weekend_normal() {
    let _guard = setup_api();
    let client = Client::new();

    let res = client
        .post("http://127.0.0.1:3006/api/pricing")
        .json(&json!({
            "date": "2026-04-12", // Sunday
            "time": "02:00 PM"
        }))
        .send()
        .expect("Failed to send request");

    assert!(res.status().is_success());
    let data: serde_json::Value = res.json().unwrap();
    
    assert_eq!(data["day_type"], "Weekend");
    assert_eq!(data["current"]["timeslot"], "10:00 AM");
    assert_eq!(data["next"]["timeslot"], "7:00 PM");
}

#[test]
fn test_pricing_holiday() {
    let _guard = setup_api();
    let client = Client::new();

    let res = client
        .post("http://127.0.0.1:3006/api/pricing")
        .json(&json!({
            "date": "2026-01-01", // New Year's Day
            "time": "10:00 AM"
        }))
        .send()
        .expect("Failed to send request");

    assert!(res.status().is_success());
    let data: serde_json::Value = res.json().unwrap();
    
    assert_eq!(data["day_type"], "Holiday");
}

#[test]
fn test_pricing_wrap_sunday_to_monday() {
    let _guard = setup_api();
    let client = Client::new();

    // Sunday Night 11 PM
    let res = client
        .post("http://127.0.0.1:3006/api/pricing")
        .json(&json!({
            "date": "2026-04-12",
            "time": "11:00 PM"
        }))
        .send()
        .expect("Failed to send request");

    assert!(res.status().is_success());
    let data: serde_json::Value = res.json().unwrap();
    
    assert_eq!(data["day_type"], "Weekend");
    assert_eq!(data["current"]["timeslot"], "9:00 PM");
    // Should roll over to Monday Weekday
    assert_eq!(data["next"]["timeslot"], "5:00 AM");
    // Monday 5am Weekday WB is 69.10
    assert_eq!(data["next"]["average_wb"], 69.10);
}

#[test]
fn test_pricing_wrap_friday_to_saturday() {
    let _guard = setup_api();
    let client = Client::new();

    // Friday Night 11 PM
    let res = client
        .post("http://127.0.0.1:3006/api/pricing")
        .json(&json!({
            "date": "2026-04-10",
            "time": "11:00 PM"
        }))
        .send()
        .expect("Failed to send request");

    assert!(res.status().is_success());
    let data: serde_json::Value = res.json().unwrap();
    
    assert_eq!(data["day_type"], "Weekday");
    assert_eq!(data["current"]["timeslot"], "9:00 PM");
    // Should roll over to Saturday Weekend
    assert_eq!(data["next"]["timeslot"], "8:30 AM");
    // Saturday 8:30am Weekend WB is 58.15
    assert_eq!(data["next"]["average_wb"], 58.15);
}

#[test]
fn test_pricing_invalid_date() {
    let _guard = setup_api();
    let client = Client::new();

    let res = client
        .post("http://127.0.0.1:3006/api/pricing")
        .json(&json!({
            "date": "invalid-date",
            "time": "10:00 AM"
        }))
        .send()
        .expect("Failed to send request");

    assert_eq!(res.status(), 400);
}

#[test]
fn test_pricing_invalid_time() {
    let _guard = setup_api();
    let client = Client::new();

    let res = client
        .post("http://127.0.0.1:3006/api/pricing")
        .json(&json!({
            "date": "2026-04-13",
            "time": "99:99"
        }))
        .send()
        .expect("Failed to send request");

    assert_eq!(res.status(), 400);
}

#[test]
fn test_pricing_exact_timeslot_start() {
    let _guard = setup_api();
    let client = Client::new();

    // Exactly at 7:00 AM Weekday
    let res = client
        .post("http://127.0.0.1:3006/api/pricing")
        .json(&json!({
            "date": "2026-04-13",
            "time": "07:00 AM"
        }))
        .send()
        .expect("Failed to send request");

    assert!(res.status().is_success());
    let data: serde_json::Value = res.json().unwrap();
    
    assert_eq!(data["current"]["timeslot"], "7:00 AM");
}

#[test]
fn test_pricing_one_minute_before_timeslot() {
    let _guard = setup_api();
    let client = Client::new();

    // 6:59 AM should still be in the 5:00 AM timeslot
    let res = client
        .post("http://127.0.0.1:3006/api/pricing")
        .json(&json!({
            "date": "2026-04-13",
            "time": "06:59 AM"
        }))
        .send()
        .expect("Failed to send request");

    assert!(res.status().is_success());
    let data: serde_json::Value = res.json().unwrap();
    
    assert_eq!(data["current"]["timeslot"], "5:00 AM");
    assert_eq!(data["next"]["timeslot"], "7:00 AM");
}
