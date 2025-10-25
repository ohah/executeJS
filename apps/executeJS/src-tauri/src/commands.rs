use crate::js_executor::{execute_javascript_code, JsExecutionResult, JsExecutorState};
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
pub fn execute_js(code: &str, state: State<JsExecutorState>) -> Result<JsExecutionResult, String> {
    let result = execute_javascript_code(code);

    // 실행 결과를 히스토리에 추가
    state.add_execution(result.clone());

    if result.success {
        Ok(result)
    } else {
        Err(result
            .error
            .unwrap_or_else(|| "알 수 없는 오류".to_string()))
    }
}

#[tauri::command]
pub fn get_js_execution_history(state: State<JsExecutorState>) -> Vec<JsExecutionResult> {
    state.get_history()
}

#[tauri::command]
pub fn clear_js_execution_history(state: State<JsExecutorState>) -> Result<(), String> {
    state.clear_history();
    Ok(())
}

#[tauri::command]
pub fn save_js_code(code: &str, filename: &str) -> Result<String, String> {
    use std::fs;
    use std::path::Path;

    let code_dir = Path::new("saved_codes");
    if !code_dir.exists() {
        fs::create_dir_all(code_dir).map_err(|e| format!("디렉토리 생성 실패: {}", e))?;
    }

    let file_path = code_dir.join(filename);
    fs::write(&file_path, code).map_err(|e| format!("파일 저장 실패: {}", e))?;

    Ok(format!("코드가 {}에 저장되었습니다", file_path.display()))
}

#[tauri::command]
pub fn load_js_code(filename: &str) -> Result<String, String> {
    use std::fs;
    use std::path::Path;

    let file_path = Path::new("saved_codes").join(filename);

    if !file_path.exists() {
        return Err("파일을 찾을 수 없습니다".to_string());
    }

    fs::read_to_string(&file_path).map_err(|e| format!("파일 읽기 실패: {}", e))
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
