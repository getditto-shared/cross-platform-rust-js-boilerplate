use common::Store;
use async_trait::async_trait;
use node_bindgen::{core::{TryIntoJs, val::JsEnv, NjError}, derive::*};
use node_bindgen::sys::napi_value;

#[derive(Debug)]
pub struct SledStore {
    db: sled::Db,
}

// #[async_trait(?Send)]
impl SledStore {
    async fn get(&self, key: &str) -> Result<Option<String>, ()> {
        // TODO: properly handle errors.
        match self.db.get(key) {
            Ok(value_vec_option) => Ok(value_vec_option.map(|value_vec| {
                let value_data: &[u8] = &value_vec;
                let value_str = std::str::from_utf8(value_data).unwrap();
                value_str.to_string()
            })),

            Err(_error) => Err(())
        }
    }

    async fn put(&mut self, key: &str, value: &str) -> Result<(), ()> {
        // TODO: properly handle errors.
        if let Err(_error) = self.db.insert(key, value) {
            return Err(());
        }

        if let Err(_error) = self.db.flush_async().await {
            return Err(());
        }

        Ok(())
    }

    async fn clear(&mut self) -> Result<(), ()> {
        // TODO: properly handle errors.
        if let Err(_error) = self.db.clear() {
            return Err(());
        }

        if let Err(_error) = self.db.flush_async().await {
            return Err(());
        }

        Ok(())
    }
}

impl SledStore {
    fn new(name: &str) -> Self {
        // TODO: properly handle errors.
        let db = sled::open(name).unwrap();
        Self { db: db }
    }
}

#[derive(Debug)]
struct NativeStore {
    store: SledStore,
}

#[node_bindgen]
impl NativeStore {
    #[node_bindgen(constructor)]
    fn new(name: String) -> Self {
        Self { store: SledStore::new(&name) }
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
