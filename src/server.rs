use std::{
    io::prelude::*,
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};
use chrono;
use crate::logger::{Level, Logger};
use crate::render::render_name;
use crate::constants;

pub fn run() {
    let _port = constants::spawn_port();
    let _log = Logger::new(Level::Info);
    let addr = format!("{}:{}", constants::ADDRESS, _port);
    let listner = TcpListener::bind(addr).expect("Failed to bind to address!");
    start_screen(_port);


    let threadlog = Logger::new(Level::Info);

    thread::spawn(move || {
        loop {
            // Testing for some time other background thread will delete the expired keys this is just for showing the logs
            threadlog.info(&format!("Current time: {}", chrono::Utc::now().format("%d-%m-%y %H:%M:%S")));
            thread::sleep(Duration::from_secs(1));
        }
    });

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
    let read = stream.read(&mut buffer).unwrap();

    // If the buffer is empty, return
    if read == 0 {
        return;
    }

    let log = Logger::new(Level::Info);
    let request = String::from_utf8_lossy(&buffer[..]);
    let loginfo = format!("Request: {}", request);

    log.info(&loginfo);

    let date = chrono::Utc::now().format("%d-%m-%y %H:%M:%S").to_string();

    if request.contains("shutdown") {
        let response = "Server is shutting down...".to_string();

        let bytes_amount = stream.write(response.as_bytes()).unwrap();

        if bytes_amount == 0 {
            return;
        }

        stream.flush().unwrap();

        log.info(&response);

        std::process::exit(0);
    } else if request.contains("SET") {
        let args = request.split_whitespace().collect::<Vec<&str>>();

        let key = args[1];
        let value = args[2];

        let response = format!("[{}] - SET key: {} value: {}", date, key, value);

        let bytes_amount = stream.write(response.as_bytes()).unwrap();

        if bytes_amount == 0 {
            return;
        }

        stream.flush().unwrap();

        log.info(&response);

        return;
    } else {
        let response = format!("[{}] - Command not found...", date);

        let bytes_amount = stream.write(response.as_bytes()).unwrap();

        if bytes_amount == 0 {
            return;
        }

        stream.flush().unwrap();

        log.info(&response);

        return;
    }
}

fn start_screen(port: u16) {
    let _log = Logger::new(Level::Info);
    render_name("Burst");

    _log.info(&format!("v{} - PID {}", constants::VERSION, std::process::id()));
    _log.info(&format!("This instance of Burst is now ready to accept connections on port {}", port));
    _log.info("- Press ^C to stop the server");
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
