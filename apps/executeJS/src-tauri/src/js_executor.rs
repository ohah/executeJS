use node_runtime::NodeExecutor;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsExecutionResult {
    pub code: String,
    pub result: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub success: bool,
    pub error: Option<String>,
}

pub async fn execute_javascript_code(code: &str) -> JsExecutionResult {
    let timestamp = chrono::Utc::now();

    // 빈 코드 체크
    if code.trim().is_empty() {
        return JsExecutionResult {
            code: code.to_string(),
            result: String::new(),
            timestamp,
            success: false,
            error: Some("코드가 비어있습니다".to_string()),
        };
    }

    // NodeExecutor를 사용한 실제 JavaScript 실행
    match execute_with_node(code).await {
        Ok(output) => JsExecutionResult {
            code: code.to_string(),
            result: output,
            timestamp,
            success: true,
            error: None,
        },
        Err(error) => JsExecutionResult {
            code: code.to_string(),
            result: String::new(),
            timestamp,
            success: false,
            error: Some(format!("{}", error)),
        },
    }
}

/// Node.js를 사용한 JavaScript 코드 실행
async fn execute_with_node(code: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    // NodeExecutor 생성
    let executor = NodeExecutor::new().map_err(|e| format!("{}", e))?;

    // 코드 실행
    let result = executor
        .execute_script("index.js", code)
        .await
        .map_err(|e| format!("{}", e))?;

    Ok(result)
}
