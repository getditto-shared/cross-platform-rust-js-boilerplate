use async_trait::async_trait;

#[async_trait(?Send)]
pub trait Store {
    async fn get(&self, key: &str) -> Result<Option<String>, ()>;
    async fn put(&mut self, key: &str, value: &str) -> Result<(), ()>;
    async fn clear(&mut self) -> Result<(), ()>;
}
