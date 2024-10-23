use std::collections::HashMap;

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
}

#[cfg(test)]
mod tests {
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
}
