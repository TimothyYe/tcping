use crate::stats::StatsCalculator;
use colored::Colorize;
use std::net::{SocketAddr, TcpStream, ToSocketAddrs};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::thread;
use std::time::{Duration, Instant};

pub fn run_tcping(
    host: &str,
    port: u16,
    num_pings: u32,
    timeout: Duration,
    interval: Duration,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut stats = StatsCalculator::new();
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::Relaxed);
    })?;

    println!(
        "TCPing {} on port {}.",
        host.yellow(),
        port.to_string().yellow(),
    );

    let addr = (host.to_string(), port)
        .to_socket_addrs()?
        .next()
        .ok_or("Failed to resolve address")?;
    let ip = addr.ip();

    for i in 1..=num_pings {
        if !running.load(Ordering::Relaxed) {
            break;
        }

        match tcp_ping(&addr, timeout) {
            Ok(duration) => {
                let latency = duration.as_secs_f64() * 1000.0;
                println!(
                    "Reply from {host}({ip}) on port {port} TCP_conn={i} time={latency:.3} ms"
                );
                stats.add(latency);
            }
            Err(e) => {
                println!("Failed to connect (TCP_conn={i}): {e}");
                stats.add_loss();
            }
        }

        if i < num_pings {
            thread::sleep(interval);
        }
    }

    let ping_stat = stats.get_result();
    println!("--- {host} ping statistics ---");

    println!(
        "{} packets transmitted, {} packets received, {:.1}% packet loss",
        ping_stat.total_packages,
        ping_stat.received_packages,
        ((ping_stat.total_packages - ping_stat.received_packages) as f64
            / ping_stat.total_packages as f64)
            * 100.0
    );

    println!(
        "round-trip min/avg/max/stddev = {:.3}/{:.3}/{:.3}/{:.3} ms",
        ping_stat.min_latency,
        ping_stat.avg_latency,
        ping_stat.max_latency,
        ping_stat.std_dev_latency
    );

    Ok(())
}

fn tcp_ping(addr: &SocketAddr, timeout: Duration) -> Result<Duration, std::io::Error> {
    let start = Instant::now();
    match TcpStream::connect_timeout(addr, timeout) {
        Ok(stream) => {
            // Close the stream immediately to avoid resource leaks
            drop(stream);
            Ok(start.elapsed())
        }
        Err(e) => match e.kind() {
            std::io::ErrorKind::TimedOut => Err(std::io::Error::new(
                std::io::ErrorKind::TimedOut,
                "Connection timed out",
            )),
            std::io::ErrorKind::ConnectionRefused => Err(std::io::Error::new(
                std::io::ErrorKind::ConnectionRefused,
                "Connection refused",
            )),
            std::io::ErrorKind::ConnectionAborted => Err(std::io::Error::new(
                std::io::ErrorKind::ConnectionAborted,
                "Connection was aborted",
            )),
            std::io::ErrorKind::ConnectionReset => Err(std::io::Error::new(
                std::io::ErrorKind::ConnectionReset,
                "Connection was reset",
            )),
            std::io::ErrorKind::AddrNotAvailable => Err(std::io::Error::new(
                std::io::ErrorKind::AddrNotAvailable,
                "Address not available",
            )),
            _ => Err(std::io::Error::new(
                e.kind(),
                format!("Connection failed: {e}"),
            )),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::{IpAddr, Ipv4Addr};

    #[test]
    fn test_tcp_ping_timeout() {
        // Use a non-routable address to test timeout
        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(10, 254, 254, 254)), 9999);
        let timeout = Duration::from_millis(100);

        let start = Instant::now();
        let result = tcp_ping(&addr, timeout);
        let elapsed = start.elapsed();

        assert!(result.is_err());
        // Should timeout within reasonable bounds (100ms + some tolerance)
        assert!(elapsed >= timeout);
        assert!(elapsed <= timeout + Duration::from_millis(50));
    }

    #[test]
    fn test_tcp_ping_connection_refused() {
        // Try to connect to localhost on a likely unused port
        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 9999);
        let timeout = Duration::from_secs(1);

        let result = tcp_ping(&addr, timeout);
        assert!(result.is_err());

        let err = result.unwrap_err();
        assert_eq!(err.kind(), std::io::ErrorKind::ConnectionRefused);
    }

    #[test]
    fn test_tcp_ping_successful_connection() {
        // This test requires an actual reachable service
        // Using Google's DNS server on port 53 as it's likely to be available
        let addr = "8.8.8.8:53".parse::<SocketAddr>().unwrap();
        let timeout = Duration::from_secs(5);

        let result = tcp_ping(&addr, timeout);
        match result {
            Ok(duration) => {
                // Connection should be reasonably fast
                assert!(duration < Duration::from_secs(2));
            }
            Err(_) => {
                // This might fail in some network environments, so we'll just log it
                // In a real test suite, you might want to skip this test if network is unavailable
                println!("Network test skipped - no connection to 8.8.8.8:53");
            }
        }
    }

    #[test]
    fn test_tcp_ping_timing_accuracy() {
        // Test with a local service (assuming ssh is running on port 22)
        // This test verifies that timing measurement is working
        let addr = "127.0.0.1:22".parse::<SocketAddr>().unwrap();
        let timeout = Duration::from_secs(1);

        let start = Instant::now();
        let result = tcp_ping(&addr, timeout);
        let total_elapsed = start.elapsed();

        match result {
            Ok(measured_duration) => {
                // Measured duration should be less than total elapsed time
                assert!(measured_duration <= total_elapsed);
                // Should be a reasonable connection time for localhost
                assert!(measured_duration < Duration::from_millis(100));
            }
            Err(_) => {
                // SSH not running or connection refused is fine for this test
                println!("Local SSH test skipped - service not available");
            }
        }
    }

    #[test]
    fn test_error_message_formatting() {
        // Test that error messages are properly formatted
        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 9999);
        let timeout = Duration::from_millis(10);

        let result = tcp_ping(&addr, timeout);
        assert!(result.is_err());

        let err = result.unwrap_err();
        let err_msg = err.to_string();

        // Should contain meaningful error description
        assert!(
            err_msg == "Connection refused"
                || err_msg == "Connection timed out"
                || err_msg.contains("Connection failed")
        );
    }
}
