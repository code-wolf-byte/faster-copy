use std::net::{TcpStream, SocketAddr};
use std::thread;
use std::sync::mpsc;
use std::env;

const CONCURRENT_TASKS: usize = 10;



pub struct Network{
    ip :String,
    port: u16,
    flags : Vec<Flag>
}



fn scan_port(ip: &str, port: u16) -> Option<u16> {
    let address = format!("{}:{}", ip, port);
    let socket_addr: SocketAddr = address.parse().unwrap();

    match TcpStream::connect(&socket_addr) {
        Ok(_) => Some(port),
        Err(_) => None,
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    print!("{:?}", args);
    if args.len() < 1 {
        eprintln!("Usage: port_scanner <ip>");
        std::process::exit(1);
    }

    let ip = &args[1];
    let ports: Vec<u16> = (1..=65535).collect();
    let (tx, rx) = mpsc::channel();

    for chunk in ports.chunks(CONCURRENT_TASKS) {
        let tx = tx.clone();
        let ip = ip.to_string();
        let chunk = chunk.to_vec();

        thread::spawn(move || {
            for port in chunk {
                print!("Checking port {}...\n", port);
                if let Some(port) = scan_port(&ip, port) {
                    tx.send(port).unwrap();
                }
            }
        });
    }

    drop(tx); // Close the sending side

    let open_ports: Vec<u16> = rx.iter().collect();
    println!("Open ports: {:?}", open_ports);
}
