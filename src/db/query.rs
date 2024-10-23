use super::index::Index;
use super::storage::Storage;

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

    pub fn get(&self, key: &str) -> Option<&String> {
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
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
