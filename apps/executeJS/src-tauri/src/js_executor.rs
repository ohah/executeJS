use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::sync::Mutex;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsExecutionResult {
    pub code: String,
    pub result: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub success: bool,
    pub error: Option<String>,
}

#[derive(Debug)]
pub struct JsExecutorState {
    pub history: Mutex<VecDeque<JsExecutionResult>>,
    pub max_history: usize,
}

impl JsExecutorState {
    pub fn new(max_history: usize) -> Self {
        Self {
            history: Mutex::new(VecDeque::new()),
            max_history,
        }
    }

    pub fn add_execution(&self, result: JsExecutionResult) {
        if let Ok(mut history) = self.history.lock() {
            history.push_back(result);
            if history.len() > self.max_history {
                history.pop_front();
            }
        }
    }

    pub fn get_history(&self) -> Vec<JsExecutionResult> {
        if let Ok(history) = self.history.lock() {
            history.iter().cloned().collect()
        } else {
            vec![]
        }
    }

    pub fn clear_history(&self) {
        if let Ok(mut history) = self.history.lock() {
            history.clear();
        }
    }
}

impl Default for JsExecutorState {
    fn default() -> Self {
        Self::new(100) // 최대 100개의 실행 기록 보관
    }
}

pub fn execute_javascript_code(code: &str) -> JsExecutionResult {
    let timestamp = chrono::Utc::now();

    // 실제 JavaScript 실행 로직은 여기에 구현
    // 현재는 시뮬레이션으로 처리
    let result = if code.trim().is_empty() {
        Err("코드가 비어있습니다".to_string())
    } else if code.contains("error") || code.contains("Error") {
        Err("JavaScript 실행 중 오류가 발생했습니다".to_string())
    } else {
        Ok(format!("실행 결과: {}", code))
    };

    match result {
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
            error: Some(error),
        },
    }
}
