mod node_downloader;

use anyhow::{Context, Result};
use node_downloader::NodeDownloader;
use std::path::{Path, PathBuf};
use std::process::Stdio;
use tokio::io::{AsyncReadExt, BufReader};
use tokio::process::Command;

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

        // 1. 캐시 경로에서 찾기 (우선)
        let cache_dir = NodeDownloader::cache_dir()?;
        let node_dir = cache_dir.join(format!(
            "node-{}-{}-{}",
            node_downloader::NODE_VERSION,
            os_name,
            arch
        ));
        let node_path = node_dir.join(&binary_name);

        if node_path.exists() {
            tracing::debug!("캐시에서 Node.js 바이너리 발견: {}", node_path.display());
            return Self::set_permissions_if_needed(node_path);
        }

        // 2. 개발 모드: src-tauri/resources/ 폴더에서 찾기 (폴백)
        // CARGO_MANIFEST_DIR에서 src-tauri로 이동 (crates/node-runtime -> 프로젝트 루트 -> apps/executeJS/src-tauri)
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        let crate_root = Path::new(manifest_dir);
        // crates/node-runtime -> 프로젝트 루트 -> apps/executeJS/src-tauri
        let src_tauri_dir = crate_root
            .parent()
            .and_then(|p| p.parent())
            .map(|p| p.join("apps").join("executeJS").join("src-tauri"));

        if let Some(ref tauri_dir) = src_tauri_dir {
            let resources_node_dir =
                tauri_dir
                    .join("resources")
                    .join("node-runtime")
                    .join(format!(
                        "node-{}-{}-{}",
                        node_downloader::NODE_VERSION,
                        os_name,
                        arch
                    ));
            let resources_node_path = resources_node_dir.join(&binary_name);

            if resources_node_path.exists() {
                tracing::debug!(
                    "개발 모드: Node.js 바이너리 경로: {}",
                    resources_node_path.display()
                );
                return Self::set_permissions_if_needed(resources_node_path);
            }
        }

        // 3. 프로덕션 모드: 실행 파일 위치 기준으로 리소스 찾기 (폴백, 거의 사용되지 않음)
        // 캐시 경로가 우선이므로 이 경로는 거의 사용되지 않음
        if let Ok(exe_path) = std::env::current_exe() {
            // macOS .app 번들 구조: .app/Contents/MacOS/executeJS -> .app/Contents/Resources/
            #[cfg(target_os = "macos")]
            {
                // .app/Contents/Resources/ 경로 확인
                if let Some(macos_dir) = exe_path.parent() {
                    eprintln!("[NodeExecutor] MacOS 디렉토리: {}", macos_dir.display());

                    // MacOS 디렉토리에서 Contents로 이동
                    if macos_dir.ends_with("MacOS")
                        || macos_dir.file_name().and_then(|n| n.to_str()) == Some("MacOS")
                    {
                        if let Some(contents_dir) = macos_dir.parent() {
                            eprintln!(
                                "[NodeExecutor] Contents 디렉토리: {}",
                                contents_dir.display()
                            );
                            let resources_dir = contents_dir.join("Resources");
                            eprintln!(
                                "[NodeExecutor] Resources 디렉토리 확인: {}",
                                resources_dir.display()
                            );

                            // Tauri가 리소스를 포함할 때의 경로 구조 확인
                            // tauri.conf.json 설정: "resources/node-runtime/": "node-runtime/"
                            // 따라서 Resources/node-runtime/... 경로에 있음

                            // 1. node-runtime/node-v24.12.0-*/node (우선 확인 - tauri.conf.json 설정에 따라)
                            let resource_path = resources_dir
                                .join("node-runtime")
                                .join(format!("node-v24.12.0-{}-{}", os_name, arch))
                                .join(&binary_name);
                            eprintln!(
                                "[NodeExecutor] 경로 1 확인 (우선): {}",
                                resource_path.display()
                            );
                            if resource_path.exists() {
                                eprintln!("[NodeExecutor] ✅ 경로 1에서 발견!");
                                return Self::set_permissions_if_needed(resource_path);
                            }

                            // 2. resources/node-runtime/node-v24.12.0-*/node (다른 구조)
                            let resource_path2 = resources_dir
                                .join("resources")
                                .join("node-runtime")
                                .join(format!("node-v24.12.0-{}-{}", os_name, arch))
                                .join(&binary_name);
                            eprintln!("[NodeExecutor] 경로 2 확인: {}", resource_path2.display());
                            if resource_path2.exists() {
                                eprintln!("[NodeExecutor] ✅ 경로 2에서 발견!");
                                return Self::set_permissions_if_needed(resource_path2);
                            }

                            // 3. _up_/_up_/_up_/resources/node-runtime/... (이전 설정 호환성)
                            let tauri_resource_path = resources_dir
                                .join("_up_")
                                .join("_up_")
                                .join("_up_")
                                .join("resources")
                                .join("node-runtime")
                                .join(format!("node-v24.12.0-{}-{}", os_name, arch))
                                .join(&binary_name);
                            eprintln!(
                                "[NodeExecutor] 경로 3 확인 (이전 호환): {}",
                                tauri_resource_path.display()
                            );
                            if tauri_resource_path.exists() {
                                eprintln!("[NodeExecutor] ✅ 경로 3에서 발견!");
                                return Self::set_permissions_if_needed(tauri_resource_path);
                            }

                            // Resources 디렉토리 전체 구조 확인 (디버깅용)
                            eprintln!("[NodeExecutor] Resources 디렉토리 전체 구조:");
                            if let Ok(entries) = std::fs::read_dir(&resources_dir) {
                                for entry in entries.flatten() {
                                    let path = entry.path();
                                    if path.is_dir() {
                                        eprintln!("  [DIR] {}", path.display());
                                        // 하위 디렉토리도 확인
                                        if let Ok(sub_entries) = std::fs::read_dir(&path) {
                                            for sub_entry in sub_entries.flatten() {
                                                eprintln!("    - {}", sub_entry.path().display());
                                            }
                                        }
                                    } else {
                                        eprintln!("  [FILE] {}", path.display());
                                    }
                                }
                            }

                            // 4. 직접 Resources에 있는 경우
                            let direct_resource_path = resources_dir.join(&binary_name);
                            eprintln!(
                                "[NodeExecutor] 경로 4 확인: {}",
                                direct_resource_path.display()
                            );
                            if direct_resource_path.exists() {
                                eprintln!("[NodeExecutor] ✅ 경로 4에서 발견!");
                                return Self::set_permissions_if_needed(direct_resource_path);
                            }

                            // Resources 디렉토리 내용 확인 (디버깅용)
                            if let Ok(entries) = std::fs::read_dir(&resources_dir) {
                                eprintln!("[NodeExecutor] Resources 디렉토리 내용:");
                                for entry in entries.flatten() {
                                    eprintln!("  - {}", entry.path().display());
                                }
                            }
                        }
                    }
                }
            }

            // 실행 파일의 부모 디렉토리들에서 resources 폴더 찾기
            let mut search_path = exe_path.parent();
            for depth in 0..10 {
                if let Some(path) = search_path {
                    // 일반 resources 폴더
                    let resource_path = path
                        .join("resources")
                        .join("node-runtime")
                        .join(format!("node-v24.12.0-{}-{}", os_name, arch))
                        .join(&binary_name);
                    if resource_path.exists() {
                        tracing::info!(
                            "프로덕션 모드 (depth {}): Node.js 바이너리 경로: {}",
                            depth,
                            resource_path.display()
                        );
                        return Self::set_permissions_if_needed(resource_path);
                    }

                    // 리소스가 직접 있는 경우 (폴더 구조 없이)
                    let direct_resource_path = path
                        .join("node-runtime")
                        .join(format!("node-v24.12.0-{}-{}", os_name, arch))
                        .join(&binary_name);
                    if direct_resource_path.exists() {
                        tracing::info!(
                            "프로덕션 모드 (직접, depth {}): Node.js 바이너리 경로: {}",
                            depth,
                            direct_resource_path.display()
                        );
                        return Self::set_permissions_if_needed(direct_resource_path);
                    }

                    // Windows/Linux: 실행 파일과 같은 디렉토리
                    let same_dir_path = path.join(&binary_name);
                    if same_dir_path.exists() && path != exe_path.parent().unwrap() {
                        // 실행 파일과 같은 디렉토리가 아닌 경우만 (이미 확인했으므로)
                        // 이건 실제로는 필요 없을 수 있음
                    }

                    search_path = path.parent();
                } else {
                    break;
                }
            }
        }

        // 에러 메시지
        let cache_path = cache_dir.join(format!(
            "node-{}-{}-{}",
            node_downloader::NODE_VERSION,
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

#[cfg(test)]
mod tests {
    use super::*;
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
}
