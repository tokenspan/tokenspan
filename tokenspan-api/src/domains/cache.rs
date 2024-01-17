use async_trait::async_trait;

#[async_trait]
pub trait CacheExt<TKey, TValue> {
    async fn set(&self, key: TKey, value: TValue);
    async fn get(&self, key: TKey) -> Option<TValue>;
}
