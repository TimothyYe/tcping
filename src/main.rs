use clap::{command, Arg, ArgMatches, Command};
use std::time::Duration;
mod stats;
mod tcping;

fn build_cli() -> Command {
    command!()
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
                .value_parser(clap::value_parser!(u16))
                .help("Port to ping"),
        )
        .arg(
            Arg::new("n")
                .short('n')
                .long("num")
                .default_value("10")
                .value_parser(clap::value_parser!(u32))
                .help("Number of pings to send"),
        )
        .arg(
            Arg::new("t")
                .short('t')
                .long("timeout")
                .default_value("3")
                .value_parser(clap::value_parser!(u64))
                .help("Timeout in seconds for each connection attempt"),
        )
        .arg(
            Arg::new("i")
                .short('i')
                .long("interval")
                .default_value("500")
                .value_parser(clap::value_parser!(u64))
                .help("Time interval between pings in milliseconds"),
        )
}

fn run_from_matches(matches: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let host = matches.get_one::<String>("host").unwrap();
    let port_num = *matches.get_one::<u16>("port").unwrap();
    let num_pings = *matches.get_one::<u32>("n").unwrap();

    let timeout_secs = *matches.get_one::<u64>("t").unwrap();
    if timeout_secs == 0 {
        return Err("Timeout must be greater than 0 seconds".into());
    }
    let timeout = Duration::from_secs(timeout_secs);

    let interval_ms = *matches.get_one::<u64>("i").unwrap();
    let interval = Duration::from_millis(interval_ms);

    tcping::run_tcping(host, port_num, num_pings, timeout, interval)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = build_cli().get_matches();
    run_from_matches(&matches)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_port_parsing() {
        let matches = build_cli()
            .try_get_matches_from(vec!["tcping", "google.com", "443"])
            .unwrap();

        let port = matches.get_one::<u16>("port").unwrap();
        assert_eq!(*port, 443);
    }

    #[test]
    fn test_invalid_port_parsing() {
        let result = build_cli().try_get_matches_from(vec!["tcping", "google.com", "70000"]);

        assert!(result.is_err());
    }

    #[test]
    fn test_default_values() {
        let matches = build_cli()
            .try_get_matches_from(vec!["tcping", "google.com", "443"])
            .unwrap();

        assert_eq!(*matches.get_one::<u32>("n").unwrap(), 10);
        assert_eq!(*matches.get_one::<u64>("t").unwrap(), 3);
        assert_eq!(*matches.get_one::<u64>("i").unwrap(), 500);
    }

    #[test]
    fn test_custom_values() {
        let matches = build_cli()
            .try_get_matches_from(vec![
                "tcping",
                "google.com",
                "443",
                "-n",
                "5",
                "-t",
                "10",
                "-i",
                "1000",
            ])
            .unwrap();

        assert_eq!(*matches.get_one::<u32>("n").unwrap(), 5);
        assert_eq!(*matches.get_one::<u64>("t").unwrap(), 10);
        assert_eq!(*matches.get_one::<u64>("i").unwrap(), 1000);
    }

    #[test]
    fn test_long_argument_names() {
        let matches = build_cli()
            .try_get_matches_from(vec![
                "tcping",
                "google.com",
                "443",
                "--num",
                "20",
                "--timeout",
                "5",
                "--interval",
                "200",
            ])
            .unwrap();

        assert_eq!(*matches.get_one::<u32>("n").unwrap(), 20);
        assert_eq!(*matches.get_one::<u64>("t").unwrap(), 5);
        assert_eq!(*matches.get_one::<u64>("i").unwrap(), 200);
    }

    #[test]
    fn test_parse_numeric_values() {
        let matches = build_cli()
            .try_get_matches_from(vec![
                "tcping",
                "localhost",
                "22",
                "-n",
                "15",
                "-t",
                "2",
                "-i",
                "750",
            ])
            .unwrap();

        assert_eq!(*matches.get_one::<u16>("port").unwrap(), 22);
        assert_eq!(*matches.get_one::<u32>("n").unwrap(), 15);
        assert_eq!(*matches.get_one::<u64>("t").unwrap(), 2);
        assert_eq!(*matches.get_one::<u64>("i").unwrap(), 750);
    }

    #[test]
    fn test_zero_values() {
        let matches = build_cli()
            .try_get_matches_from(vec![
                "tcping",
                "localhost",
                "22",
                "-n",
                "0",
                "-t",
                "0",
                "-i",
                "0",
            ])
            .unwrap();

        assert_eq!(*matches.get_one::<u32>("n").unwrap(), 0);
        assert_eq!(*matches.get_one::<u64>("t").unwrap(), 0);
        assert_eq!(*matches.get_one::<u64>("i").unwrap(), 0);
    }

    #[test]
    fn test_zero_timeout_rejected() {
        let matches = build_cli()
            .try_get_matches_from(vec!["tcping", "localhost", "22", "-t", "0"])
            .unwrap();

        let result = run_from_matches(&matches);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Timeout must be greater than 0"));
    }
}
