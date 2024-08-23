extern crate colored;

use clap::{command, Arg};
use colored::Colorize;
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
    let matches = command!()
        .arg(
            Arg::new("host")
                .required(true)
                .index(1)
                .help("Host to ping"),
        )
        .arg(
            Arg::new("port")
                .required(true)
                .index(2)
                .help("Port to ping"),
        )
        .get_matches();

    let host = matches.get_one::<String>("host").unwrap().clone();
    let port = matches.get_one::<String>("port").unwrap().to_owned();
    let port_num = port.parse::<u16>()?;
    let timeout = Duration::from_secs(5);
    let num_pings = 7;

    println!(
        "TCPinging {} on port {}.",
        host.yellow(),
        port.to_string().yellow(),
    );

    let addr = (host.clone(), port_num)
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
        thread::sleep(Duration::from_millis(500));
    }

    Ok(())
}
