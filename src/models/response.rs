use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub model: String,
    pub choices: Vec<Choice>,
    pub usage: Usage,
    pub fingerprint: Option<String>
}

#[derive(Debug, Serialize, Deserialize)]
struct Choice {
    index: u32,
    message: Message,

    #[serde(skip_serializing_if = "Option::is_none")]
    logprobs: Option<Value>,
    finish_reason: String
}

#[derive(Debug, Serialize, Deserialize)]
struct Message {
    role: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    content: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    tool_calls: Option<ToolCall>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ToolCall {
    pub id: String,

    #[serde(rename = "type")]
    pub tool_type: String,
    pub function: ToolFunction
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ToolFunction {
    pub name: String,
    pub arguments: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Usage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32
}