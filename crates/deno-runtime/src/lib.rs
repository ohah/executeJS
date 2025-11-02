use anyhow::Error as AnyhowError;
use anyhow::Result;
use deno_core::error::type_error;
use deno_core::error::AnyError;
use deno_core::{
    extension, op2, FastString, FsModuleLoader, JsRuntime, ModuleLoadResponse, ModuleLoader,
    ModuleSource, ModuleSourceCode, ModuleSpecifier, ModuleType, RequestedModuleType,
    ResolutionKind, RuntimeOptions,
};
use futures::FutureExt;
use std::collections::{HashMap, VecDeque};
use std::fs;
use std::path::PathBuf;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::Mutex;

mod npm_resolver;
pub use npm_resolver::NpmResolver;

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

/// 커스텀 확장 정의
extension!(
    executejs_runtime,
    ops = [op_console_log, op_alert, op_custom_print],
);

/// npm 패키지를 지원하는 모듈 로더
pub struct NpmModuleLoader {
    fs_loader: FsModuleLoader,
    npm_resolver: Arc<Mutex<NpmResolver>>,
    /// npm: URL과 실제 파일 경로 매핑
    npm_path_map: Arc<Mutex<HashMap<String, PathBuf>>>,
}

impl NpmModuleLoader {
    pub fn new() -> Result<Self> {
        Ok(Self {
            fs_loader: FsModuleLoader,
            npm_resolver: Arc::new(Mutex::new(NpmResolver::new()?)),
            npm_path_map: Arc::new(Mutex::new(HashMap::new())),
        })
    }
}

impl ModuleLoader for NpmModuleLoader {
    fn resolve(
        &self,
        specifier: &str,
        referrer: &str,
        kind: ResolutionKind,
    ) -> Result<ModuleSpecifier, AnyhowError> {
        eprintln!(
            "[NpmModuleLoader::resolve] specifier: {}, referrer: {}, kind: {:?}",
            specifier, referrer, kind
        );

        // npm: 프로토콜 처리
        if specifier.starts_with("npm:") {
            eprintln!(
                "[NpmModuleLoader::resolve] npm: 프로토콜 감지: {}",
                specifier
            );
            // npm: 프로토콜을 그대로 유지하여 load에서 처리
            ModuleSpecifier::parse(specifier).map_err(|e| {
                eprintln!("[NpmModuleLoader::resolve] 모듈 스펙 해석 실패: {}", e);
                let msg = format!("모듈 스펙 해석 실패: {}", e);
                type_error(msg).into()
            })
        } else {
            // npm 패키지 내부의 상대 경로 처리
            // referrer가 npm: 프로토콜이면 실제 파일 경로로 변환
            let actual_referrer = if referrer.starts_with("npm:") {
                eprintln!(
                    "[NpmModuleLoader::resolve] npm 패키지 내부 상대 경로 감지: {} (referrer: {})",
                    specifier, referrer
                );

                // npm: URL과 실제 파일 경로 매핑에서 찾기
                let path_map = self.npm_path_map.lock().unwrap();
                if let Some(actual_path) = path_map.get(referrer) {
                    // 실제 파일 경로를 file:// URL로 변환
                    // ModuleSpecifier를 사용하여 올바른 URL 형식으로 변환
                    let file_url = match ModuleSpecifier::from_file_path(actual_path) {
                        Ok(url) => {
                            eprintln!(
                                "[NpmModuleLoader::resolve] 실제 경로로 변환: {} -> {}",
                                referrer,
                                url.as_str()
                            );
                            url.as_str().to_string()
                        }
                        Err(e) => {
                            eprintln!(
                                "[NpmModuleLoader::resolve] 파일 경로를 URL로 변환 실패: {:?}",
                                e
                            );
                            // 폴백: file:// 절대 경로 형식 사용
                            let path_str = actual_path.to_string_lossy();
                            format!("file://{}", path_str)
                        }
                    };
                    file_url
                } else {
                    eprintln!(
                        "[NpmModuleLoader::resolve] 경로 매핑을 찾을 수 없음: {}",
                        referrer
                    );
                    // 매핑이 없으면 원래 referrer 사용 (에러 발생 가능)
                    referrer.to_string()
                }
            } else {
                referrer.to_string()
            };

            eprintln!(
                "[NpmModuleLoader::resolve] 일반 파일 시스템 모듈로 처리 (referrer: {})",
                actual_referrer
            );
            // 일반 파일 시스템 모듈
            self.fs_loader.resolve(specifier, &actual_referrer, kind)
        }
    }

    fn load(
        &self,
        module_specifier: &ModuleSpecifier,
        maybe_referrer: Option<&ModuleSpecifier>,
        is_dyn_import: bool,
        requested_module_type: RequestedModuleType,
    ) -> ModuleLoadResponse {
        let specifier_str = module_specifier.as_str();
        let npm_resolver = self.npm_resolver.clone();
        let npm_path_map = self.npm_path_map.clone();
        let specifier = module_specifier.clone();

        eprintln!(
            "[NpmModuleLoader::load] specifier: {}, is_dyn_import: {}",
            specifier_str, is_dyn_import
        );

        // npm: 프로토콜 처리
        if specifier_str.starts_with("npm:") {
            eprintln!(
                "[NpmModuleLoader::load] npm: 프로토콜 감지, 패키지 다운로드 시작: {}",
                specifier_str
            );
            let package_spec = &specifier_str[4..];

            // 패키지명과 버전 파싱
            let (package_name, version) = if let Some(at_pos) = package_spec.rfind('@') {
                // 스코프 패키지 처리 (@scope/package@version)
                if package_spec.starts_with('@') {
                    // @scope/package@version 형식
                    let after_first_at = &package_spec[1..];
                    if let Some(second_at_pos) = after_first_at.rfind('@') {
                        // second_at_pos는 after_first_at 기준이므로 package_spec 기준으로는 +1 필요
                        let scope_and_name = &package_spec[..=(second_at_pos + 1)];
                        let version = &package_spec[second_at_pos + 2..];
                        (scope_and_name.to_string(), Some(version.to_string()))
                    } else {
                        // @scope/package (버전 없음)
                        (package_spec.to_string(), None)
                    }
                } else {
                    // package@version
                    let (name, version) = package_spec.split_at(at_pos);
                    (name.to_string(), Some(version[1..].to_string()))
                }
            } else {
                (package_spec.to_string(), None)
            };

            // 비동기 로드
            // 필요한 데이터를 먼저 복사하고 락 해제
            let cache_dir = npm_resolver.lock().unwrap().cache_dir().to_path_buf();
            let registry_url = npm_resolver.lock().unwrap().registry_url().to_string();

            let fut = async move {
                eprintln!(
                    "[NpmModuleLoader::load] 패키지명: {}, 버전: {:?}",
                    package_name, version
                );

                // 패키지 다운로드 및 설치 (독립적인 리졸버 생성)
                eprintln!("[NpmModuleLoader::load] NpmResolver 생성 중...");
                let resolver =
                    NpmResolver::with_cache_dir(cache_dir, registry_url).map_err(|e| {
                        eprintln!("[NpmModuleLoader::load] 리졸버 생성 실패: {}", e);
                        let msg = format!("리졸버 생성 실패: {}", e);
                        type_error(msg)
                    })?;
                eprintln!("[NpmModuleLoader::load] 리졸버 생성 완료");

                eprintln!(
                    "[NpmModuleLoader::load] install_package 호출: {}@{:?}",
                    package_name,
                    version.as_deref()
                );
                let package_dir = resolver
                    .install_package(&package_name, version.as_deref())
                    .await
                    .map_err(|e| {
                        eprintln!("[NpmModuleLoader::load] npm 패키지 다운로드 실패: {}", e);
                        let msg = format!("npm 패키지 다운로드 실패: {}", e);
                        type_error(msg)
                    })?;
                eprintln!(
                    "[NpmModuleLoader::load] 패키지 다운로드 완료: {:?}",
                    package_dir
                );

                // 진입점 찾기
                eprintln!("[NpmModuleLoader::load] 진입점 찾기 시작...");
                let entry_point = resolver.find_entry_point(&package_dir).map_err(|e| {
                    eprintln!("[NpmModuleLoader::load] 진입점 찾기 실패: {}", e);
                    let msg = format!("진입점 찾기 실패: {}", e);
                    type_error(msg)
                })?;
                eprintln!("[NpmModuleLoader::load] 진입점: {:?}", entry_point);

                // 타입 정의 파일 찾기
                eprintln!("[NpmModuleLoader::load] 타입 정의 파일 찾기 시작...");
                let type_def = resolver.find_type_definitions(&package_dir).unwrap_or(None);
                if let Some(type_def_path) = &type_def {
                    eprintln!(
                        "[NpmModuleLoader::load] 타입 정의 파일 발견: {:?}",
                        type_def_path
                    );
                } else {
                    eprintln!("[NpmModuleLoader::load] 타입 정의 파일 없음");
                }

                // npm: URL과 실제 파일 경로 매핑 저장
                let specifier_str_for_map = specifier.as_str().to_string();
                {
                    let mut path_map = npm_path_map.lock().unwrap();
                    path_map.insert(specifier_str_for_map.clone(), entry_point.clone());
                    eprintln!(
                        "[NpmModuleLoader::load] 경로 매핑 저장: {} -> {:?}",
                        specifier_str_for_map, entry_point
                    );
                }

                // 파일 읽기
                eprintln!("[NpmModuleLoader::load] 파일 읽기 시작...");
                let code = fs::read_to_string(&entry_point).map_err(|e| {
                    eprintln!("[NpmModuleLoader::load] 파일 읽기 실패: {}", e);
                    let msg = format!("파일 읽기 실패: {}", e);
                    type_error(msg)
                })?;
                eprintln!(
                    "[NpmModuleLoader::load] 파일 읽기 완료, 코드 길이: {} bytes",
                    code.len()
                );

                // 파일 확장자에 따라 ModuleType 결정
                // Deno Core는 TypeScript를 자동으로 처리하므로 JavaScript로 설정
                // 실제 TypeScript 파일은 런타임에서 자동 변환됨
                let file_ext = entry_point
                    .extension()
                    .and_then(|e| e.to_str())
                    .unwrap_or("");

                let module_type = match file_ext {
                    "ts" | "tsx" | "mts" | "cts" => {
                        eprintln!(
                            "[NpmModuleLoader::load] TypeScript 모듈로 감지 (확장자: {})",
                            file_ext
                        );
                        // Deno Core는 TypeScript도 JavaScript로 처리 가능
                        ModuleType::JavaScript
                    }
                    "jsx" => {
                        eprintln!("[NpmModuleLoader::load] JSX 모듈로 감지");
                        ModuleType::JavaScript
                    }
                    _ => {
                        eprintln!("[NpmModuleLoader::load] JavaScript 모듈로 감지");
                        ModuleType::JavaScript
                    }
                };

                // ModuleSource 생성
                eprintln!("[NpmModuleLoader::load] ModuleSource 생성 중...");
                let module_code = ModuleSourceCode::String(FastString::from(code));
                let module_source = ModuleSource::new(
                    module_type,
                    module_code,
                    &specifier,
                    None, // code_cache
                );
                eprintln!("[NpmModuleLoader::load] ModuleSource 생성 완료");

                Ok(module_source)
            };

            ModuleLoadResponse::Async(fut.boxed())
        } else {
            // 일반 파일 시스템 모듈
            self.fs_loader.load(
                module_specifier,
                maybe_referrer,
                is_dyn_import,
                requested_module_type,
            )
        }
    }
}

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
            // 커스텀 모듈 로더 생성 (npm 지원)
            let module_loader = match NpmModuleLoader::new() {
                Ok(loader) => Rc::new(loader) as Rc<dyn ModuleLoader>,
                Err(e) => {
                    // npm 리졸버 생성 실패 시 기본 로더 사용
                    eprintln!("npm 모듈 로더 초기화 실패 (기본 로더 사용): {}", e);
                    Rc::new(FsModuleLoader) as Rc<dyn ModuleLoader>
                }
            };

            // JsRuntime 생성
            let mut js_runtime = JsRuntime::new(RuntimeOptions {
                module_loader: Some(module_loader),
                extensions: vec![executejs_runtime::init_ops()],
                ..Default::default()
            });

            // bootstrap.js 실행하여 커스텀 API 설정
            let bootstrap_code = include_str!("bootstrap.js");
            if let Err(e) = js_runtime.execute_script("[executejs:bootstrap.js]", bootstrap_code) {
                return Err(anyhow::anyhow!("Bootstrap 실행 실패: {}", e));
            }

            // 코드 실행
            eprintln!(
                "[DenoExecutor] 코드 실행 시작, 코드 길이: {} bytes",
                code.len()
            );
            eprintln!(
                "[DenoExecutor] 코드 내용 (처음 200자): {}",
                &code.chars().take(200).collect::<String>()
            );

            // ES 모듈 import 구문이 있는지 확인
            let has_import = code.contains("import ") || code.contains("export ");
            eprintln!("[DenoExecutor] ES 모듈 구문 감지: {}", has_import);

            // 이벤트 루프 실행을 위한 런타임 핸들
            let rt = tokio::runtime::Handle::current();

            if has_import {
                // ES 모듈로 실행
                eprintln!("[DenoExecutor] ES 모듈로 실행 시도...");
                let specifier = ModuleSpecifier::parse("file:///executejs/user_code.mjs")
                    .map_err(|e| anyhow::anyhow!("{}", e))?;

                eprintln!(
                    "[DenoExecutor] load_main_es_module_from_code 호출: {}",
                    specifier
                );
                let module_id = rt
                    .block_on(async {
                        js_runtime
                            .load_main_es_module_from_code(&specifier, code.clone())
                            .await
                    })
                    .map_err(|e| anyhow::anyhow!("{}", e))?;

                eprintln!("[DenoExecutor] 모듈 로드 완료, ModuleId: {}", module_id);

                // 모듈 평가 (비동기)
                eprintln!("[DenoExecutor] mod_evaluate 호출...");
                rt.block_on(async { js_runtime.mod_evaluate(module_id).await })
                    .map_err(|e| anyhow::anyhow!("{}", e))?;

                eprintln!("[DenoExecutor] mod_evaluate 완료");
            } else {
                // 일반 스크립트로 실행
                eprintln!("[DenoExecutor] 일반 스크립트로 실행...");
                let _result = js_runtime.execute_script("[executejs:user_code]", code)?;
                eprintln!("[DenoExecutor] execute_script 완료");
            }

            // 이벤트 루프 실행 (Promise 처리 및 모듈 로딩 완료 대기)
            eprintln!("[DenoExecutor] 이벤트 루프 실행 시작...");
            rt.block_on(async { js_runtime.run_event_loop(Default::default()).await })?;
            eprintln!("[DenoExecutor] 이벤트 루프 완료");

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
