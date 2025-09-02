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
    if timeout_secs == 0 {
        return Err("Timeout must be greater than 0 seconds".into());
    }
    let timeout = Duration::from_secs(timeout_secs);

    let interval_ms = matches.get_one::<String>("i").unwrap().parse::<u64>()?;
    let interval = Duration::from_millis(interval_ms);

    tcping::run_tcping(host, port_num, num_pings, timeout, interval)
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Command;

    fn create_test_app() -> Command {
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
    }

    #[test]
    fn test_valid_port_parsing() {
        let matches = create_test_app()
            .try_get_matches_from(vec!["tcping", "google.com", "443"])
            .unwrap();

        let port_result = matches.get_one::<String>("port").unwrap().parse::<u16>();
        assert!(port_result.is_ok());
        assert_eq!(port_result.unwrap(), 443);
    }

    #[test]
    fn test_invalid_port_parsing() {
        let matches = create_test_app()
            .try_get_matches_from(vec!["tcping", "google.com", "70000"])
            .unwrap();

        let port_result = matches.get_one::<String>("port").unwrap().parse::<u16>();
        assert!(port_result.is_err());
    }

    #[test]
    fn test_default_values() {
        let matches = create_test_app()
            .try_get_matches_from(vec!["tcping", "google.com", "443"])
            .unwrap();

        assert_eq!(matches.get_one::<String>("n").unwrap(), "10");
        assert_eq!(matches.get_one::<String>("t").unwrap(), "3");
        assert_eq!(matches.get_one::<String>("i").unwrap(), "500");
    }

    #[test]
    fn test_custom_values() {
        let matches = create_test_app()
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

        assert_eq!(matches.get_one::<String>("n").unwrap(), "5");
        assert_eq!(matches.get_one::<String>("t").unwrap(), "10");
        assert_eq!(matches.get_one::<String>("i").unwrap(), "1000");
    }

    #[test]
    fn test_long_argument_names() {
        let matches = create_test_app()
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

        assert_eq!(matches.get_one::<String>("n").unwrap(), "20");
        assert_eq!(matches.get_one::<String>("t").unwrap(), "5");
        assert_eq!(matches.get_one::<String>("i").unwrap(), "200");
    }

    #[test]
    fn test_parse_numeric_values() {
        let matches = create_test_app()
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

        // Test that all numeric parsing works
        let num_pings = matches.get_one::<String>("n").unwrap().parse::<u32>();
        let timeout = matches.get_one::<String>("t").unwrap().parse::<u64>();
        let interval = matches.get_one::<String>("i").unwrap().parse::<u64>();

        assert!(num_pings.is_ok());
        assert!(timeout.is_ok());
        assert!(interval.is_ok());

        assert_eq!(num_pings.unwrap(), 15);
        assert_eq!(timeout.unwrap(), 2);
        assert_eq!(interval.unwrap(), 750);
    }

    #[test]
    fn test_zero_values() {
        let matches = create_test_app()
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

        // Test parsing zero values
        let num_pings = matches
            .get_one::<String>("n")
            .unwrap()
            .parse::<u32>()
            .unwrap();
        let timeout = matches
            .get_one::<String>("t")
            .unwrap()
            .parse::<u64>()
            .unwrap();
        let interval = matches
            .get_one::<String>("i")
            .unwrap()
            .parse::<u64>()
            .unwrap();

        assert_eq!(num_pings, 0);
        assert_eq!(timeout, 0);
        assert_eq!(interval, 0);
    }
}
