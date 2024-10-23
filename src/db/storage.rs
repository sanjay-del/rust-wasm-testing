use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read, Write};

#[derive(Deserialize, Serialize)]
pub struct Storage {
    pub store: HashMap<String, String>,
}
impl Storage {
    pub fn new() -> Self {
        Storage {
            store: HashMap::new(),
        }
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
        Ok(Storage { store })
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
}
