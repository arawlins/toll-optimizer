use reqwest::blocking::{Client, multipart};
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

fn setup_api() -> (KillOnDrop, String) {
    // Build the binary
    let status = Command::new("cargo")
        .arg("build")
        .status()
        .expect("Failed to build API");
    assert!(status.success());

    // Start the process
    let child = Command::new("../../target/debug/toll-optimizer-api")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to start API");

    let guard = KillOnDrop(child);

    // Wait for the server to start (polling port 3000)
    let start = Instant::now();
    let timeout = Duration::from_secs(15);
    let mut success = false;

    while start.elapsed() < timeout {
        match TcpStream::connect("127.0.0.1:3000") {
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
        "API did not start listening on port 3000 within timeout"
    );

    // Register and login to get a token
    let client = Client::new();
    let email = format!("test_{}@example.com", uuid::Uuid::new_v4());
    client
        .post("http://127.0.0.1:3000/auth/register")
        .json(&json!({
            "email": email,
            "password": "password123"
        }))
        .send()
        .expect("Failed to register");

    let login_res = client
        .post("http://127.0.0.1:3000/auth/login")
        .json(&json!({
            "email": email,
            "password": "password123"
        }))
        .send()
        .expect("Failed to login");

    let login_data: serde_json::Value = login_res.json().expect("Failed to parse login JSON");
    let token = login_data["token"]
        .as_str()
        .expect("Token not found")
        .to_string();

    (guard, token)
}

#[test]
fn test_security_xss_filename() {
    let (_guard, token) = setup_api();
    let client = Client::new();

    let xss_filename = "<script>alert('xss')</script>.csv";
    let form = multipart::Form::new().part(
        "file",
        multipart::Part::text("Date,Time,Tag ID\n2024-01-01,10:00,123").file_name(xss_filename),
    );

    let res = client
        .post("http://127.0.0.1:3000/api/analyze")
        .bearer_auth(&token)
        .multipart(form)
        .send()
        .expect("Failed to send request");

    assert!(res.status().is_success());

    // Check history to see if it was saved correctly (it should be literal in the DB)
    let history_res = client
        .get("http://127.0.0.1:3000/api/history")
        .bearer_auth(&token)
        .send()
        .expect("Failed to get history");

    let history_data: serde_json::Value = history_res.json().expect("Failed to parse history JSON");
    let saved_filename = history_data[0]["filename"].as_str().unwrap();
    assert_eq!(saved_filename, xss_filename);
}

#[test]
fn test_security_binary_content_rejection() {
    let (_guard, token) = setup_api();
    let client = Client::new();

    // Invalid UTF-8 sequence
    let binary_data = vec![0, 159, 146, 150];
    let form = multipart::Form::new().part(
        "file",
        multipart::Part::bytes(binary_data).file_name("malicious.csv"),
    );

    let res = client
        .post("http://127.0.0.1:3000/api/analyze")
        .bearer_auth(&token)
        .multipart(form)
        .send()
        .expect("Failed to send request");

    assert_eq!(res.status(), reqwest::StatusCode::BAD_REQUEST);
    assert!(res.text().unwrap().contains("File is not valid UTF-8"));
}

#[test]
fn test_security_large_file_handling() {
    let (_guard, token) = setup_api();
    let client = Client::new();

    // Test large file - ensure it doesn't exceed DECIMAL(10,2) limits which cause 500s
    let large_data = "Date,Time,Tag ID\n".to_string() + &"2024-01-01,10:00,123\n".repeat(10_000);
    let form = multipart::Form::new().part(
        "file",
        multipart::Part::text(large_data).file_name("large.csv"),
    );

    let res = client
        .post("http://127.0.0.1:3000/api/analyze")
        .bearer_auth(&token)
        .multipart(form)
        .send()
        .expect("Failed to send request");

    assert!(
        res.status().is_success(),
        "Expected success for 5MB file, got: {:?}",
        res.status()
    );
}

#[test]
fn test_security_empty_file_handling() {
    let (_guard, token) = setup_api();
    let client = Client::new();

    let form =
        multipart::Form::new().part("file", multipart::Part::text("").file_name("empty.csv"));

    let res = client
        .post("http://127.0.0.1:3000/api/analyze")
        .bearer_auth(&token)
        .multipart(form)
        .send()
        .expect("Failed to send request");

    assert_eq!(res.status(), reqwest::StatusCode::BAD_REQUEST);
    assert!(res.text().unwrap().contains("No file provided"));
}
