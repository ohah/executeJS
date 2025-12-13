use crate::execution::ExecutionOutput;
use crate::node_downloader::NodeDownloader;
use crate::npm_manager::NpmManager;
use anyhow::{Context, Result};
use std::path::PathBuf;
use std::process::Stdio;
use tempfile::NamedTempFile;
use tokio::io::{AsyncReadExt, AsyncWriteExt, BufReader};
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

        // macOS/Linux: bin/node, Windows: node.exe
        let node_path = if os_name == "win" {
            node_dir.join(&binary_name)
        } else {
            node_dir.join("bin").join(&binary_name)
        };

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
            - 예상 경로: {}\n\
            NodeExecutor::new()는 바이너리를 자동으로 다운로드하지 않습니다.\n\
            앱 시작 시 ensure_node_binary()가 호출되어 자동으로 다운로드됩니다.",
            cache_path.display(),
            std::env::consts::OS,
            std::env::consts::ARCH,
            binary_name,
            node_path.display()
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

        // 1. 패키지 파싱 및 설치
        let required_packages = NpmManager::parse_required_packages(code).unwrap_or_else(|e| {
            tracing::warn!("패키지 파싱 실패: {}, 계속 진행합니다", e);
            Vec::new()
        });

        if !required_packages.is_empty() {
            tracing::info!("필요한 패키지 발견: {:?}", required_packages);
            let npm_manager = NpmManager::new(self.node_path.clone())?;
            npm_manager.install_packages(&required_packages).await?;
        }

        // 2. 임시 파일 생성 (node_modules가 있는 디렉토리에)
        let node_dir = self
            .node_path
            .parent()
            .context("Node.js 바이너리 경로가 유효하지 않습니다")?;

        // ES modules 지원을 위해 .mjs 확장자 사용
        let temp_file = NamedTempFile::with_suffix_in(".mjs", node_dir)
            .context("임시 파일 생성 실패")?
            .into_temp_path();

        let temp_file_path = temp_file.to_path_buf();
        let temp_file_name = temp_file_path
            .file_name()
            .and_then(|n| n.to_str())
            .context("임시 파일 이름을 가져올 수 없습니다")?
            .to_string();

        // 코드를 임시 파일에 쓰기
        {
            let mut file = tokio::fs::File::create(&temp_file_path)
                .await
                .context("임시 파일 쓰기 실패")?;
            file.write_all(code.as_bytes())
                .await
                .context("코드 쓰기 실패")?;
        }

        // 3. Node.js로 임시 파일 실행
        let mut child = Command::new(&self.node_path)
            .arg(&temp_file_name)
            .current_dir(node_dir)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .with_context(|| {
                format!(
                    "Node.js 프로세스를 시작할 수 없습니다: {}",
                    self.node_path.display()
                )
            })?;

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

        // 4. 임시 파일 삭제
        drop(temp_file);

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
