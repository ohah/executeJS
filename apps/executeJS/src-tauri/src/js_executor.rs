use crate::deno_runtime::DenoExecutor;
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

    // DenoExecutor를 사용한 실제 JavaScript 실행
    match execute_with_deno(code).await {
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
            error: Some(format!("JavaScript 실행 오류: {}", error)),
        },
    }
}

/// Deno를 사용한 JavaScript 코드 실행
async fn execute_with_deno(code: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    // DenoExecutor 생성
    let mut executor = DenoExecutor::new()
        .await
        .map_err(|e| format!("Deno 런타임 초기화 실패: {}", e))?;

    // 코드 실행
    let result = executor
        .execute_script("index.js", code)
        .await
        .map_err(|e| format!("JavaScript 실행 실패: {}", e))?;

    Ok(result)
}
