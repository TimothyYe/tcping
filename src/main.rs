use std::io::Error;
use std::net::{SocketAddr, TcpStream, ToSocketAddrs};
use std::thread;
use std::time::{Duration, Instant};

fn tcp_ping(addr: &SocketAddr, timeout: Duration) -> Result<Duration, Error> {
    let start = Instant::now();
    match TcpStream::connect_timeout(addr, timeout) {
        Ok(_) => Ok(start.elapsed()),
        Err(e) => Err(e),
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let host = "xiaozhou.net";
    let port = 443;
    let timeout = Duration::from_secs(5);
    let num_pings = 7;

    println!("TCPinging {} on port {}.", host, port);

    let addr = (host, port)
        .to_socket_addrs()?
        .next()
        .ok_or("Failed to resolve address")?;
    let ip = addr.ip();

    for i in 1..=num_pings {
        match tcp_ping(&addr, timeout) {
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

        // Add a small delay between pings
        thread::sleep(Duration::from_millis(300));
    }

    Ok(())
}
