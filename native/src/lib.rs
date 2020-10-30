use std::collections::HashMap;

use common::Store;
use async_trait::async_trait;
use napi::{CallContext, JsFunction, JsObject, JsString, JsUndefined, Module, Property, register_module};
use napi_derive::js_function;

/// Copied from wasm module -- to be replaced with sled
#[derive(Debug)]
pub struct InMemoryStore {
    entries: HashMap<String, String>,
}

#[async_trait(?Send)]
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

#[js_function(1)]
fn create_native_store_class(ctx: CallContext) -> napi::Result<JsFunction> {
    let add_get_method = Property::new(&ctx.env, "get")?.with_method(get);
    let add_put_method = Property::new(&ctx.env, "put")?.with_method(put);
    let add_clear_method = Property::new(&ctx.env, "clear")?.with_method(clear);
    let properties = vec![
        add_get_method,
        add_put_method,
        add_clear_method,
    ];
    ctx.env.define_class(
        "NativeStore",
        native_store_constructor,
        properties.as_slice(),
    )
}

#[js_function(1)]
fn native_store_constructor(ctx: CallContext<JsObject>) -> napi::Result<JsUndefined> {
    let in_string = ctx.get::<JsString>(0)?;
    let name = in_string.as_str()?;

    let mut this = ctx.this;
    ctx.env.wrap(&mut this, InMemoryStore::new(name))?;
    ctx.env.get_undefined()
}

#[js_function(1)]
fn get(ctx: CallContext<JsObject>) -> napi::Result<JsUndefined> {
    let in_key = ctx.get::<JsString>(0)?;
    let key = in_key.as_str()?;

    let in_value = ctx.get::<JsString>(1)?;
    let value = in_value.as_str()?; 

    let this: JsObject = ctx.this;
    let store: &mut InMemoryStore = ctx.env.unwrap(&this)?;
    todo!();
    // ctx.env.get_undefined()
}

#[js_function(1)]
fn put(ctx: CallContext<JsObject>) -> napi::Result<JsUndefined> {
    let this: JsObject = ctx.this;
    let store: &mut InMemoryStore = ctx.env.unwrap(&this)?;
    store.clear();
    ctx.env.get_undefined()
}

#[js_function(1)]
fn clear(ctx: CallContext<JsObject>) -> napi::Result<JsUndefined> {
    let this: JsObject = ctx.this;
    let store: &mut InMemoryStore = ctx.env.unwrap(&this)?;
    store.clear();
    ctx.env.get_undefined()
}

fn init(module: &mut Module) -> napi::Result<()> {
    module.create_named_method("createNativeStoreClass", create_native_store_class)?;
    Ok(())
}

register_module!(test_module, init);