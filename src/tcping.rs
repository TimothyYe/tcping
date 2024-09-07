use colored::Colorize;
use std::net::{SocketAddr, TcpStream, ToSocketAddrs};
use std::thread;
use std::time::{Duration, Instant};

const INTERVAL: Duration = Duration::from_millis(500);
const DEFAULT_TIMEOUT: Duration = Duration::from_secs(5);

pub fn run_tcping(host: &str, port: u16, num_pings: u32) -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "TCPinging {} on port {}.",
        host.yellow(),
        port.to_string().yellow(),
    );

    let addr = (host.to_string(), port)
        .to_socket_addrs()?
        .next()
        .ok_or("Failed to resolve address")?;
    let ip = addr.ip();

    for i in 1..=num_pings {
        match tcp_ping(&addr, DEFAULT_TIMEOUT) {
            Ok(duration) => {
                println!(
                    "Reply from {}({}) on port {} TCP_conn={} time={:.3} ms",
                    host,
                    ip,
                    port,
                    i,
                    duration.as_secs_f64() * 1000.0
                );
            }
            Err(e) => println!("Failed to connect (TCP_conn={}): {}", i, e),
        }

        thread::sleep(INTERVAL);
    }

    Ok(())
}

fn tcp_ping(addr: &SocketAddr, timeout: Duration) -> Result<Duration, std::io::Error> {
    let start = Instant::now();
    match TcpStream::connect_timeout(addr, timeout) {
        Ok(_) => Ok(start.elapsed()),
        Err(e) => Err(e),
    }
}
