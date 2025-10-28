use crate::js_executor::{execute_javascript_code, JsExecutionResult};
use serde::{Deserialize, Serialize};
use tauri::State;

#[derive(Debug, Serialize, Deserialize)]
pub struct AppInfo {
    pub name: String,
    pub version: String,
    pub description: String,
    pub author: String,
}

#[tauri::command]
pub async fn execute_js(code: &str) -> Result<JsExecutionResult, JsExecutionResult> {
    let result = execute_javascript_code(code).await;

    if result.success {
        Ok(result)
    } else {
        let error_message = result
            .error
            .unwrap_or_else(|| "알 수 없는 오류".to_string());

        Err(JsExecutionResult {
            code: code.to_string(),
            result: error_message.clone(),
            timestamp: chrono::Utc::now(),
            success: false,
            error: Some(error_message.clone()),
        })
    }
}

#[tauri::command]
pub fn get_app_info() -> AppInfo {
    AppInfo {
        name: "ExecuteJS".to_string(),
        version: "0.1.0".to_string(),
        description: "JavaScript 코드를 실행하는 Tauri 애플리케이션".to_string(),
        author: "ExecuteJS Team".to_string(),
    }
}
