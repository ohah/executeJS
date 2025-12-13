use crate::execution::ExecutionOutput;
use crate::node_downloader::NodeDownloader;
use anyhow::{Context, Result};
use std::path::PathBuf;
use std::process::Stdio;
use tokio::io::{AsyncReadExt, BufReader};
use tokio::process::Command;

/// JavaScript 실행기 (Node.js 기반)
pub struct NodeExecutor {
    node_path: PathBuf,
}

impl NodeExecutor {
    /// 새로운 NodeExecutor 인스턴스 생성
    pub fn new() -> Result<Self> {
        let node_path = Self::find_node_binary().map_err(|e| {
            tracing::error!("Node.js 바이너리를 찾을 수 없습니다: {}", e);
            e
        })?;
        tracing::info!("NodeExecutor 초기화 완료: {}", node_path.display());
        Ok(Self { node_path })
    }

    /// OS별 Node.js 바이너리 경로 찾기
    fn find_node_binary() -> Result<PathBuf> {
        let (os_name, arch, _extension, binary_name) = NodeDownloader::get_platform_info()?;

        // 캐시 경로에서만 찾기
        let cache_dir = NodeDownloader::cache_dir()?;
        let node_dir = cache_dir.join(format!(
            "node-{}-{}-{}",
            crate::node_downloader::NODE_VERSION,
            os_name,
            arch
        ));
        let node_path = node_dir.join(&binary_name);

        if node_path.exists() {
            tracing::debug!("캐시에서 Node.js 바이너리 발견: {}", node_path.display());
            return Self::set_permissions_if_needed(node_path);
        }

        // 에러 메시지
        let cache_path = cache_dir.join(format!(
            "node-{}-{}-{}",
            crate::node_downloader::NODE_VERSION,
            os_name,
            arch
        ));
        anyhow::bail!(
            "Node.js 바이너리를 찾을 수 없습니다.\n\
            - 캐시 경로: {}\n\
            - OS: {}, Arch: {}\n\
            - 바이너리 이름: {}\n\
            NodeExecutor::new()는 바이너리를 자동으로 다운로드하지 않습니다.\n\
            앱 시작 시 ensure_node_binary()가 호출되어 자동으로 다운로드됩니다.",
            cache_path.display(),
            std::env::consts::OS,
            std::env::consts::ARCH,
            binary_name
        );
    }

    /// Node.js 바이너리 확인 및 다운로드 (공개 메서드)
    pub async fn ensure_node_binary() -> Result<PathBuf> {
        NodeDownloader::ensure_node_binary().await
    }

    /// 실행 권한 설정 (필요한 경우)
    fn set_permissions_if_needed(node_path: PathBuf) -> Result<PathBuf> {
        // 실행 권한 확인 (Unix 계열) - 이미 실행 가능한 경우 스킵하여 파일 변경 방지
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let metadata = std::fs::metadata(&node_path).with_context(|| {
                format!(
                    "파일 메타데이터를 읽을 수 없습니다: {}",
                    node_path.display()
                )
            })?;
            let perms = metadata.permissions();
            // 이미 실행 권한이 있는 경우 스킵 (파일 변경 방지)
            if perms.mode() & 0o111 == 0 {
                // 실행 권한이 없는 경우에만 설정
                let mut new_perms = perms.clone();
                new_perms.set_mode(0o755);
                std::fs::set_permissions(&node_path, new_perms).with_context(|| {
                    format!("실행 권한을 설정할 수 없습니다: {}", node_path.display())
                })?;
            }
        }

        Ok(node_path)
    }

    /// JavaScript 코드 실행
    pub async fn execute_script(&self, _filename: &str, code: &str) -> Result<String> {
        tracing::debug!("Node.js 코드 실행 시작, 코드 길이: {} bytes", code.len());

        // 임시 디렉토리를 working directory로 설정하여 프로젝트 폴더 변경 방지
        let temp_dir = std::env::temp_dir();

        // Node.js subprocess 실행 (stdin으로 코드 전달)
        let mut child = Command::new(&self.node_path)
            .current_dir(&temp_dir) // 임시 디렉토리에서 실행하여 프로젝트 폴더 변경 방지
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .with_context(|| {
                format!(
                    "Node.js 프로세스를 시작할 수 없습니다: {}",
                    self.node_path.display()
                )
            })?;

        // stdin에 코드 쓰기
        let mut stdin = child.stdin.take().expect("stdin이 설정되지 않았습니다");
        use tokio::io::AsyncWriteExt;
        stdin
            .write_all(code.as_bytes())
            .await
            .context("stdin에 코드 쓰기 실패")?;
        drop(stdin); // stdin 닫기

        // stdout와 stderr를 비동기로 읽기
        let stdout = child.stdout.take().expect("stdout가 설정되지 않았습니다");
        let stderr = child.stderr.take().expect("stderr가 설정되지 않았습니다");

        let mut stdout_reader = BufReader::new(stdout);
        let mut stderr_reader = BufReader::new(stderr);

        let mut stdout_buf = String::new();
        let mut stderr_buf = String::new();

        // stdout와 stderr를 동시에 읽기
        let (stdout_result, stderr_result) = tokio::join!(
            stdout_reader.read_to_string(&mut stdout_buf),
            stderr_reader.read_to_string(&mut stderr_buf)
        );

        stdout_result.context("stdout 읽기 실패")?;
        stderr_result.context("stderr 읽기 실패")?;

        // 프로세스 종료 대기
        let status = child.wait().await.context("프로세스 종료 대기 실패")?;

        // 출력 버퍼 생성
        let mut output = ExecutionOutput::new();
        output.stdout = stdout_buf.trim().to_string();
        output.stderr = stderr_buf.trim().to_string();

        // 프로세스가 실패한 경우 (0이 아닌 종료 코드)
        if !status.success() {
            let error_msg = if !output.stderr.is_empty() {
                output.stderr.clone()
            } else {
                format!(
                    "프로세스가 종료 코드 {}로 종료되었습니다",
                    status.code().unwrap_or(-1)
                )
            };
            return Err(anyhow::anyhow!("{}", error_msg));
        }

        let result_text = output.get_output();

        if result_text.is_empty() {
            Ok("코드가 실행되었습니다.".to_string())
        } else {
            Ok(result_text)
        }
    }
}

