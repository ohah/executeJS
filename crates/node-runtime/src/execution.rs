/// JavaScript 실행 결과를 저장하는 구조체
#[derive(Debug, Clone)]
pub struct ExecutionOutput {
    pub stdout: String,
    pub stderr: String,
}

impl ExecutionOutput {
    pub fn new() -> Self {
        Self {
            stdout: String::new(),
            stderr: String::new(),
        }
    }

    pub fn get_output(&self) -> String {
        let mut output = Vec::new();

        if !self.stdout.is_empty() {
            output.push(self.stdout.clone());
        }

        if !self.stderr.is_empty() {
            output.push(format!("[ERROR] {}", self.stderr));
        }

        output.join("\n")
    }
}

