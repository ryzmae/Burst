use std::{
    io::prelude::*,
    net::{TcpListener, TcpStream},
};
use chrono;
use crate::logger::{Level, Logger};
use crate::render::render_name;
use crate::constants;

pub fn run() {
    let _log = Logger::new(Level::Info);
    let addr = format!("{}:{}", constants::ADDRESS, constants::PORT);
    let listner = TcpListener::bind(addr).expect("Failed to bind to address!");
    start_screen(constants::PORT);

    for stream in listner.incoming() {
        match stream {
            Ok(stream) => {
                std::thread::spawn(|| handle_connection(stream));
            }
            Err(e) => {
                _log.trace(&format!("Failed to establish a connection: {}", e));
            }
        }
    }
}


fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    let log = Logger::new(Level::Info);

    stream.read(&mut buffer).expect("Failed to read from client!");
    let request = String::from_utf8_lossy(&buffer[..]);
    let loginfo = format!("Request: {}", request);

    log.info(&loginfo);

    let date = chrono::Utc::now().format("%d-%m-%y %H:%M:%S").to_string();

    if request.contains("shutdown") {
        let response = format!("Server is shutting down...");

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();

        log.info(&response);
        
        log.info("Shutting down the server...");

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
    let _log = Logger::new(Level::Info);
    render_name("Burst");

    _log.info(&format!("v{} - PID {}", constants::VERSION, std::process::id()));
    _log.info(&format!("This instance of Burst is now ready to accept connections on port {}",port));
    _log.info(&format!("- Press ^C to stop the server",));
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_handle_connection_set_command() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();

        thread::spawn(move || {
            for stream in listener.incoming() {
                match stream {
                    Ok(stream) => handle_connection(stream),
                    Err(_) => break,
                }
            }
        });

        let mut stream = TcpStream::connect(addr).unwrap();
        stream.write(b"SET key value").unwrap();
        stream.flush().unwrap();

        let mut buffer = [0; 1024];
        stream.read(&mut buffer).unwrap();
        let response = String::from_utf8_lossy(&buffer[..]);

        assert!(response.contains("SET key: key value: value"));
    }

    #[test]
    fn test_handle_connection_shutdown_command() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();

        thread::spawn(move || {
            for stream in listener.incoming() {
                match stream {
                    Ok(stream) => handle_connection(stream),
                    Err(_) => break,
                }
            }
        });

        let mut stream = TcpStream::connect(addr).unwrap();
        stream.write(b"shutdown").unwrap();
        stream.flush().unwrap();

        let mut buffer = [0; 1024];
        stream.read(&mut buffer).unwrap();
        let response = String::from_utf8_lossy(&buffer[..]);

        assert!(response.contains("Server is shutting down..."));
    }

    #[test]
    fn test_handle_connection_unknown_command() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();

        thread::spawn(move || {
            for stream in listener.incoming() {
                match stream {
                    Ok(stream) => handle_connection(stream),
                    Err(_) => break,
                }
            }
        });

        let mut stream = TcpStream::connect(addr).unwrap();
        stream.write(b"UNKNOWN").unwrap();
        stream.flush().unwrap();

        let mut buffer = [0; 1024];
        stream.read(&mut buffer).unwrap();
        let response = String::from_utf8_lossy(&buffer[..]);

        assert!(response.contains("Command not found..."));
    }
}
