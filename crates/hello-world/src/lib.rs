/// Hello World 크레이트
///
/// 이 크레이트는 ExecuteJS 모노레포의 예제 크레이트입니다.
/// 사용자에게 인사를 전하는 함수
pub fn say_hello(name: &str) -> String {
    format!("Hello, {}! Welcome to ExecuteJS!", name)
}

/// 두 숫자를 더하는 함수
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

/// JavaScript 코드를 실행한다고 가정하는 함수 (실제 구현은 아님)
pub fn execute_javascript(code: &str) -> Result<String, String> {
    if code.is_empty() {
        return Err("코드가 비어있습니다".to_string());
    }
    Ok(format!("실행된 코드: {}", code))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_say_hello() {
        let result = say_hello("ExecuteJS");
        assert_eq!(result, "Hello, ExecuteJS! Welcome to ExecuteJS!");
    }

    #[test]
    fn test_add() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_execute_javascript() {
        let result = execute_javascript("console.log('hello')");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "실행된 코드: console.log('hello')");
    }

    #[test]
    fn test_execute_empty_javascript() {
        let result = execute_javascript("");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "코드가 비어있습니다");
    }
}
