use crate::js_executor::{execute_javascript_code, JsExecutionResult};
use serde::{Deserialize, Serialize};
use std::process::Command;

#[derive(Debug, Serialize, Deserialize)]
pub struct AppInfo {
    pub name: String,
    pub version: String,
    pub description: String,
    pub author: String,
}

// 린트 결과를 나타내는 구조체
#[derive(Debug, Serialize, Deserialize)]
pub struct LintResult {
    pub line: usize,
    pub column: usize,
    pub end_line: usize,
    pub end_column: usize,
    pub message: String,
    // "error" 또는 "warning"
    pub severity: String,
    pub rule_id: String,
}

// oxlint JSON 출력 형식에 맞는 구조체
#[derive(Debug, Deserialize)]
struct OxlintOutput {
    diagnostics: Vec<OxlintDiagnostic>,
}

// oxlint 진단 정보
#[derive(Debug, Deserialize)]
struct OxlintDiagnostic {
    message: String,
    code: String,
    severity: String,
    labels: Vec<OxlintLabel>,
}

// oxlint 레이블 (위치 정보 포함)
#[derive(Debug, Deserialize)]
struct OxlintLabel {
    span: OxlintSpan,
}

// oxlint 스팬 (라인, 컬럼, 길이 정보)
#[derive(Debug, Deserialize)]
struct OxlintSpan {
    line: usize,
    column: usize,
    #[serde(default)]
    length: usize,
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

// JavaScript 코드를 oxlint로 린트하고 결과를 반환
#[tauri::command]
pub async fn lint_code(code: String) -> Result<Vec<LintResult>, String> {
    use std::io::Write;
    use tempfile::NamedTempFile;

    // 임시 파일 생성
    let mut temp_file = NamedTempFile::with_suffix(".js")
        .map_err(|e| format!("Failed to create temp file: {}", e))?;

    temp_file
        .write_all(code.as_bytes())
        .map_err(|e| format!("Failed to write to temp file: {}", e))?;

    let temp_path = temp_file.path().to_str().ok_or("Invalid temp path")?;

    // 프로젝트 루트 경로 찾기 (현재 실행 파일 위치 기준)
    let current_exe = std::env::current_exe()
        .map_err(|e| format!("Failed to get current exe path: {}", e))?;

    // Tauri 앱의 경우: target/debug/executeJS 또는 target/release/executeJS
    // 프로젝트 루트는 3단계 위 (target/debug 또는 target/release)
    let mut project_root = current_exe
        .parent() // target/debug 또는 target/release
        .and_then(|p| p.parent()) // target
        .and_then(|p| p.parent()) // 프로젝트 루트
        .ok_or("Failed to find project root")?;

    // 개발 모드에서는 src-tauri가 있으므로 한 단계 더 올라가야 함
    let src_tauri_path = project_root.join("apps/executeJS/src-tauri");
    if src_tauri_path.exists() {
        project_root = src_tauri_path
            .parent()
            .and_then(|p| p.parent())
            .ok_or("Failed to find project root")?;
    }

    // oxlint 경로: 프로젝트 루트의 node_modules/.bin/oxlint
    let oxlint_path = project_root
        .join("node_modules")
        .join(".bin")
        .join("oxlint");

    // oxlint 실행 (로컬 설치된 버전 사용)
    let output = if oxlint_path.exists() {
        // 로컬 설치된 oxlint 사용
        Command::new(oxlint_path)
            .arg("--format")
            .arg("json")
            .arg(temp_path)
            .output()
            .map_err(|e| format!("Failed to execute oxlint: {}", e))?
    } else {
        // fallback: pnpm exec oxlint (pnpm이 설치되어 있다면)
        Command::new("pnpm")
            .arg("exec")
            .arg("oxlint")
            .arg("--format")
            .arg("json")
            .arg(temp_path)
            .current_dir(project_root)
            .output()
            .map_err(|e| format!("Failed to execute oxlint: {}", e))?
    };

    drop(temp_file);

    // oxlint 출력 파싱
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    let results = parse_oxlint_output(&stdout, &stderr);

    Ok(results)
}

// oxlint의 JSON 출력을 파싱하여 LintResult 벡터로 변환
fn parse_oxlint_output(stdout: &str, stderr: &str) -> Vec<LintResult> {
    let output_text = if stdout.trim().is_empty() {
        stderr
    } else {
        stdout
    };

    // JSON 형식으로 직접 deserialize
    match serde_json::from_str::<OxlintOutput>(output_text) {
        Ok(oxlint_output) => {
            let mut results = Vec::new();

            for diagnostic in oxlint_output.diagnostics {
                // labels의 첫 번째 항목에서 위치 정보 가져오기
                if let Some(label) = diagnostic.labels.first() {
                    let span = &label.span;
                    let line = span.line;
                    let column = span.column.max(1);
                    let end_column = if span.length > 0 {
                        column + span.length
                    } else {
                        // length가 없을 경우 기본값 사용
                        column + 10
                    };

                    // code에서 rule_id 추출
                    // 예: "eslint(no-unused-vars)" -> "no-unused-vars"
                    let rule_id = if diagnostic.code.starts_with("eslint(") {
                        diagnostic
                            .code
                            .strip_prefix("eslint(")
                            .and_then(|s| s.strip_suffix(')'))
                            .unwrap_or("unknown")
                            .to_string()
                    } else {
                        diagnostic.code.clone()
                    };

                    results.push(LintResult {
                        line,
                        column,
                        end_line: line,
                        end_column,
                        message: diagnostic.message,
                        severity: diagnostic.severity,
                        rule_id,
                    });
                }
            }

            results
        }
        Err(_) => Vec::new(),
    }
}
