use async_graphql::Subscription;
use futures::Stream;
use futures_util::StreamExt;
use std::time::Duration;

#[derive(Default)]
pub struct TaskSubscription;

#[Subscription]
impl TaskSubscription {
    async fn execute(&self, #[graphql(default = 1)] condition: i32) -> impl Stream<Item = i32> {
        let mut value = 0;

        tokio_stream::wrappers::IntervalStream::new(tokio::time::interval(Duration::from_secs(1)))
            .map(move |_| {
                value += condition;
                value
            })
    }
}
