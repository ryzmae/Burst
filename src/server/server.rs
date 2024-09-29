use std::{
    io::prelude::*,
    net::{TcpListener, TcpStream},
};
use chrono;
use crate::log::Log;
use crate::server::render_name::render_name;
use crate::constants;

pub fn run() {
    let addr = format!("{}:{}", constants::ADDRESS, constants::PORT);
    let listner = TcpListener::bind(addr).expect("Failed to bind to address!");
    start_screen(constants::PORT);

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

    let date = chrono::Utc::now().format("%d-%m-%y %H:%M:%S").to_string();

    if request.contains("shutdown") {
        let response = format!("[{}] - Server is shutting down...", date);

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();

        log.info(&response);
        println!("{}", response);

        std::process::exit(0);
    } else if request.contains("SET") {
        let args = request.split_whitespace().collect::<Vec<&str>>();

        let key = args[1];
        let value = args[2];

        let response = format!("[{}] - SET key: {} value: {}", date, key, value);

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();

        log.info(&response);

        return;
    } else {
        let response = format!("[{}] - Command not found...", date);

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();

        log.info(&response);

        return;
    }
}

fn start_screen(port: u16) {
    render_name("Burst.sh");

    let pid = std::process::id();
    let date = chrono::Utc::now().format("%d-%m-%y %H:%M:%S").to_string();

    println!("[{}] - v{} - PID {} - {}", date, constants::VERSION, pid, constants::DOMAIN_NAME);

    println!("[{}] - This server is now ready to accept connections on port {}", date, port);
    println!("[{}] - Press ^C to stop the server", date);
}
