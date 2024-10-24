use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read, Write};
use std::time::{Duration, SystemTime};

#[derive(Deserialize, Serialize)]
pub struct Storage {
    pub store: HashMap<String, String>,
    pub ttl_map: HashMap<String, SystemTime>,
    pub current_transaction: Option<Transaction>,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Operation {
    Set(String, String),
    Delete(String),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Transaction {
    operations: Vec<Operation>,
}
impl Storage {
    pub fn new() -> Self {
        Storage {
            store: HashMap::new(),
            ttl_map: HashMap::new(),
            current_transaction: None,
        }
    }

    pub fn begin_transaction(&mut self) {
        self.current_transaction = Some(Transaction {
            operations: Vec::new(),
        });
    }

    pub fn add_operation(&mut self, operation: Operation) {
        if let Some(transaction) = &mut self.current_transaction {
            transaction.operations.push(operation);
        }
    }
    pub fn commit_transaction(&mut self) -> Result<(), String> {
        if let Some(transaction) = self.current_transaction.take() {
            for operation in transaction.operations {
                match operation {
                    Operation::Set(key, value) => self.set(key, value),
                    Operation::Delete(key) => {
                        let _ = self.delete(&key);
                    }
                }
            }
            Ok(())
        } else {
            Err("No active transactions".to_string())
        }
    }

    pub fn rollback_transaction(&mut self) {
        self.current_transaction = None;
    }
    pub fn set(&mut self, key: String, value: String) {
        self.store.insert(key, value);
    }
    pub fn set_with_ttl(&mut self, key: String, value: String, ttl: Option<Duration>) {
        self.store.insert(key.clone(), value);

        if let Some(ttl_duration) = ttl {
            let expiration_time = SystemTime::now() + ttl_duration;
            self.ttl_map.insert(key, expiration_time);
        } else {
            self.ttl_map.remove(&key);
        }
    }
    pub fn get(&mut self, key: &str) -> Option<&String> {
        if let Some(expiration) = self.ttl_map.get(key) {
            if SystemTime::now() > *expiration {
                self.store.remove(key);
                self.ttl_map.remove(key);
                return None;
            }
        }
        self.store.get(key)
    }

    pub fn clean_expired_keys(&mut self) {
        let now = SystemTime::now();
        let expired_keys: Vec<String> = self
            .ttl_map
            .iter()
            .filter(|(_, &expiraion)| now > expiraion)
            .map(|(key, _)| key.clone())
            .collect();

        for key in expired_keys {
            self.store.remove(&key);
            self.ttl_map.remove(&key);
        }
    }
    pub fn delete(&mut self, key: &str) -> Option<String> {
        self.store.remove(key)
    }
    pub fn save_to_file(&self, filename: &str) -> io::Result<()> {
        let mut file = File::create(filename)?;
        let serialized = serde_json::to_string(&self.store)?;
        writeln!(file, "{}", serialized)?;
        Ok(())
    }

    pub fn load_from_file(filename: &str) -> io::Result<Self> {
        let mut file = File::open(filename)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let store: HashMap<String, String> = serde_json::from_str(&contents)?;
        Ok(Storage {
            store,
            current_transaction: None,
            ttl_map: HashMap::new(),
        })
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_set_get() {
        let mut storage = Storage::new();
        storage.set("key1".to_string(), "value1".to_string());

        assert_eq!(storage.get("key1"), Some(&"value1".to_string()));
    }

    #[test]
    fn test_set_with_ttl() {
        let mut storage = Storage::new();
        storage.set_with_ttl(
            "key1".to_string(),
            "value1".to_string(),
            Some(Duration::new(2, 0)),
        );
        std::thread::sleep(Duration::new(3, 0));
        assert_eq!(storage.get("key1"), None);
    }

    #[test]
    fn test_delete() {
        let mut storage = Storage::new();
        storage.set("key1".to_string(), "value2".to_string());
        storage.delete("key1");
        assert_eq!(storage.get("key1"), None);
    }

    #[test]
    fn test_save_load() {
        let mut storage = Storage::new();
        storage.set("key1".to_string(), "value1".to_string());
        storage.set("key2".to_string(), "value2".to_string());

        storage.save_to_file("test_db.json").unwrap();

        let mut load_storage = Storage::load_from_file("test_db.json").unwrap();
        assert_eq!(load_storage.get("key1"), Some(&"value1".to_string()));
        assert_eq!(load_storage.get("key2"), Some(&"value2".to_string()));

        fs::remove_file("test_db.json").unwrap();
    }

    #[test]
    fn test_transaction() {
        let mut storage: Storage = Storage::new();
        storage.begin_transaction();

        storage.add_operation(Operation::Set("key1".to_string(), "value1".to_string()));
        storage.add_operation(Operation::Set("key2".to_string(), "value2".to_string()));
        let _ = storage.commit_transaction();

        assert_eq!(storage.get("key1"), Some(&"value1".to_string()));
        assert_eq!(storage.get("key2"), Some(&"value2".to_string()));

        storage.begin_transaction();
        storage.add_operation(Operation::Delete("key1".to_string()));
        let _ = storage.commit_transaction();
        assert_eq!(storage.get("key1"), None);
        storage.begin_transaction();
        storage.add_operation(Operation::Delete("key2".to_string()));
        storage.rollback_transaction();
        assert_eq!(storage.get("key2"), Some(&"value2".to_string()));
    }
}
