use async_openai::config::OpenAIConfig;
use async_openai::types::CreateCompletionRequestArgs;
use async_openai::Client;
use futures::StreamExt;

mod types;

pub struct OpenAI {
    client: Client<OpenAIConfig>,
}

impl OpenAI {
    pub fn new(api_key: String) -> Self {
        let config = OpenAIConfig::new().with_api_key(api_key);
        let client = Client::with_config(config);
        Self { client }
    }

    pub async fn chat_completion(&self) -> anyhow::Result<()> {
        let request = CreateCompletionRequestArgs::default()
            .model("text-davinci-003")
            .n(1)
            .prompt("Tell me a bedtime story about Optimus Prime and Bumblebee")
            .stream(true)
            .max_tokens(1024_u16)
            .build()?;

        let mut stream = self.client.completions().create_stream(request).await?;

        while let Some(response) = stream.next().await {
            match response {
                Ok(response) => {
                    response.choices.iter().for_each(|choice| {
                        println!("Text: {}", choice.text);
                    });
                }
                Err(err) => {
                    println!("Error: {:?}", err);
                }
            }
        }

        Ok(())
    }
}
