use std::collections::HashMap;
pub struct Index {
    pub index: HashMap<String, Vec<String>>,
}

impl Index {
    pub fn new() -> Self {
        Index {
            index: HashMap::new(),
        }
    }
    pub fn add(&mut self, key: String, value: String) {
        self.index.entry(key).or_insert_with(Vec::new).push(value);
    }

    pub fn get(&self, key: &str) -> Option<&Vec<String>> {
        self.index.get(key)
    }

    pub fn remove(&mut self, key: &str, value: &str) {
        if let Some(values) = self.index.get_mut(key) {
            values.retain(|v| v != value);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_get() {
        let mut index = Index::new();
        index.add("key1".to_string(), "value1".to_string());
        index.add("key1".to_string(), "value2".to_string());

        let values = index.get("key1").unwrap();
        assert!(values.contains(&"value1".to_string()));
        assert!(values.contains(&"value2".to_string()));
    }

    #[test]
    fn test_remove() {
        let mut index = Index::new();
        index.add("key1".to_string(), "value1".to_string());
        index.add("key1".to_string(), "value2".to_string());

        index.remove("key1", "value1");
        let values = index.get("key1").unwrap();
        assert!(!values.contains(&"value1".to_string()));
        assert!(values.contains(&"value2".to_string()));
    }
}
