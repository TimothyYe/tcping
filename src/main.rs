use clap::{command, Arg};
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
        .get_matches();

    let host = matches.get_one::<String>("host").unwrap();
    let port = matches.get_one::<String>("port").unwrap().to_owned();
    let port_num = port.parse::<u16>()?;
    let num = matches.get_one::<String>("n").unwrap().to_owned();
    let num_pings = num.parse::<u32>().unwrap().to_owned();

    tcping::run_tcping(host, port_num, num_pings)
}
