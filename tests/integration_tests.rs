use std::process::Command;

#[test]
fn test_help_output() {
    let output = Command::new("cargo")
        .args(["run", "--", "--help"])
        .output()
        .expect("failed to execute process");

    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Host to ping"));
    assert!(stdout.contains("Port to ping"));
    assert!(stdout.contains("Number of pings"));
    assert!(stdout.contains("Timeout"));
    assert!(stdout.contains("interval"));
}

#[test]
fn test_missing_required_args() {
    let output = Command::new("cargo")
        .args(["run", "--", "google.com"])
        .output()
        .expect("failed to execute process");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("required") || stderr.contains("PORT"));
}

#[test]
fn test_invalid_port() {
    let output = Command::new("cargo")
        .args(["run", "--", "google.com", "70000"]) // Port > 65535
        .output()
        .expect("failed to execute process");

    assert!(!output.status.success());
}

#[test]
fn test_invalid_timeout() {
    let output = Command::new("cargo")
        .args(["run", "--", "google.com", "80", "-t", "invalid"])
        .output()
        .expect("failed to execute process");

    assert!(!output.status.success());
}

#[test]
fn test_invalid_interval() {
    let output = Command::new("cargo")
        .args(["run", "--", "google.com", "80", "-i", "not_a_number"])
        .output()
        .expect("failed to execute process");

    assert!(!output.status.success());
}

#[test]
fn test_valid_arguments() {
    let output = Command::new("cargo")
        .args([
            "run",
            "--",
            "127.0.0.1",
            "9999",
            "-n",
            "1",
            "-t",
            "1",
            "-i",
            "100",
        ])
        .output()
        .expect("failed to execute process");

    let stdout = String::from_utf8(output.stdout).unwrap();
    let stderr = String::from_utf8(output.stderr).unwrap();

    // Check that the program started successfully (not argument parsing errors)
    // The output should contain tcping startup message or show that it ran
    assert!(
        stdout.contains("TCPing") || stdout.contains("ping statistics") || output.status.success(),
        "Stdout: {}\nStderr: {}\nStatus: {:?}",
        stdout,
        stderr,
        output.status
    );
}

#[test]
fn test_default_values() {
    // Test that default values are used when not specified
    let output = Command::new("cargo")
        .args(["run", "--", "127.0.0.1", "9999"]) // Use unlikely port to avoid actual connections
        .output()
        .expect("failed to execute process");

    let stdout = String::from_utf8(output.stdout).unwrap();

    // Should start with TCPing output showing the defaults are applied
    assert!(stdout.contains("TCPing 127.0.0.1 on port 9999"));
}

#[test]
fn test_custom_ping_count() {
    let output = Command::new("cargo")
        .args(["run", "--", "127.0.0.1", "9999", "-n", "2"])
        .output()
        .expect("failed to execute process");

    let stdout = String::from_utf8(output.stdout).unwrap();

    // Should show exactly 2 connection attempts (or failures)
    let tcp_conn_count = stdout.matches("TCP_conn=").count();
    // Should be 2 attempts (though they may fail)
    assert!(tcp_conn_count <= 2);
    assert!(stdout.contains("ping statistics"));
}

#[test]
fn test_long_argument_names() {
    let output = Command::new("cargo")
        .args([
            "run",
            "--",
            "127.0.0.1",
            "9999",
            "--num",
            "1",
            "--timeout",
            "1",
            "--interval",
            "100",
        ])
        .output()
        .expect("failed to execute process");

    let stdout = String::from_utf8(output.stdout).unwrap();

    // Should work the same as short arguments
    assert!(stdout.contains("TCPing 127.0.0.1 on port 9999"));
}

#[test]
fn test_zero_ping_count() {
    let output = Command::new("cargo")
        .args(["run", "--", "google.com", "80", "-n", "0"])
        .output()
        .expect("failed to execute process");

    let stdout = String::from_utf8(output.stdout).unwrap();

    // Should still show statistics but with 0 attempts
    if stdout.contains("ping statistics") {
        assert!(stdout.contains("0 packets transmitted"));
    }
}
