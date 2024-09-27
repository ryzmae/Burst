use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;

const DEFAULT_DATA_FILE: &str = "../test.burst";
const DEFAULT_DUMP_FILE: &str = "../dump.burst";

pub struct Store {
    pub data_file: PathBuf,
    pub dump_file: PathBuf,
}

impl Store {
    pub fn new() -> Store {
        Store {
            data_file: PathBuf::from(DEFAULT_DATA_FILE),
            dump_file: PathBuf::from(DEFAULT_DUMP_FILE),
        }
    }

    pub fn send_to_dump_file(&self, data: String) {
        let mut file = File::create(&self.dump_file).unwrap();
        file.write_all(data.as_bytes()).unwrap();
    }

    pub fn set_data(&self, data: String) -> Result<(), std::io::Error> {
        if !self.data_file.exists() {
            let mut file = File::create(DEFAULT_DATA_FILE).unwrap();
            file.write_all(data.as_bytes()).unwrap();
        } else if self.data_file.exists() {
            let mut file = File::open(DEFAULT_DATA_FILE).unwrap();
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();
            let mut file = File::create(DEFAULT_DATA_FILE).unwrap();
            file.write_all(contents.as_bytes()).unwrap();
        }

        self.send_to_dump_file(data);

        return Ok(());
    }

    pub fn get_data(&self) -> Result<String, std::io::Error> {
        let mut file = File::open(DEFAULT_DATA_FILE).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        return Ok(contents);
    }

    pub fn get_dump(&self) -> Result<String, std::io::Error> {
        let mut file = File::open(DEFAULT_DUMP_FILE).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        return Ok(contents);
    }

    pub fn clear_data(&self) -> Result<(), std::io::Error> {
        let mut file = File::create(DEFAULT_DATA_FILE).unwrap();
        file.write_all("".as_bytes()).unwrap();

        return Ok(());
    }

    pub fn clear_dump(&self) -> Result<(), std::io::Error> {
        let mut file = File::create(DEFAULT_DUMP_FILE).unwrap();
        file.write_all("".as_bytes()).unwrap();

        return Ok(());
    }

    pub fn clear_all(&self) -> Result<(), std::io::Error> {
        self.clear_data()?;
        self.clear_dump()?;

        return Ok(());
    }

    pub fn send_dump_data(&self) -> Result<(), std::io::Error> {
        let mut file = File::open(DEFAULT_DUMP_FILE).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let mut file = File::create(DEFAULT_DATA_FILE).unwrap();
        file.write_all(contents.as_bytes()).unwrap();

        return Ok(());
    }

    pub fn delete(&self, data: String) -> () {
        let mut file = File::open(DEFAULT_DATA_FILE).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let mut file = File::create(DEFAULT_DATA_FILE).unwrap();
        file.write_all(contents.replace(data.as_str(), "").as_bytes()).unwrap();
    }
}