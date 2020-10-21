use async_trait::async_trait;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::*;
use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;
use js_sys::Promise;
use common::Store;
use web_sys::IdbFactory;

#[derive(Debug)]
pub struct InMemoryStore {
    entries: HashMap<String, String>,
}

#[async_trait]
impl Store for InMemoryStore {
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

impl InMemoryStore {
    fn new(_name: &str) -> Self {
        // TODO: use global hash table and reference the entries by name
        // (so it can be "opened" later on, as with a real implementation
        // persisting contents).
        Self { entries: HashMap::new() }
    }
}

#[derive(Debug)]
pub struct IndexedDBStore {
    name: String
}

#[async_trait]
impl Store for IndexedDBStore {
    async fn get(&self, key: &str) -> Result<Option<String>, ()> {
        let window = web_sys::window().expect("no global `window` exists");
        let indexed_db_option = window.indexed_db().expect("should have `indexedDB` on `window`.");
        let indexed_db = if indexed_db_option.is_some() { indexed_db_option.unwrap() } else { return Err(()) };
        let db = indexed_db.open(&self.name);
        dbg!(indexed_db);
        Err(())
    }

    async fn put(&mut self, key: &str, value: &str) -> Result<(), ()> {
        // TODO: implement.
        Err(())
    }

    fn clear(&mut self) {
        // TODO: implement.
    }
}

impl IndexedDBStore {
    fn new(name: &str) -> Self {
        Self { name: name.to_string() }
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

#[wasm_bindgen]
// NOTE: can't use term `Type` here, conflicts with keyword `type` and
// leads to weird variable names. Therefore "variant" it is.
pub enum JSStoreVariant {
    InMemory,
    IndexedDB
}

#[wasm_bindgen]
pub struct JSStore {
    store: Rc<RefCell<dyn Store>>,
}

#[wasm_bindgen]
impl JSStore {

    #[wasm_bindgen(constructor)]
    pub fn new(name: &str, variant: JSStoreVariant) -> JSStore {
        match variant {
            JSStoreVariant::InMemory => {
                let store = InMemoryStore::new(name);
                let store_ref_celled = RefCell::new(store);
                let store_reference_counted = Rc::new(store_ref_celled);
                Self { store: store_reference_counted }
            }

            JSStoreVariant::IndexedDB => {
                let store = IndexedDBStore::new(name);
                let store_ref_celled = RefCell::new(store);
                let store_reference_counted = Rc::new(store_ref_celled);
                Self { store: store_reference_counted }
            }
        }
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
