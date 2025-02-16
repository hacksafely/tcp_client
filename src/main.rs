use std::io::{Read, Write};
use std::net::{TcpStream, ToSocketAddrs};

fn main() {

    let args = std::env::args().collect::<Vec<String>>();

    if args.len() < 3 {
        eprintln!("Usage: {} <host> <port>", args[0]);
        std::process::exit(1);
    }

    let host = &args[1];
    let binding = args[2].parse::<u16>();
    let port = match binding {
        Ok(port) => port,
        Err(_) => {eprintln!("Invalid port specified: {}", args[2]); std::process::exit(1);}
    };
    println!("host: {}, port: {}", host, port);

    // DNS Resolution to get IPv4 Address

    let addr = format!("{}:{}", host, port)
        .to_socket_addrs()
        .expect("Failed to resolve host")
        .find(|a| a.is_ipv4())
        .expect("No Ipv4 address found");

    println!("Resolved addresses: {:?}", addr);

    // Three-way handshake and creating a stream

    let mut stream = TcpStream::connect(addr).expect("Failed to connect to server");
    println!("Connected to server");

    // Sending HTTP request
    let request = "GET / HTTP/1.1\r\nHost: www.google.com\r\nConnection: close\r\n\r\n";
    stream.write_all(request.as_bytes()).expect("Failed to write to server");

    // Reading the response from the server
    let mut buffer = Vec::new();// 4KB buffer
    stream.read_to_end(&mut buffer).expect("Failed to read from server");

    // Print response
    println!("{}", String::from_utf8_lossy(&buffer));
}
