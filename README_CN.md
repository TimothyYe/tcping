# TCPing

TCPing 是一个命令行工具，用于 ping 指定主机上的 TCP 端口，使用 Rust 实现。它提供了 ping 主机时的平均、最大和最小延迟等统计信息。

[![asciicast](https://asciinema.org/a/CgpXmFi9g8guvfOyDpm9ehtml.svg)](https://asciinema.org/a/CgpXmFi9g8guvfOyDpm9ehtml)

## 功能

- ping 指定主机上的 TCP 端口。
- 支持位置参数和命名参数。
- 计算并显示包括平均、最大和最小延迟在内的统计信息。

## 使用方法

```sh
tcping <host> <port> [-n <number_of_pings>] [-t <timeout>] [-i <interval>]
```

### 参数

* `<host>` : 要 ping 的主机。
* `<port>` : 要 ping 的端口。
* `-n, --num` : ping 尝试次数（默认为 10）。
* `-t, --timeout` : 每次连接尝试的超时时间，以秒为单位（默认为 3）。
* `-i, --interval` : ping 之间的时间间隔，以毫秒为单位（默认为 500）。

### 示例

```sh
tcping google.com 443 -n 5
tcping google.com 443 -n 20 -t 5 -i 1000
```

## 安装

### 从源代码安装

1. 克隆仓库：

```sh
git clone https://github.com/TimothyYe/tcping.git
```

2. 构建项目：

```sh
cargo build --release
```

3. 运行二进制文件：

```sh
./target/release/tcping google.com 443 -n 10 -t 3 -i 500
```

### 从二进制文件安装

从 [Releases](https://github.com/TimothyYe/tcping/releases) 页面下载二进制文件。

## 许可证

本项目采用 Apache License 2.0 许可证 - 详情请参见 [LICENSE](https://github.com/TimothyYe/tcping/blob/master/LICENSE) 文件。
