use std::process::{Command, Stdio, Child};
use std::net::TcpStream;
use std::time::{Duration, Instant};
use std::io::ErrorKind;

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

    assert!(success, "API did not start listening on port 3000 within timeout");
}
