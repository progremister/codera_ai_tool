use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize)]
pub struct Request {
    pub model: String,
    pub messages: Vec<Message>,
    pub tools: Vec<Tool>,
    pub tool_choice: ToolChoice
}

#[derive(Clone, Serialize, Deserialize)]
struct Message {
    role: String,
    content: String
}

#[derive(Serialize, Deserialize)]
pub struct Tool {
    #[serde(rename = "type")]
    pub tool_type: String,
    pub functin: ToolFunction
}

#[derive(Serialize, Deserialize)]
pub struct ToolFunction {
    pub name: String,
    pub description: String,
    pub parameters: Parameters
}

#[derive(Serialize, Deserialize)]
pub struct ToolChoice {
    #[serde(rename = "type")]
    pub tool_type: String,
    pub function: Function
}

#[derive(Serialize, Deserialize)]
pub struct Parameters {
    #[serde(rename = "type")]
    parameter_type: String,
    properties: Value,
    required: Vec<String>
}

#[derive(Serialize, Deserialize)]
pub struct Function {
    pub name: String
}