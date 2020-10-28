use async_trait::async_trait;

#[async_trait(?Send)]
pub trait Store {
    // We are using type erasure (dyn Store) in `JSStore` to retain a
    // handle to 'any Store'. Therefore, we apparently can't have a constructor
    // as part of the trait (size can't be known at compile time). Not sure if
    // there is a way to square this circle, until then, commented out:
    // fn new(name: &str) -> Self;

    async fn get(&self, key: &str) -> Result<Option<String>, ()>;
    async fn put(&mut self, key: &str, value: &str) -> Result<(), ()>;

    async fn clear(&mut self) -> Result<(), ()>;
}
