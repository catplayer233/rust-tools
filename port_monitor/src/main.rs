use std::env;
use std::net::{Ipv4Addr, SocketAddrV4, TcpStream};
use std::net::Shutdown::Both;
use std::process;
use std::thread;
use std::time::{Duration, SystemTime};

const WRONG_ARGUMENT_LENGTH: i32 = 1;
const WRONG_IP_FORMAT: i32 = 2;
const WRONG_PORT: i32 = 3;
const WRONG_IP_PART: i32 = 4;

fn main() {
    println!("******start monitor******");
    let start_time = SystemTime::now();
    let arguments: Vec<String> = env::args().collect();
    let socket_addr_v4 = new_socket_v4_addr(arguments);

    let tcp_stream = try_connect(&socket_addr_v4);
    let end_time = SystemTime::now();
    println!("******cost time is {} mills******", end_time.duration_since(start_time).unwrap().as_millis());
    tcp_stream.shutdown(Both).unwrap_or_else(|err| {
        eprintln!("close with error:{}", err);
    });
}

fn new_socket_v4_addr(arguments: Vec<String>) -> SocketAddrV4 {
    if arguments.len() < 3 {
        eprintln!("you should invoke with ip and port arguments");
        eprintln!("eg. xxx.exe 127.0.0.1 8080");
        process::exit(WRONG_ARGUMENT_LENGTH);
    }
    let ip_port_vec: Vec<&str> = arguments[1].split(".").collect();
    if ip_port_vec.len() < 4 {
        eprintln!("wrong ip format");
        eprintln!("eg. 127.0.0.1");
        process::exit(WRONG_IP_FORMAT);
    }
    let ipv4addr = Ipv4Addr::new(
        parse_ip_part(ip_port_vec[0]),
        parse_ip_part(ip_port_vec[1]),
        parse_ip_part(ip_port_vec[2]),
        parse_ip_part(ip_port_vec[3]));
    let port: u16 = arguments[2].parse().unwrap_or_else(|_err| {
        eprintln!("wrong port, must be an int");
        process::exit(WRONG_PORT);
    });
    SocketAddrV4::new(ipv4addr, port)
}

fn parse_ip_part(part: &str) -> u8 {
    part.parse().unwrap_or_else(|_err| {
        eprintln!("wrong port, must be an int");
        process::exit(WRONG_IP_PART);
    })
}

fn try_connect(socket_addr: &SocketAddrV4) -> TcpStream {
    let tcp_stream = TcpStream::connect(&socket_addr).unwrap_or_else(|_err| {
        thread::sleep(Duration::from_secs(1));
        try_connect(&socket_addr)
    });
    tcp_stream
}
