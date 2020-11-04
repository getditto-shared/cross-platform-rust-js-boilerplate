use std::collections::HashMap;
use node_bindgen::{core::{TryIntoJs, val::JsEnv, NjError}, derive::*};
use node_bindgen::sys::napi_value;

/// Copied from wasm module -- to be replaced with sled
#[derive(Debug)]
pub struct InMemoryStore {
    entries: HashMap<String, String>,
}

impl InMemoryStore {
    fn new(_name: &str) -> Self {
        // TODO: use global hash table and reference the entries by name
        // (so it can be "opened" later on, as with a real implementation
        // persisting contents).
        Self {
            entries: HashMap::new(),
        }
    }

    async fn get(&self, key: &str) -> Result<Option<String>, ()> {
        let result = self.entries.get(&key.to_string());
        match result {
            Some(value) => Ok(Some(value.to_string())),
            None => Ok(None),
        }
    }

    async fn put(&mut self, key: &str, value: &str) -> Result<(), ()> {
        // TODO: delay a bit to simulate async.
        self.entries.insert(key.to_string(), value.to_string());
        Ok(())
    }

    async fn clear(&mut self) -> Result<(), ()> {
        self.entries.clear();
        Ok(())
    }
}

#[derive(Debug)]
struct NativeStore {
    store: InMemoryStore,
}

#[node_bindgen]
impl NativeStore {
    #[node_bindgen(constructor)]
    fn new(name: String) -> Self {
        Self { store: InMemoryStore::new(&name) }
    }

    #[node_bindgen]
    async fn get(&self, key: String) -> NapiOptString {
        self.store.get(&key).await.map(NapiOptString).unwrap()
    }

    #[node_bindgen]
    async fn put(&mut self, key: String, value: String) {
        self.store.put(&key, &value).await.unwrap()
    }

    #[node_bindgen]
    async fn clear(&mut self) {
        self.store.clear().await.unwrap()
    }
}

// wait for https://github.com/infinyon/node-bindgen/issues/33
struct NapiOptString(Option<String>);

impl TryIntoJs for NapiOptString {
    fn try_to_js(self, js_env: &JsEnv) -> Result<napi_value,NjError> {
        if let Some(x) = self.0 {
            js_env.create_string_utf8(&x)
        } else {
            js_env.get_undefined()
        }
    }
}