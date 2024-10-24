use std::time::Duration;

use crate::db::storage::Storage;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct WasmStorage {
    inner: Storage,
}

#[wasm_bindgen]
impl WasmStorage {
    #[wasm_bindgen(constructor)]
    pub fn new() -> WasmStorage {
        WasmStorage {
            inner: Storage::new(),
        }
    }

    #[wasm_bindgen]
    pub fn set_with_ttl(&mut self, key: String, value: String, ttl: u64) {
        self.inner
            .set_with_ttl(key, value, Some(Duration::new(ttl, 0)));
    }

    #[wasm_bindgen]
    pub fn get(&mut self, key: &str) -> Option<String> {
        self.inner.get(key).cloned()
    }

    #[wasm_bindgen]
    pub fn set(&mut self, key: String, value: String) {
        self.inner.set(key, value)
    }
}
