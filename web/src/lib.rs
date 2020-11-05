use async_trait::async_trait;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use wasm_bindgen_futures::*;

use common::Store;

use std::cell::RefCell;
use std::rc::Rc;

use futures::select;
use futures::channel::oneshot;
use futures::future::FutureExt;
use futures::pin_mut;

use js_sys::Promise;

use web_sys::Event;
use web_sys::IdbRequest;
use web_sys::IdbDatabase;
use web_sys::IdbTransactionMode;
use web_sys::IdbOpenDbRequest;
use web_sys::IdbObjectStore;

use log::Level;
use log::info;

#[derive(Debug)]
pub struct IndexedDBStore {
    name: String
}

#[async_trait(?Send)]
impl Store for IndexedDBStore {
    async fn get(&self, key: &str) -> Result<Option<String>, ()> {
        // TODO: properly handle and report errors.
        let key_js = JsValue::from(key);

        let value_js = self.transaction(IdbTransactionMode::Readonly, move |object_store| {
            object_store.get(&key_js).unwrap()
        }).await.unwrap();

        if value_js.is_undefined() {
            return Ok(None);
        }

        if value_js.is_string() {
            let value = value_js.as_string().unwrap();
            return Ok(Some(value));
        }

        panic!("Expected string or undefined but got: {:?}", value_js);
    }

    async fn put(&mut self, key: &str, value: &str) -> Result<(), ()> {
        let key_js = JsValue::from(key);
        let value_js = JsValue::from(value);

        let _result_js = self.transaction(IdbTransactionMode::Readwrite, move |object_store| {
            object_store.put_with_key(&value_js, &key_js).unwrap()
        }).await.unwrap();

        Ok(())
    }

    async fn clear(&mut self) -> Result<(), ()> {
        let _result_js = self.transaction(IdbTransactionMode::Readwrite, move |object_store| {
            object_store.clear().unwrap()
        }).await.unwrap();

        Ok(())
    }
}

impl IndexedDBStore {
    async fn new(name: &str) -> Self {
        // TODO: handle errors properly, don't just unwrap/expect.
        let db = Self::open_db(name).await.unwrap();
        db.close();
        Self { name: name.to_string() }
    }

    async fn open_db(name: &str) -> Result<IdbDatabase, ()> {
        let window = web_sys::window().expect("No global `window` exists.");
        let indexed_db = window.indexed_db().unwrap().expect("Should have `indexedDB` on `window`, running in an outdated browser?");
        let open_db_request = indexed_db.open_with_u32(name, 1).unwrap();

        let on_upgrade_needed_closure: Closure<dyn FnMut(_)> = Closure::once(move |event: Event| {
            let open_db_request_js = JsValue::from(event.current_target());
            let open_db_request = IdbOpenDbRequest::from(open_db_request_js);
            let db_js = open_db_request.result().unwrap();
            let db = IdbDatabase::from(db_js);
            db.create_object_store("entries").unwrap();
        });

        let (on_error_sender, on_error_receiver) = oneshot::channel::<Result<(), ()>>();
        let (on_success_sender, on_success_receiver) = oneshot::channel::<Result<(), ()>>();

        let on_error_closure: Closure<dyn FnMut(_)> = Closure::once(move |_event: Event| {
            // TODO: extract proper error object.
            on_error_sender.send(Err(())).unwrap();
        });

        let on_success_closure: Closure<dyn FnMut(_)> = Closure::once(move |_event: Event| {
            on_success_sender.send(Ok(())).unwrap();
        });

        open_db_request.set_onupgradeneeded(on_upgrade_needed_closure.as_ref().dyn_ref());
        open_db_request.set_onerror(on_error_closure.as_ref().dyn_ref());
        open_db_request.set_onsuccess(on_success_closure.as_ref().dyn_ref());

        let on_error_receiver_fused = on_error_receiver.fuse();
        let on_success_receiver_fused = on_success_receiver.fuse();
        pin_mut!(on_error_receiver_fused, on_success_receiver_fused);

        let result: Result<(), ()> = select! {
            // TODO: pass proper error.
            _ = on_error_receiver_fused => Err(()),
            _ = on_success_receiver_fused => Ok(()),
        };

        result.expect("An error occured while opening indexed DB.");

        open_db_request.set_onupgradeneeded(None);
        open_db_request.set_onerror(None);
        open_db_request.set_onsuccess(None);

        let db_js = open_db_request.result().unwrap();
        let db = IdbDatabase::from(db_js);

        Ok(db)
    }

    async fn transaction<B>(&self, mode: IdbTransactionMode, block: B) -> Result<JsValue, ()> where B: Fn(IdbObjectStore) -> IdbRequest {
        let db = IndexedDBStore::open_db(&self.name).await.unwrap();
        let transaction = db.transaction_with_str_and_mode("entries", mode).unwrap();
        let object_store = transaction.object_store("entries").unwrap();
        let request = block(object_store);

        let (on_error_sender, on_error_receiver) = oneshot::channel::<Result<(), ()>>();
        let (on_success_sender, on_success_receiver) = oneshot::channel::<Result<(), ()>>();

        let on_error_closure: Closure<dyn FnMut(_)> = Closure::once(move |_event: Event| {
            info!("Error occured while trying to get value for key.");
            // TODO: extract proper error object.
            on_error_sender.send(Err(())).unwrap();
        });

        let on_success_closure: Closure<dyn FnMut(_)> = Closure::once(move |_event: Event| {
            on_success_sender.send(Ok(())).unwrap();
        });

        request.set_onerror(on_error_closure.as_ref().dyn_ref());
        request.set_onsuccess(on_success_closure.as_ref().dyn_ref());

        let on_error_receiver_fused = on_error_receiver.fuse();
        let on_success_receiver_fused = on_success_receiver.fuse();

        pin_mut!(on_error_receiver_fused, on_success_receiver_fused);

        let result: Result<(), ()> = select! {
            // TODO: pass proper error.
            _ = on_error_receiver_fused => Err(()),
            _ = on_success_receiver_fused => Ok(()),
        };

        result.expect("An error occured while trying to get value for key.");
        db.close();

        request.set_onerror(None);
        request.set_onsuccess(None);

        let js_value = request.result().unwrap();
        Ok(js_value)
    }
}

#[wasm_bindgen]
pub struct JSStore {
    store: Rc<RefCell<IndexedDBStore>>,
}

#[wasm_bindgen]
impl JSStore {

    #[wasm_bindgen(constructor)]
    pub fn new(name: &str) -> Promise {
        console_log::init_with_level(Level::Debug).unwrap();
        info!("JSStore infrastructure initialized.");

        let name = name.to_string();
        let future = async move {
            let store = IndexedDBStore::new(&name).await;
            let store_ref_celled = RefCell::new(store);
            let store_reference_counted = Rc::new(store_ref_celled);
            Ok(JsValue::from(Self { store: store_reference_counted }))
        };
        
        future_to_promise(future)
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
    pub fn clear(&self) -> Promise {
        let store = self.store.clone();
        
        let future = async move {
            let mut store = store.borrow_mut();
            let result = store.clear().await;
            match result {
                Ok(_value) => Ok(JsValue::undefined()),
                Err(_error) => Err(JsValue::undefined())
            }
        };

        future_to_promise(future)
    }
}
