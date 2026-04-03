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

#[test]
fn test_api_starts_and_listens() {
    // Note: This test requires DATABASE_URL to be set in environment or .env
    // If it's not set, it might fail to start.
    // In a CI environment, we would typically provide a mock or a test container.

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

    let _guard = KillOnDrop(child);

    // Wait for the server to start (polling port 3000)
    let start = Instant::now();
    let timeout = Duration::from_secs(10);
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
}

#[test]
fn test_registration_and_login() {
    // 1. Build the API binary
    let status = Command::new("cargo")
        .arg("build")
        .status()
        .expect("Failed to build API");
    assert!(status.success());

    // 2. Start the API
    let child = Command::new("../../target/debug/toll-optimizer-api")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to start API");

    let _guard = KillOnDrop(child);

    // 3. Wait for the server to start
    let start = Instant::now();
    let timeout = Duration::from_secs(10);
    let mut ready = false;
    while start.elapsed() < timeout {
        if TcpStream::connect("127.0.0.1:3000").is_ok() {
            ready = true;
            break;
        }
        std::thread::sleep(Duration::from_millis(500));
    }
    assert!(ready, "API did not start in time");

    // 4. Test /auth/register
    let client = reqwest::blocking::Client::new();
    let unique_email = format!("test_{}@example.com", uuid::Uuid::new_v4());
    let register_res = client
        .post("http://127.0.0.1:3000/auth/register")
        .json(&serde_json::json!({
            "email": unique_email,
            "password": "password123"
        }))
        .send()
        .expect("Failed to send register request");

    assert!(
        register_res.status().is_success(),
        "Registration failed: {:?}",
        register_res.text()
    );

    // 5. Test /auth/login
    let login_res = client
        .post("http://127.0.0.1:3000/auth/login")
        .json(&serde_json::json!({
            "email": unique_email,
            "password": "password123"
        }))
        .send()
        .expect("Failed to send login request");

    assert!(
        login_res.status().is_success(),
        "Login failed: {:?}",
        login_res.text()
    );

    let login_data: serde_json::Value = login_res.json().expect("Failed to parse login JSON");
    assert!(
        login_data["token"].is_string(),
        "Token not found in login response"
    );
}
