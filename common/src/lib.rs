use async_trait::async_trait;

#[async_trait]
pub trait Store {
    fn new(name: &str) -> Self;

    async fn get(&self, key: &str) -> Result<Option<String>, ()>;
    async fn put(&mut self, key: &str, value: &str) -> Result<(), ()>;

    fn clear(&mut self);
}
