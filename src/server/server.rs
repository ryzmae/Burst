use std::{
    io::prelude::*,
    net::{TcpListener, TcpStream},
};
use chrono;
use crate::log::Log;
use crate::server::render_name::render_name;

pub fn run(addr: &str, port: u16) {
    let addr = format!("{}:{}", addr, port);
    let listner = TcpListener::bind(addr).unwrap();
    start_screen(port);

    for stream in listner.incoming() {
        match stream {
            Ok(stream) => {
                std::thread::spawn(|| handle_connection(stream));
            }
            Err(e) => {
                eprintln!("Failed to establish a connection: {}", e);
            }
        }
    }
}


fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    let log = Log::new();

    stream.read(&mut buffer).expect("Failed to read from client!");
    let request = String::from_utf8_lossy(&buffer[..]);
    let loginfo = format!("Request: {}", request);

    log.info(&loginfo);

    if request.contains("status") {
        let response = "HTTP/1.1 200 OK\r\n\r\n";

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();

        log.info("Response sent!");

        return;
    } else if request.contains("shutdown") {
        let response = "HTTP/1.1 200 OK\r\n\r\n";

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();

        log.info("Response sent!");

        std::process::exit(0);
    } else {
        let response = "HTTP/1.1 404 NOT FOUND\r\n\r\n";

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();

        log.info("Response sent!");

        return;
    }
}

fn start_screen(port: u16) {
    render_name("Burst.sh");

    let version = env!("CARGO_PKG_VERSION");
    let pid = std::process::id();
    let site = "http://burst.sh";
    let date = chrono::Utc::now();
    let date = date.format("%d-%m-%y %H:%M:%S").to_string();

    println!("[{}] - v{} - PID {} - {}", date, version, pid, site);

    println!("[{}] - This server is now ready to accept connections on port {}", date, port)
}
