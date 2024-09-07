# TCPing

TCPing is a command-line tool for pinging a TCP port on a specified host, implemented in Rust. It provides statistics such as average, maximum, and minimum latency for the pinged host.

## Features

- Ping a TCP port on a specified host.
- Support for both positional and named parameters.
- Calculate and display statistics including average, maximum, and minimum latency.

## Usage

```sh
tcping <host> <port> -n <number_of_pings>
```

### Parameters

* `<host>` : The host to ping.
* `<port>` : The port to ping.
* `-n, --number` : The number of ping attempts (default is 4).

### Example

```sh
tcping google.com 443 -n 10
```

## Installation

### From Source

1. Clone the repository:

```sh
git clone https://github.com/TimothyYe/tcping.git
```

2. Build the project:

```sh
cargo build --release
```

3. Run the binary:

```sh
./target/release/tcping google.com 443 -n 10
```

### From Binary

Download the binary from the [Releases]() page.

## License

This project is licensed under the Apache License 2.0 - see the [LICENSE](https://github.com/TimothyYe/tcping/blob/master/LICENSE) file for details.