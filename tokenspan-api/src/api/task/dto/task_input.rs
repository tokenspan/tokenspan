use async_graphql::InputObject;
use serde::Deserialize;
use validator::Validate;

#[derive(InputObject)]
pub struct TaskCreateInput {
    pub name: String,
    pub slug: String,
    pub private: bool,
}

#[derive(InputObject)]
pub struct TaskUpdateInput {
    pub name: Option<String>,
    pub private: Option<bool>,
}

#[derive(Deserialize, Clone)]
pub enum Role {
    #[serde(rename = "user")]
    User,
    #[serde(rename = "system")]
    System,
    #[serde(rename = "assistant")]
    Assistant,
}

#[derive(Deserialize, Validate, Clone)]
pub struct Message {
    pub content: String,
    pub role: Role,
}

#[derive(Deserialize, Validate, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TaskExecuteInput {
    // pub task_version_id: TaskVersionId,
    // pub parameter_id: ParameterId,
    // pub api_key_id: ApiKeyId,
    pub messages: Vec<Message>,
}
