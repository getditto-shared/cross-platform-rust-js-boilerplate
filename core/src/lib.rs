use async_trait::async_trait;

#[async_trait]
pub trait Store {
    fn new(name: &str) -> Self;
    
    async fn get(&self, key: &str) -> Result<String, ()>;
    async fn put(&self, key: &str, value: &str) -> Result<(), ()>;
    
    fn clear(&self);
}


