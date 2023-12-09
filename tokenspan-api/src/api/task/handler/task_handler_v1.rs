use std::convert::Infallible;

use async_openai::config::OpenAIConfig;
use async_openai::types::{ChatCompletionRequestUserMessageArgs, CreateChatCompletionRequestArgs};
use async_openai::Client;
use axum::response::sse::EventExt;
use axum::response::Sse;
use bytes::{Bytes, BytesMut};
use futures::Stream;
use futures_util::StreamExt;

use crate::api::services::TaskServiceDyn;
use crate::api::task::dto::TaskExecuteInput;

/// Server-sent event
#[derive(Debug, Default, Clone)]
#[must_use]
pub struct TextEvent {
    buffer: BytesMut,
}

impl TextEvent {
    fn set(&mut self, value: impl AsRef<[u8]>) {
        let value = value.as_ref();
        self.buffer.extend_from_slice(value);
    }
}

impl EventExt for TextEvent {
    fn data<T>(mut self, data: T) -> Self
    where
        T: AsRef<str>,
    {
        let data = data.as_ref();
        self.set(data);
        self
    }

    fn finalize(self) -> Bytes {
        self.buffer.freeze()
    }
}

pub async fn execute_task_v1(
    _task_service: TaskServiceDyn,
    input: TaskExecuteInput,
) -> Sse<impl Stream<Item = Result<TextEvent, Infallible>>, TextEvent> {
    let request = CreateChatCompletionRequestArgs::default()
        .model("gpt-3.5-turbo")
        .max_tokens(512u16)
        .messages([ChatCompletionRequestUserMessageArgs::default()
            .content(input.message)
            .build()
            .unwrap()
            .into()])
        .build()
        .unwrap();

    let api_key = std::env::var("OPENAI_API_KEY").unwrap();
    let config = OpenAIConfig::new().with_api_key(api_key);
    let client = Client::with_config(config);
    let mut stream = client.chat().create_stream(request).await.unwrap();
    let stream = async_stream::stream! {
        while let Some(response) = stream.next().await {
            match response {
                Ok(response) => {
                    for choice in response.choices {
                        if let Some(ref content) = choice.delta.content {
                            yield Ok(TextEvent::default().data(content));
                        }
                    }
                }
                Err(err) => {
                    yield Ok(TextEvent::default().data(format!("Error: {:?}", err)));
                }
            }
        }
    };

    // let stream = async_stream::stream! {
    //     let mut interval = tokio::time::interval(Duration::from_secs(1));
    //     let mut count = 0;
    //     loop {
    //         interval.tick().await;
    //         yield Ok(TextEvent::default().data("hi".to_string()));
    //         count += 1;
    //         if count == 5 {
    //             break;
    //         }
    //     }
    // };

    Sse::new(stream)
}
