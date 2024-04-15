use serde_json::Value;
use std::process::Command;

use crate::models::response::ToolCall;
use crate::errors::coderaError::CoderaError;

pub fn handle(tool_call: &ToolCall) -> Result<String, CoderaError> {
    if tool_call.function.name != "command_line" {
        Err(CoderaError::new("Tool call is not 'command_line'"))
    }
    

    let args = match from_str(&tool_call.function.args) {
        Ok(value) => value,
        Err(err) => Err(CoderaError::new(format!("Failed to parse args: {}", err))),
    };

    let command = args.get("command")
        .and_then(|v| v.as_str())
        .ok_or_else(|| CoderaError::new("Command not found in args!"))?;
    
        let output = match Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()
    {
        Ok(output) => output,
        Err(err) => Err(CoderaError::new(format!("Failed to execute process: {}", err))),
    };

    // Extract result based on success
    let result = match output.status.success() {
        true => String::from_utf8(output.stdout)?,
        false => String::from_utf8(output.stderr)?,
    };

}
