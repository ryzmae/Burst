#![allow(dead_code)]

use std::collections::{HashMap, VecDeque};
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
struct KeyValuePair {
    key: String,
    value: String,
    expiry: Instant,
}

pub struct Memory {
    data: HashMap<String, KeyValuePair>,
    expiry_queue: VecDeque<String>,
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            data: HashMap::new(),
            expiry_queue: VecDeque::new(),
        }
    }
    /// Set a key-value pair with an expiry time
    /// The key-value pair is stored in the data HashMap
    /// The key is also added to the expiry_queue VecDeque
    /// # Example
    /// ```no_run
    /// use std::time::Duration;
    /// let mut memory = Memory::new();
    /// memory.set("key".to_string(), "value".to_string(), Duration::from_secs(1));
    /// ```
    pub fn set(&mut self, key: String, value: String, expiry: Duration) -> () {
        let expires_at = Instant::now() + expiry;
        let kv_pair = KeyValuePair {
            key: key.clone(),
            value,
            expiry: expires_at,
        };
        self.data.insert(key.clone(), kv_pair);
        self.expiry_queue.push_back(key);
    }

    /// Get the value of a key
    /// # Example
    /// ```no_run
    /// let mut memory = Memory::new();
    /// memory.set("key".to_string(), "value".to_string(), Duration::from_secs(1));
    /// assert_eq!(memory.get("key"), Some(&"value".to_string()));
    /// ```
    pub fn get(&self, key: &str) -> Option<&String> {
        self.data.get(key).map(|kv_pair| &kv_pair.value) // Return the value if the key exists
    }

    /// Delete a key from the data HashMap and expiry_queue VecDeque
    /// # Example
    /// ```no_run
    /// let mut memory = Memory::new();
    /// memory.set("key".to_string(), "value".to_string(), Duration::from_secs(1));
    /// memory.delete("key");
    /// assert_eq!(memory.get("key"), None);
    /// ```
    pub fn delete(&mut self, key: &str) -> () {
        // Check if the key exists in the data
        if !self.data.contains_key(key) {
            return;
        }
        self.data.remove(key);
        self.expiry_queue.retain(|k| k != key);
    }   

    /// List all the key-value pairs in the data HashMap
    /// # Example
    /// ```no_run
    /// let mut memory = Memory::new();
    /// memory.set("key1".to_string(), "value1".to_string(), Duration::from_secs(1));
    /// memory.set("key2".to_string(), "value2".to_string(), Duration::from_secs(1));
    /// memory.set("key3".to_string(), "value3".to_string(), Duration::from_secs(1));
    /// let expected_keys = vec!["key1".to_string(), "key2".to_string(), "key3".to_string()];
    /// let listed_keys = memory.list().into_iter().map(|(k, _)| k).collect::<Vec<String>>();
    /// assert!(expected_keys.iter().all(|key| listed_keys.contains(key)));
    /// ```
    pub fn list(&self) -> Vec<(String, String)> {
        self.data
            .iter()
            .filter(|(_, kv_pair)| kv_pair.expiry > Instant::now())
            .map(|(key, kv_pair)| (key.clone(), kv_pair.value.clone())) // Clone the key and value
            .collect()
    }

    /// Clean expired key-value pairs from the data HashMap
    /// # Example
    /// ```no_run
    /// let mut memory = Memory::new();
    /// memory.set("key1".to_string(), "value1".to_string(), Duration::from_secs(1));
    /// memory.set("key2".to_string(), "value2".to_string(), Duration::from_secs(1));
    /// memory.set("key3".to_string(), "value3".to_string(), Duration::from_secs(1));
    /// std::thread::sleep(Duration::from_secs(2)); // Sleep for 2 seconds to expire all keys
    /// memory.clean_expired();
    /// assert_eq!(memory.list(), vec![]);
    /// ```
    pub fn clean_expired(&mut self) -> () {
        while let Some(key) = self.expiry_queue.front() {
            if let Some(kv_pair) = self.data.get(key) {
                if kv_pair.expiry <= Instant::now() {
                    self.data.remove(key);
                    self.expiry_queue.pop_front(); // Remove the key from the queue
                } else {
                    break;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_set_get() {
        let mut memory = Memory::new();
        memory.set(
            "key".to_string(),
            "value".to_string(),
            Duration::from_secs(1),
        );
        assert_eq!(memory.get("key"), Some(&"value".to_string())); // Assert that the value is set
    }

    #[test]
    fn test_delete() {
        let mut memory = Memory::new();
        memory.set(
            "key".to_string(),
            "value".to_string(),
            Duration::from_secs(1),
        );
        memory.delete("key");
        assert_eq!(memory.get("key"), None);
    }

    #[test]
    fn test_clean_expired() {
        let mut memory = Memory::new();
        memory.set(
            "key1".to_string(),
            "value1".to_string(),
            Duration::from_secs(1),
        );
        memory.set(
            "key2".to_string(),
            "value2".to_string(),
            Duration::from_secs(1),
        );
        memory.set(
            "key3".to_string(),
            "value3".to_string(),
            Duration::from_secs(1),
        );
        std::thread::sleep(Duration::from_secs(2)); // Sleep for 2 seconds to expire all keys
        memory.clean_expired();
        assert_eq!(memory.list(), vec![]);
    }

    #[test]
    fn test_list() {
        let mut memory = Memory::new();
        memory.set(
            "key1".to_string(),
            "value1".to_string(),
            Duration::from_secs(1),
        );
        memory.set(
            "key2".to_string(),
            "value2".to_string(),
            Duration::from_secs(1),
        );
        memory.set(
            "key3".to_string(),
            "value3".to_string(),
            Duration::from_secs(1),
        );

        let expected_keys = vec!["key1".to_string(), "key2".to_string(), "key3".to_string()];
        let listed_keys = memory
            .list()
            .into_iter()
            .map(|(k, _)| k)
            .collect::<Vec<String>>();

        // Check if all expected keys are present in the list
        assert!(expected_keys.iter().all(|key| listed_keys.contains(key)));
    }
}
