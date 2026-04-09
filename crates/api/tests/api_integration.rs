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

fn setup_api() -> (KillOnDrop, String) {
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
        .env("PORT", "3005")
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
        match TcpStream::connect("127.0.0.1:3005") {
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
        .post("http://127.0.0.1:3005/auth/register")
        .json(&json!({
            "email": email,
            "password": "password123"
        }))
        .send()
        .expect("Failed to register");

    let login_res = client
        .post("http://127.0.0.1:3005/auth/login")
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
    let mut db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://admin:password@localhost:5433/toll_optimizer".to_string());
    if db_url.contains("@localhost/") || db_url.contains("@127.0.0.1/") {
        db_url = db_url.replace("@localhost/", "@localhost:5433/");
        db_url = db_url.replace("@127.0.0.1/", "@127.0.0.1:5433/");
    }

    let child = Command::new("../../target/debug/toll-optimizer-api")
        .env("DATABASE_URL", db_url)
        .env("PORT", "3005")
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
        match TcpStream::connect("127.0.0.1:3005") {
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
    let mut db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://admin:password@localhost:5433/toll_optimizer".to_string());
    if db_url.contains("@localhost/") || db_url.contains("@127.0.0.1/") {
        db_url = db_url.replace("@localhost/", "@localhost:5433/");
        db_url = db_url.replace("@127.0.0.1/", "@127.0.0.1:5433/");
    }

    let child = Command::new("../../target/debug/toll-optimizer-api")
        .env("DATABASE_URL", db_url)
        .env("PORT", "3005")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to start API");

    let _guard = KillOnDrop(child);

    // 3. Wait for the server to start
    let start = Instant::now();
    let timeout = Duration::from_secs(30);
    let mut ready = false;
    while start.elapsed() < timeout {
        if TcpStream::connect("127.0.0.1:3005").is_ok() {
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
        .post("http://127.0.0.1:3005/auth/register")
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
        .post("http://127.0.0.1:3005/auth/login")
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

#[test]
fn test_duplicate_email_registration() {
    let (_guard, _) = setup_api();
    let client = Client::new();

    let email = format!("test_{}@example.com", uuid::Uuid::new_v4());
    
    // First registration should succeed
    let register_res1 = client
        .post("http://127.0.0.1:3005/auth/register")
        .json(&json!({
            "email": email,
            "password": "password123"
        }))
        .send()
        .expect("Failed to send register request");
        
    assert!(register_res1.status().is_success());

    // Second registration with same email should fail (400 Bad Request usually, or some client error)
    let register_res2 = client
        .post("http://127.0.0.1:3005/auth/register")
        .json(&json!({
            "email": email,
            "password": "password123"
        }))
        .send()
        .expect("Failed to send second register request");

    assert!(register_res2.status().is_client_error());
}

#[test]
fn test_get_summaries_isolation() {
    let (_guard, token_a) = setup_api();
    let client = Client::new();

    // Register User B and get token
    let email_b = format!("test_{}@example.com", uuid::Uuid::new_v4());
    client
        .post("http://127.0.0.1:3005/auth/register")
        .json(&json!({
            "email": email_b,
            "password": "password123"
        }))
        .send()
        .expect("Failed to register User B");

    let login_res_b = client
        .post("http://127.0.0.1:3005/auth/login")
        .json(&json!({
            "email": email_b,
            "password": "password123"
        }))
        .send()
        .expect("Failed to login User B");

    let token_b = login_res_b.json::<serde_json::Value>().unwrap()["token"].as_str().unwrap().to_string();

    // User A uploads a file containing TAG123
    let form_a = reqwest::blocking::multipart::Form::new().part(
        "file",
        reqwest::blocking::multipart::Part::text("\"Transponder/Plate Number\",\"Vehicle Class\",\"Date of Trip\",\"Entry Time\",\"Entry Point\",\"Exit Point\",\"Distance (km)\",\"Toll Charge ($)\",\"Trip Toll Charge ($)\",\"Camera Charge ($)\"\n\"TAG123\",\"Light vehicle\",\"28 Aug 25\",\"10:00 AM\",\"QEW\",\"Trafalgar\",\"10.0\",\"0.00\",\"0.00\",\"0.00\"").file_name("file_a.csv"),
    );

    client.post("http://127.0.0.1:3005/api/analyze").bearer_auth(&token_a).multipart(form_a).send().expect("User A upload failed");

    // User B uploads a file containing TAG888
    let form_b = reqwest::blocking::multipart::Form::new().part(
        "file",
        reqwest::blocking::multipart::Part::text("\"Transponder/Plate Number\",\"Vehicle Class\",\"Date of Trip\",\"Entry Time\",\"Entry Point\",\"Exit Point\",\"Distance (km)\",\"Toll Charge ($)\",\"Trip Toll Charge ($)\",\"Camera Charge ($)\"\n\"TAG888\",\"Light vehicle\",\"28 Aug 25\",\"10:00 AM\",\"QEW\",\"Trafalgar\",\"10.0\",\"0.00\",\"0.00\",\"0.00\"").file_name("file_b.csv"),
    );

    client.post("http://127.0.0.1:3005/api/analyze").bearer_auth(&token_b).multipart(form_b).send().expect("User B upload failed");

    // Check User A history
    let history_a: serde_json::Value = client.get("http://127.0.0.1:3005/api/history").bearer_auth(&token_a).send().unwrap().json().unwrap();
    let history_a_str = history_a.to_string();
    assert!(history_a_str.contains("file_a.csv"));
    assert!(!history_a_str.contains("file_b.csv"));

    // Check User B history
    let history_b: serde_json::Value = client.get("http://127.0.0.1:3005/api/history").bearer_auth(&token_b).send().unwrap().json().unwrap();
    let history_b_str = history_b.to_string();
    assert!(!history_b_str.contains("file_a.csv"));
    assert!(history_b_str.contains("file_b.csv"));
}

#[test]
fn test_decimal_boundary() {
    let (_guard, token) = setup_api();
    let client = Client::new();

    // A decimal value near 1 million for DECIMAL(15,2) limit testing.
    // 9,999,999.99
    // A single trip's reported cost can't be set to a million since calculate_cost depends on logic,
    // but the `toll_charge` parsed from CSV shouldn't crash if it's very large!
    
    // Simulate an enormous reported value
    let form = reqwest::blocking::multipart::Form::new().part(
        "file",
        reqwest::blocking::multipart::Part::text("\"Transponder/Plate Number\",\"Vehicle Class\",\"Date of Trip\",\"Entry Time\",\"Entry Point\",\"Exit Point\",\"Distance (km)\",\"Toll Charge ($)\",\"Trip Toll Charge ($)\",\"Camera Charge ($)\"\n\"RICH_DUDE\",\"Light vehicle\",\"28 Aug 25\",\"10:00 AM\",\"QEW\",\"Trafalgar\",\"10.0\",\"9999999.99\",\"0.00\",\"0.00\"").file_name("large_values.csv"),
    );

    let res = client
        .post("http://127.0.0.1:3005/api/analyze")
        .bearer_auth(&token)
        .multipart(form)
        .send()
        .expect("Failed to send request");

    // The backend should either process it (storing safely in DECIMAL(15,2)) or validation block it.
    // Given the task was to ensure DECIMAL(15,2) supports large numbers without a database query failure (Status 500),
    // It should be 200 OK.
    
    assert!(res.status().is_success(), "Expected 200 OK for large value, got {:?}", res.status());
}
