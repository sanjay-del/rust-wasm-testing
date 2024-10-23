use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read, Write};

#[derive(Deserialize, Serialize)]
pub struct Storage {
    pub store: HashMap<String, String>,
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

    pub fn get(&self, key: &str) -> Option<&String> {
        self.store.get(key)
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

        let load_storage = Storage::load_from_file("test_db.json").unwrap();
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
