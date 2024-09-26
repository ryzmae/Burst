// This file contains the kvstore implementation

use std::collections::HashMap;
pub use crate::store::Store;

pub struct KvStore {
    pub store: HashMap<String, String>,
}

impl KvStore {
    pub fn new() -> () {
        KvStore {
            store: HashMap::new(),
        };
    }

    pub fn set(&mut self, key: String, value: String) -> () { 
        let _store = Store::new();


        if self.store.contains_key(key.as_str()) {
            self.store.remove_entry(key.as_str());
            _store.delete(key);

            return;
        } else if !self.store.contains_key(key.as_str()) {
            let value_key_merged = format!("{}={}", key, value);
            self.store.insert(key, value);
            let _ = _store.set_data(value_key_merged);

            return;
        }

        return;
    }
}