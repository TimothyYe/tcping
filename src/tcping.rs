use crate::stats::StatsCalculator;
use colored::Colorize;
use ctrlc;
use std::net::{SocketAddr, TcpStream, ToSocketAddrs};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::thread;
use std::time::{Duration, Instant};

const INTERVAL: Duration = Duration::from_millis(500);
const DEFAULT_TIMEOUT: Duration = Duration::from_secs(3);

pub fn run_tcping(host: &str, port: u16, num_pings: u32) -> Result<(), Box<dyn std::error::Error>> {
    let mut stats = StatsCalculator::new();
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
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
        // Check for SIGINT signal
        if !running.load(Ordering::SeqCst) {
            break;
        }

        match tcp_ping(&addr, DEFAULT_TIMEOUT) {
            Ok(duration) => {
                let latency = duration.as_secs_f64() * 1000.0;
                println!(
                    "Reply from {}({}) on port {} TCP_conn={} time={:.3} ms",
                    host,
                    ip,
                    port,
                    i,
                    latency
                );
                stats.add(latency);
            }
            Err(e) => {
                println!("Failed to connect (TCP_conn={}): {}", i, e);
                stats.add_loss();
            }
        }

        thread::sleep(INTERVAL);
    }

    let ping_stat = stats.get_result();
    println!("--- {} ping statistics ---", host);

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
        Ok(_) => Ok(start.elapsed()),
        Err(e) => match e.kind() {
            std::io::ErrorKind::TimedOut => Err(std::io::Error::new(
                std::io::ErrorKind::TimedOut,
                "Connection timed out",
            )),
            std::io::ErrorKind::ConnectionRefused => Err(std::io::Error::new(
                std::io::ErrorKind::ConnectionRefused,
                "Connection refused",
            )),
            _ => Err(e),
        },
    }
}
