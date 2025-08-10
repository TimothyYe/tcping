use clap::{command, Arg};
use std::time::Duration;
mod stats;
mod tcping;

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
        .arg(
            Arg::new("n")
                .short('n')
                .long("num")
                .default_value("10")
                .help("Number of pings to send"),
        )
        .arg(
            Arg::new("t")
                .short('t')
                .long("timeout")
                .default_value("3")
                .help("Timeout in seconds for each connection attempt"),
        )
        .arg(
            Arg::new("i")
                .short('i')
                .long("interval")
                .default_value("500")
                .help("Time interval between pings in milliseconds"),
        )
        .get_matches();

    let host = matches.get_one::<String>("host").unwrap();
    let port = matches.get_one::<String>("port").unwrap().to_owned();
    let port_num = port.parse::<u16>()?;
    let num = matches.get_one::<String>("n").unwrap().to_owned();
    let num_pings = num.parse::<u32>().unwrap().to_owned();

    let timeout_str = matches.get_one::<String>("t").unwrap().to_owned();
    let timeout_secs = timeout_str.parse::<u64>().unwrap_or(3);
    let timeout = Duration::from_secs(timeout_secs);

    let interval_str = matches.get_one::<String>("i").unwrap().to_owned();
    let interval_ms = interval_str.parse::<u64>().unwrap_or(500);
    let interval = Duration::from_millis(interval_ms);

    tcping::run_tcping(host, port_num, num_pings, timeout, interval)
}
