use log::Level::{Debug, Error, Info, Trace, Warn};
use std::io::{Read, Write};
use std::fs::File;
use std::path::PathBuf;

const DEFAULT_LOG_FILE: &str = "../burst.log";

pub struct Log {
    pub log_file: PathBuf,
}

impl Log {
    pub fn new() -> Log {
        Log {
            log_file: PathBuf::from(DEFAULT_LOG_FILE),
        }
    }

    fn exists(&self) -> bool {
        self.log_file.exists()
    }

    pub fn log(&self, level: log::Level, message: &str) -> () {
        let log_message = format!("[{}] - {}\n", level, message);
        if self.exists() {
            let mut file = File::open(DEFAULT_LOG_FILE).unwrap();

            file.write_all(log_message.as_bytes()).unwrap();

            return;
        } else {
            let mut file = File::create(DEFAULT_LOG_FILE).unwrap();
            file.write_all(log_message.as_bytes()).unwrap();           
            return;
        }
    }

    pub fn debub(&self, message: &str) -> () {
        self.log(Debug, message);
    }

    pub fn info(&self, message: &str) -> () {
        self.log(Info, message);
    }

    pub fn trace(&self, message: &str) -> () {
        self.log(Trace, message);
    }

    pub fn warn(&self, message: &str) -> () {
        self.log(Warn, message);
    }

    pub fn error(&self, message: &str) -> () {
        self.log(Error, message);
    }

    pub fn get_log(&self) -> Result<String, std::io::Error> {
        let mut file = File::open(DEFAULT_LOG_FILE).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        return Ok(contents);
    }

    pub fn clear_log(&self) -> Result<(), std::io::Error> {
        let mut file = File::create(DEFAULT_LOG_FILE).unwrap();
        file.write_all("".as_bytes()).unwrap();

        return Ok(());
    }
}