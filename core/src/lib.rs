use futures::future::Future;

pub trait Store {
    fn new(name: &'static str) -> Self;
    
    fn get(&self, key: &'static str) -> Future<Item=String, Error=()>;
    fn put(&self, key: &'static str, key: &'static str) -> Future<Item=(), Error=()>;
    
    fn clear(&self);
}