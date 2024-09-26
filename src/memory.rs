use std::collections::HashMap;
use std::sync::{Arc, Mutex};


#[derive(Debug)]
struct Entry {
    _id: String,
    data: Vec<u8>,
}


type MemoryStore = Arc<Mutex<HashMap<String, Entry>>>;

pub struct MemoryCache {
    store: MemoryStore,
}

impl MemoryCache {
    pub fn new() -> Self {
        MemoryCache {
            store: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn store(&self, _id: String, data: Vec<u8>) -> Result<(), String> {
        let mut store = self.store.lock().unwrap();

        // Generates a random ID and checks if it already exists,
        // if it does it will generate a new one until it finds a unique one
        let id = loop {
            let random_id = format!("{:x}", rand::random::<u64>());
            if !store.contains_key(&random_id) {
                break random_id;
            }
        };

        let entry = Entry { _id: id.clone(), data };
        store.insert(id, entry);

        Ok(())
    }

    pub fn get(&self, id: &str) -> Result<Vec<u8>, String> {
        let store = self.store.lock().unwrap();

        // Return a clone of the pre-existing data in memory
        match store.get(id) {
            Some(entry) => Ok(entry.data.clone()),
            None => Err(format!("ID '{}' not found", id)),
        }
    }

    pub fn delete(&self, id: &str) -> Result<(), String> {
        let mut store = self.store.lock().unwrap();

        match store.remove(id) {
            Some(_) => Ok(()),
            None => Err(format!("ID '{}' not found", id)),
        }
    }

    pub fn clear(&self) {
        let mut store = self.store.lock().unwrap();
        store.clear();
    }
}