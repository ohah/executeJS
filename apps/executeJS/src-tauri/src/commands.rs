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

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
pub async fn execute_js(code: &str) -> Result<JsExecutionResult, String> {
    let result = execute_javascript_code(code).await;

    if result.success {
        Ok(result)
    } else {
        Err(result
            .error
            .unwrap_or_else(|| "알 수 없는 오류".to_string()))
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
