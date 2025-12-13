use node_runtime::NodeExecutor;
use std::sync::Mutex;

// 테스트 간 격리를 위한 락
static TEST_LOCK: Mutex<()> = Mutex::new(());

#[tokio::test]
async fn test_console_log() {
    let _lock = TEST_LOCK.lock().unwrap();
    let executor = NodeExecutor::new().unwrap();
    let result = executor
        .execute_script("test.js", "console.log('Hello World');")
        .await;
    assert!(result.is_ok());
    let output = result.unwrap();
    println!("실제 출력: '{}'", output);
    assert!(output.contains("Hello World"));
}

#[tokio::test]
async fn test_variable_assignment() {
    let _lock = TEST_LOCK.lock().unwrap();
    let executor = NodeExecutor::new().unwrap();
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
    let executor = NodeExecutor::new().unwrap();
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
    let executor = NodeExecutor::new().unwrap();
    let result = executor.execute_script("test.js", "alert('adf'(;").await;
    // 문법 오류는 실행 실패를 반환해야 함
    assert!(result.is_err());
}

#[tokio::test]
async fn test_multiple_statements() {
    let _lock = TEST_LOCK.lock().unwrap();
    let executor = NodeExecutor::new().unwrap();
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
async fn test_multiple_console_logs() {
    let _lock = TEST_LOCK.lock().unwrap();
    let executor = NodeExecutor::new().unwrap();
    let result = executor
        .execute_script(
            "test.js",
            "console.log('First'); console.log('Second'); console.log('Third');",
        )
        .await;
    assert!(result.is_ok());
    let output = result.unwrap();
    println!("실제 출력: '{}'", output);
    assert!(output.contains("First"));
    assert!(output.contains("Second"));
    assert!(output.contains("Third"));
}

#[tokio::test]
async fn test_object_logging() {
    let _lock = TEST_LOCK.lock().unwrap();
    let executor = NodeExecutor::new().unwrap();
    let result = executor
        .execute_script("test.js", "console.log({ name: 'Test', value: 42 });")
        .await;
    assert!(result.is_ok());
    let output = result.unwrap();
    println!("실제 출력: '{}'", output);
    // Node.js는 객체를 자동으로 직렬화하여 출력
    assert!(output.contains("name") || output.contains("Test"));
}
