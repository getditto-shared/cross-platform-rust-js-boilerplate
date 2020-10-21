use async_trait::async_trait;

use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::*;

use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;

use js_sys::Promise;

// -----------------------------------------------------------------------------

#[async_trait]
pub trait Store {
    fn new(name: &str) -> Self;

    async fn get(&self, key: &str) -> Result<Option<String>, ()>;
    async fn put(&mut self, key: &str, value: &str) -> Result<(), ()>;

    fn clear(&mut self);
}

// -----------------------------------------------------------------------------

#[derive(Debug)]
pub struct InMemoryStore {
    entries: HashMap<String, String>,
}

#[async_trait]
impl Store for InMemoryStore {
    fn new(_name: &str) -> Self {
        // TODO: use global hash table and reference the entries by name
        // (so it can be "opened" later on, as with a real implementation
        // persisting contents).
        Self { entries: HashMap::new() }
    }

    async fn get(&self, key: &str) -> Result<Option<String>, ()> {
        let result = self.entries.get(&key.to_string());
        match result {
            Some(value) => Ok(Some(value.to_string())),
            None => Ok(None)
        }
    }

    async fn put(&mut self, key: &str, value: &str) -> Result<(), ()> {
        // TODO: delay a bit to simulate async.
        self.entries.insert(key.to_string(), value.to_string());
        Ok(())
    }

    fn clear(&mut self) {
        // TODO: this should probably be async too.
        self.entries.clear()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        // Nothing here yet, all good.
    }
}

// -----------------------------------------------------------------------------

#[wasm_bindgen]
pub struct JSStore {
    store: Rc<RefCell<InMemoryStore>>,
}

#[wasm_bindgen]
impl JSStore {

    #[wasm_bindgen(constructor)]
    pub fn new(name: &str) -> JSStore {
        let in_memory_store = InMemoryStore::new(name);
        let in_memory_store_ref_celled = RefCell::new(in_memory_store);
        let in_memory_store_reference_counted = Rc::new(in_memory_store_ref_celled);
        Self { store: in_memory_store_reference_counted }
    }

    #[wasm_bindgen]
    pub fn get(&self, key: &str) -> Promise {
        let store = self.store.clone();
        let key = key.to_string();
        let future = async move {
            let store = store.borrow();
            let result = store.get(&key).await;
            match result {
                Ok(value) => Ok(JsValue::from(value)),
                Err(_error) => Err(JsValue::undefined())
            }
        };
        future_to_promise(future)
    }

    #[wasm_bindgen]
    pub fn put(&self, key: &str, value: &str) -> Promise {
        let store = self.store.clone();
        let key = key.to_string();
        let value = value.to_string();
        let future = async move {
            let mut store = store.borrow_mut();
            let result = store.put(&key, &value).await;
            match result {
                Ok(_value) => Ok(JsValue::undefined()),
                Err(_error) => Err(JsValue::undefined())
            }
        };
        future_to_promise(future)
    }

    #[wasm_bindgen]
    pub fn clear(&self) {
        let mut store = self.store.borrow_mut();
        store.clear();
    }
}
