use crate::constants;
use crate::logger::{Level, Logger};
use crate::render::render_name;
use crate::memory;
use chrono;
use std::ptr::slice_from_raw_parts_mut;
use std::str::FromStr;
use std::{
    io::prelude::*,
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

pub fn run() {
    let _port = 4321; // constants::spawn_port();
    let _log: Logger = Logger::new(Level::Info);
    let addr: String = format!("{}:{}", constants::ADDRESS, _port);
    let listner: TcpListener = TcpListener::bind(addr).expect("Failed to bind to address!");
    start_screen(_port);

    let threadlog: Logger = Logger::new(Level::Info);

    thread::spawn(move || {
        loop {
            // Testing for some time other background thread will delete the expired keys this is just for showing the logs
            threadlog.info(&format!(
                "Current time: {}",
                chrono::Utc::now().format("%d-%m-%y %H:%M:%S"),
            ), None);
            thread::sleep(Duration::from_secs(1));
        }
    });

    for stream in listner.incoming() {
        match stream {
            Ok(stream) => {
                std::thread::spawn(|| handle_connection(stream));
            }
            Err(e) => {
                _log.trace(&format!("Failed to establish a connection: {}", e), None);
            }
        }
    }

    drop(listner); // Close the listener socket to free up the port
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer: [u8; 1024] = [0; 1024];
    let read: usize = stream.read(&mut buffer).unwrap();

    // If the buffer is empty, return
    if read == 0 {
        return;
    }

    let request: std::borrow::Cow<'_, str> = String::from_utf8_lossy(&buffer[..]);

    let date: String = chrono::Utc::now().format("%d-%m-%y %H:%M:%S").to_string();

    if request.contains("shutdown") {
        let response: String = "Server is shutting down...".to_string();

        let bytes_amount: usize = stream.write(response.as_bytes()).unwrap();

        if bytes_amount == 0 {
            return;
        }

        stream.flush().unwrap();

        std::process::exit(0);
    } else if request.contains("SET") {
        let args: Vec<&str> = request.split_whitespace().collect::<Vec<&str>>();

        let key: &str = args[1];
        let value: &str = args[2];
        let ttl = args[3];

        if key.is_empty() || value.is_empty() || ttl.is_empty() {
            let response: String = format!("[{}] - Key, value or ttl is empty...", date);

            let bytes_amount: usize = stream.write(response.as_bytes()).unwrap();

            if bytes_amount == 0 {
                return;
            }

            stream.flush().unwrap();

            return;
        } else if !ttl.is_empty() && !ttl.parse::<u64>().is_ok() {
            let response: String = format!("[{}] - TTL is not a number...", date);

            let bytes_amount: usize = stream.write(response.as_bytes()).unwrap();

            if bytes_amount == 0 {
                return;
            }

            stream.flush().unwrap();

            return;
        }

        let mut memory_struct = memory::Memory::new();

        // Check if the message_size is greater than the MAX_MESSAGE_SIZE
        if value.len() > constants::MAX_MESSAGE_SIZE {
            let response: String = format!("[{}] - Data size is too large...", date);

            let bytes_amount: usize = stream.write(response.as_bytes()).unwrap();

            if bytes_amount == 0 {
                return;
            }

            stream.flush().unwrap();

            return;
        }

        memory_struct.set(String::from(key), String::from(value), Duration::from_secs(ttl.parse::<u64>().unwrap()));
        let response: String = format!("[{}] - SET key: {} value: {}", date, key, value);

        let bytes_amount: usize = stream.write(response.as_bytes()).unwrap();

        if bytes_amount == 0 {
            return;
        }

        stream.flush().unwrap();
    } else {
        let response: String = format!("[{}] - Command not found...", date);

        let bytes_amount: usize = stream.write(response.as_bytes()).unwrap();

        if bytes_amount == 0 {
            return;
        }

        stream.flush().unwrap();
    }

    drop(stream); // Close the stream to free up the port
}

fn start_screen(port: u16) {
    let _log = Logger::new(Level::Info);
    render_name("Burst");

    _log.info(&format!(
        "v{} - PID {}",
        constants::VERSION,
        std::process::id()
    ), None);
    _log.info(&format!(
        "This instance of Burst is now ready to accept connections on port {}",
        port
    ), None);
    _log.info("- Press ^C to stop the server", None);

    drop(_log);
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
