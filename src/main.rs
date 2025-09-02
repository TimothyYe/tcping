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
    let port_num = matches.get_one::<String>("port").unwrap().parse::<u16>()?;
    let num_pings = matches.get_one::<String>("n").unwrap().parse::<u32>()?;

    let timeout_secs = matches.get_one::<String>("t").unwrap().parse::<u64>()?;
    let timeout = Duration::from_secs(timeout_secs);

    let interval_ms = matches.get_one::<String>("i").unwrap().parse::<u64>()?;
    let interval = Duration::from_millis(interval_ms);

    tcping::run_tcping(host, port_num, num_pings, timeout, interval)
}
