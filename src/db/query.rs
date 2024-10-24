use super::index::Index;
use super::storage::Storage;
use std::io;

pub struct Query {
    pub storage: Storage,
    pub index: Index,
}

impl Query {
    pub fn new() -> Self {
        Query {
            storage: Storage::new(),
            index: Index::new(),
        }
    }

    pub fn set(&mut self, key: String, value: String) {
        self.storage.set(key.clone(), value.clone());
        self.index.add(key, value);
    }

    pub fn get(&mut self, key: &str) -> Option<&String> {
        self.storage.get(key)
    }

    pub fn delete(&mut self, key: &str) {
        if let Some(value) = self.storage.delete(key) {
            self.index.remove(key, &value);
        }
    }
    pub fn search(&self, key: &str) -> Option<&Vec<String>> {
        self.index.get(key)
    }

    pub fn save(&self, filename: &str) -> io::Result<()> {
        self.storage.save_to_file(filename)
    }

    pub fn load(&mut self, filename: &str) -> io::Result<()> {
        let load_storage = Storage::load_from_file(filename)?;
        self.storage = load_storage;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_set_get() {
        let mut query = Query::new();
        query.set("key1".to_string(), "value1".to_string());

        assert_eq!(query.get("key1"), Some(&"value1".to_string()));
    }

    #[test]
    fn test_delete() {
        let mut query = Query::new();
        query.set("key1".to_string(), "value1".to_string());
        query.delete("key1");

        assert_eq!(query.get("key1"), None);
    }

    #[test]
    fn test_search_index() {
        let mut query = Query::new();
        query.set("key1".to_string(), "value1".to_string());
        query.set("key1".to_string(), "value2".to_string());

        let result = query.search("key1").unwrap();
        assert!(result.contains(&"value1".to_string()));
        assert!(result.contains(&"value2".to_string()));
    }

    #[test]
    fn test_save_load_query() {
        let mut query = Query::new();
        query.set("key1".to_string(), "value1".to_string());
        query.set("key2".to_string(), "value2".to_string());

        query.save("test_query_db.json").unwrap();

        let mut loaded_query = Query::new();
        loaded_query.load("test_query_db.json").unwrap();

        assert_eq!(loaded_query.get("key1"), Some(&"value1".to_string()));
        assert_eq!(loaded_query.get("key2"), Some(&"value2".to_string()));

        fs::remove_file("test_query_db.json").unwrap();
    }
}
