use std::io;
use std::net::UdpSocket;

fn main() -> io::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:1800")?;
    let mut buffer = [0; 65_507];

    println!("Listening for Anno 1800 events on 0.0.0.0:1800");

    loop {
        let (len, from) = socket.recv_from(&mut buffer)?;
        println!("--- event from {from} ({len} bytes) ---");
        println!("{}", String::from_utf8_lossy(&buffer[..len]));
    }
}
