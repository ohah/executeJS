use anyhow::Result;
use deno_core::error::AnyError;
use deno_core::{extension, op2, FsModuleLoader, JsRuntime, RuntimeOptions};
use std::collections::VecDeque;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::Mutex;

/// JavaScript 실행 결과를 저장하는 구조체
#[derive(Debug, Clone)]
pub struct ExecutionOutput {
    pub stdout: VecDeque<String>,
    pub stderr: VecDeque<String>,
}

impl ExecutionOutput {
    pub fn new() -> Self {
        Self {
            stdout: VecDeque::new(),
            stderr: VecDeque::new(),
        }
    }

    pub fn add_stdout(&mut self, message: String) {
        self.stdout.push_back(message);
    }

    pub fn add_stderr(&mut self, message: String) {
        self.stderr.push_back(message);
    }

    pub fn get_output(&self) -> String {
        let mut output = Vec::new();

        for line in &self.stdout {
            output.push(line.clone());
        }

        for line in &self.stderr {
            output.push(format!("[ERROR] {}", line));
        }

        output.join("\n")
    }
}

/// 전역 출력 버퍼 (스레드 안전)
static OUTPUT_BUFFER: Mutex<Option<Arc<Mutex<ExecutionOutput>>>> = Mutex::new(None);

/// console.log를 위한 op 함수
#[op2(fast)]
#[string]
fn op_console_log(#[string] message: String) -> Result<(), AnyError> {
    if let Ok(buffer_guard) = OUTPUT_BUFFER.lock() {
        if let Some(buffer) = buffer_guard.as_ref() {
            if let Ok(mut output) = buffer.lock() {
                output.add_stdout(message);
            }
        }
    }
    Ok(())
}

/// alert를 위한 op 함수
#[op2(fast)]
#[string]
fn op_alert(#[string] message: String) -> Result<(), AnyError> {
    if let Ok(buffer_guard) = OUTPUT_BUFFER.lock() {
        if let Some(buffer) = buffer_guard.as_ref() {
            if let Ok(mut output) = buffer.lock() {
                output.add_stdout(format!("[ALERT] {}", message));
            }
        }
    }
    Ok(())
}

/// print를 위한 op 함수 (Deno.core.print 대체)
#[op2(fast)]
#[string]
fn op_custom_print(#[string] message: String, is_err: bool) -> Result<(), AnyError> {
    if let Ok(buffer_guard) = OUTPUT_BUFFER.lock() {
        if let Some(buffer) = buffer_guard.as_ref() {
            if let Ok(mut output) = buffer.lock() {
                if is_err {
                    output.add_stderr(message);
                } else {
                    output.add_stdout(message);
                }
            }
        }
    }
    Ok(())
}

extension!(
    executejs_runtime,
    ops = [op_console_log, op_alert, op_custom_print],
);

/// JavaScript 실행기 (Deno Core 기반)
pub struct DenoExecutor {
    output_buffer: Arc<Mutex<ExecutionOutput>>,
}

impl DenoExecutor {
    /// 새로운 DenoExecutor 인스턴스 생성
    pub async fn new() -> Result<Self> {
        // 출력 버퍼 생성
        let output_buffer = Arc::new(Mutex::new(ExecutionOutput::new()));

        // 전역 버퍼에 설정
        {
            let mut global_buffer = OUTPUT_BUFFER.lock().unwrap();
            *global_buffer = Some(output_buffer.clone());
        }

        Ok(Self { output_buffer })
    }

    /// JavaScript 코드 실행
    pub async fn execute_script(&mut self, _filename: &str, code: &str) -> Result<String> {
        // 출력 버퍼 초기화
        {
            let mut output = self.output_buffer.lock().unwrap();
            *output = ExecutionOutput::new();
        }

        // 코드를 클로저로 캡처
        let code = code.to_string();
        let output_buffer = self.output_buffer.clone();

        // 별도 스레드에서 Deno Core 실행 (Send 트레이트 문제 해결)
        let result = tokio::task::spawn_blocking(move || {
            // JsRuntime 생성
            let mut js_runtime = JsRuntime::new(RuntimeOptions {
                module_loader: Some(Rc::new(FsModuleLoader)),
                extensions: vec![executejs_runtime::init_ops()],
                ..Default::default()
            });

            // bootstrap.js 실행하여 커스텀 API 설정
            let bootstrap_code = include_str!("bootstrap.js");
            if let Err(e) = js_runtime.execute_script("[executejs:bootstrap.js]", bootstrap_code) {
                return Err(anyhow::anyhow!("Bootstrap 실행 실패: {}", e));
            }

            // 사용자 코드를 래핑하여 마지막 표현식의 결과를 자동으로 출력
            // eval을 사용하여 마지막 표현식의 결과를 캡처하고 출력
            let wrapped_code = format!(
                r#"
                (function() {{
                    try {{
                        const code_output = eval(`{}`);
                        if (code_output !== undefined) {{
                            Deno.core.ops.op_console_log(String(code_output));
                        }}
                        return code_output;
                    }} catch (e) {{
                        // eval 실패 시 원본 코드를 그대로 실행
                        throw e;
                    }}
                }})();
                "#,
                code.replace('`', r"\`").replace('\\', r"\\")
            );

            // 코드 실행
            let _result = js_runtime.execute_script("[executejs:user_code]", wrapped_code)?;

            // 이벤트 루프 실행 (Promise 처리) - 블로킹 방식으로 변경
            let rt = tokio::runtime::Handle::current();
            rt.block_on(async { js_runtime.run_event_loop(Default::default()).await })?;

            // 출력 버퍼에서 결과 가져오기
            let output = output_buffer.lock().unwrap();
            let result_text = output.get_output();

            if result_text.is_empty() {
                Ok("코드가 실행되었습니다.".to_string())
            } else {
                Ok(result_text)
            }
        })
        .await
        .map_err(|e| anyhow::anyhow!("스레드 실행 실패: {}", e))?;

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;
    use tokio;

    // 테스트 간 격리를 위한 락
    static TEST_LOCK: Mutex<()> = Mutex::new(());

    #[tokio::test]
    async fn test_console_log() {
        let _lock = TEST_LOCK.lock().unwrap();
        let mut executor = DenoExecutor::new().await.unwrap();
        let result = executor
            .execute_script("test.js", "console.log('Hello World');")
            .await;
        assert!(result.is_ok());
        let output = result.unwrap();
        println!("실제 출력: '{}'", output);
        assert!(output.contains("Hello World"));
    }

    #[tokio::test]
    async fn test_alert() {
        let _lock = TEST_LOCK.lock().unwrap();
        let mut executor = DenoExecutor::new().await.unwrap();
        let result = executor
            .execute_script("test.js", "alert('Hello Alert');")
            .await;
        assert!(result.is_ok());
        let output = result.unwrap();
        println!("실제 출력: '{}'", output);
        assert!(output.contains("[ALERT] Hello Alert"));
    }

    #[tokio::test]
    async fn test_variable_assignment() {
        let _lock = TEST_LOCK.lock().unwrap();
        let mut executor = DenoExecutor::new().await.unwrap();
        let result = executor
            .execute_script("test.js", "let a = 5; console.log(a);")
            .await;
        assert!(result.is_ok());
        let output = result.unwrap();
        println!("실제 출력: '{}'", output);
        assert!(output.contains("5"));
    }

    #[tokio::test]
    async fn test_calculation() {
        let _lock = TEST_LOCK.lock().unwrap();
        let mut executor = DenoExecutor::new().await.unwrap();
        let result = executor
            .execute_script("test.js", "let a = 1; let b = 2; console.log(a + b);")
            .await;
        assert!(result.is_ok());
        let output = result.unwrap();
        println!("실제 출력: '{}'", output);
        assert!(output.contains("3"));
    }

    #[tokio::test]
    async fn test_syntax_error() {
        let _lock = TEST_LOCK.lock().unwrap();
        let mut executor = DenoExecutor::new().await.unwrap();
        let result = executor.execute_script("test.js", "alert('adf'(;").await;
        // 문법 오류는 실행 실패를 반환해야 함
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_multiple_statements() {
        let _lock = TEST_LOCK.lock().unwrap();
        let mut executor = DenoExecutor::new().await.unwrap();
        let result = executor
            .execute_script(
                "test.js",
                "let x = 5; let y = 3; console.log('result:', x + y);",
            )
            .await;
        assert!(result.is_ok());
        let output = result.unwrap();
        println!("실제 출력: '{}'", output);
        assert!(output.contains("result: 8"));
    }

    #[tokio::test]
    async fn test_lodash_import() {
        let _lock = TEST_LOCK.lock().unwrap();
        let mut executor = DenoExecutor::new().await.unwrap();
        let result = executor
            .execute_script(
                "test.js",
                r#"
                try {
                    const _ = require('lodash');
                    const numbers = [1, 2, 3, 4, 5];
                    const doubled = _.map(numbers, n => n * 2);
                    console.log('Lodash test:', doubled);
                } catch (error) {
                    console.log('Lodash not available:', error.message);
                }
                "#,
            )
            .await;
        assert!(result.is_ok());
        let output = result.unwrap();
        println!("Lodash 테스트 출력: '{}'", output);
        // lodash가 사용 가능한지 또는 오류 메시지가 나오는지 확인
        assert!(output.contains("Lodash test:") || output.contains("Lodash not available:"));
    }
}
