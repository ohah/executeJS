use anyhow::{Context, Result};
use oxc_allocator::Allocator;
use oxc_ast::ast::{
    Argument, CallExpression, Expression, ImportDeclaration, ImportExpression, MemberExpression,
};
use oxc_ast_visit::Visit;
use oxc_parser::Parser;
use oxc_span::SourceType;
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::process::Stdio;
use tokio::io::AsyncReadExt;
use tokio::process::Command;

/// npm 패키지 관리자
pub struct NpmManager {
    node_modules_path: PathBuf,
    node_path: PathBuf,
}

impl NpmManager {
    /// 새로운 NpmManager 인스턴스 생성
    pub fn new(node_path: PathBuf) -> Result<Self> {
        let node_modules_path = Self::get_node_modules_path(&node_path)?;
        Ok(Self {
            node_modules_path,
            node_path,
        })
    }

    /// Node.js 바이너리 경로에서 node_modules 경로 계산
    fn get_node_modules_path(node_path: &Path) -> Result<PathBuf> {
        let node_dir = node_path
            .parent()
            .context("Node.js 바이너리 경로가 유효하지 않습니다")?;

        // macOS/Linux: bin/node -> bin/node_modules
        // Windows: node.exe -> node_modules (루트)
        if cfg!(target_os = "windows") {
            // Windows: 루트의 node.exe와 같은 디렉토리
            Ok(node_dir.join("node_modules"))
        } else {
            // macOS/Linux: bin/node와 같은 디렉토리 (bin/)
            Ok(node_dir.join("node_modules"))
        }
    }

    /// node_modules 경로 반환
    pub fn node_modules_path(&self) -> &Path {
        &self.node_modules_path
    }

    /// 코드에서 필요한 npm 패키지 목록 추출
    pub fn parse_required_packages(code: &str) -> Result<Vec<String>> {
        let allocator = Allocator::default();
        let source_type = SourceType::default().with_module(true);

        let ret = Parser::new(&allocator, code, source_type).parse();

        if !ret.errors.is_empty() {
            // 파싱 오류가 있어도 계속 진행 (일부 패키지만 추출)
            tracing::warn!(
                "코드 파싱 중 오류 발생 ({}개), 계속 진행합니다",
                ret.errors.len()
            );
        }

        let mut extractor = PackageExtractor::new();
        extractor.visit_program(&ret.program);

        Ok(extractor.packages.into_iter().collect())
    }

    /// 패키지가 이미 설치되어 있는지 확인
    pub fn is_package_installed(&self, package_name: &str) -> bool {
        let package_dir = self.node_modules_path.join(package_name);
        package_dir.exists() && package_dir.is_dir()
    }

    /// 패키지 설치 (npm install 실행)
    pub async fn install_package(&self, package_name: &str) -> Result<()> {
        // 이미 설치되어 있으면 스킵
        if self.is_package_installed(package_name) {
            tracing::debug!("패키지가 이미 설치되어 있습니다: {}", package_name);
            return Ok(());
        }

        tracing::info!("패키지 설치 시작: {}", package_name);

        // node_modules 디렉토리 생성
        std::fs::create_dir_all(&self.node_modules_path)
            .context("node_modules 디렉토리 생성 실패")?;

        // npm 바이너리 경로 찾기
        let npm_path = self.find_npm_binary()?;

        // npm install 실행
        let node_dir = self
            .node_path
            .parent()
            .context("Node.js 바이너리 경로가 유효하지 않습니다")?;

        let mut child = Command::new(&npm_path)
            .arg("install")
            .arg(package_name)
            .current_dir(node_dir)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .context("npm install 프로세스 시작 실패")?;

        let mut stdout = String::new();
        let mut stderr = String::new();

        if let Some(mut child_stdout) = child.stdout.take() {
            let mut reader = tokio::io::BufReader::new(&mut child_stdout);
            reader.read_to_string(&mut stdout).await.ok();
        }

        if let Some(mut child_stderr) = child.stderr.take() {
            let mut reader = tokio::io::BufReader::new(&mut child_stderr);
            reader.read_to_string(&mut stderr).await.ok();
        }

        let status = child
            .wait()
            .await
            .context("npm install 프로세스 대기 실패")?;

        if !status.success() {
            anyhow::bail!(
                "npm install 실패: {}\nstdout: {}\nstderr: {}",
                package_name,
                stdout,
                stderr
            );
        }

        tracing::info!("패키지 설치 완료: {}", package_name);
        Ok(())
    }

    /// 여러 패키지 설치
    pub async fn install_packages(&self, package_names: &[String]) -> Result<()> {
        for package_name in package_names {
            self.install_package(package_name).await?;
        }
        Ok(())
    }

    /// npm 바이너리 경로 찾기
    fn find_npm_binary(&self) -> Result<PathBuf> {
        let node_dir = self
            .node_path
            .parent()
            .context("Node.js 바이너리 경로가 유효하지 않습니다")?;

        // npm은 Node.js와 함께 번들되어 있음
        // macOS/Linux: bin/npm
        // Windows: npm.cmd 또는 npm
        let npm_name = if cfg!(target_os = "windows") {
            "npm.cmd"
        } else {
            "npm"
        };

        let npm_path = if self.node_path.file_name().and_then(|n| n.to_str()) == Some("node") {
            // bin/node인 경우 -> bin/npm
            node_dir.join(npm_name)
        } else {
            // node.exe인 경우 -> npm.cmd (같은 디렉토리)
            node_dir.join(npm_name)
        };

        if npm_path.exists() {
            Ok(npm_path)
        } else {
            // npm.cmd가 없으면 npm 시도 (Windows)
            if cfg!(target_os = "windows") {
                let npm_path_alt = node_dir.join("npm");
                if npm_path_alt.exists() {
                    return Ok(npm_path_alt);
                }
            }
            anyhow::bail!("npm 바이너리를 찾을 수 없습니다: {}", npm_path.display())
        }
    }
}

/// AST를 순회하여 패키지명을 추출하는 방문자
struct PackageExtractor {
    packages: HashSet<String>,
    in_require_call: bool,    // require() 호출 컨텍스트 추적
    in_require_resolve: bool, // require.resolve() 호출 컨텍스트 추적
}

impl PackageExtractor {
    fn new() -> Self {
        Self {
            packages: HashSet::new(),
            in_require_call: false,
            in_require_resolve: false,
        }
    }

    fn extract_package_name_from_string(&mut self, value: &str) {
        // 로컬 파일 경로 제외
        if value.starts_with('.') || value.starts_with('/') {
            return;
        }

        // 스코프 패키지 또는 일반 패키지
        // @scope/package 또는 package
        if !value.is_empty() {
            self.packages.insert(value.to_string());
        }
    }
}

impl<'a> Visit<'a> for PackageExtractor {
    fn visit_call_expression(&mut self, expr: &CallExpression<'a>) {
        // require('package-name') 감지
        let was_in_require = self.in_require_call;
        let was_in_resolve = self.in_require_resolve;

        if let Expression::Identifier(ident) = &expr.callee {
            if ident.name.as_str() == "require" {
                self.in_require_call = true;
            }
        }

        // require.resolve('package-name') 감지
        // visit_member_expression에서 처리

        // 하위 노드 방문 (arguments 포함)
        // walk 함수는 oxc_ast_visit에 없을 수 있으므로 직접 처리
        for arg in &expr.arguments {
            self.visit_argument(arg);
        }

        // 컨텍스트 복원
        self.in_require_call = was_in_require;
        self.in_require_resolve = was_in_resolve;
    }

    fn visit_argument(&mut self, _arg: &Argument<'a>) {
        // Argument를 방문하여 내부 Expression 추출
        // visit_expression에서 처리됨
    }

    fn visit_expression(&mut self, expr: &Expression<'a>) {
        // require() 또는 require.resolve() 호출의 인자인 경우에만 StringLiteral 추출
        if self.in_require_call || self.in_require_resolve {
            if let Expression::StringLiteral(lit) = expr {
                let value = lit.value.to_string();
                self.extract_package_name_from_string(&value);
            }
        }
    }

    fn visit_member_expression(&mut self, member_expr: &MemberExpression<'a>) {
        // require.resolve('package-name') 감지
        if let MemberExpression::StaticMemberExpression(static_member) = member_expr {
            if let Expression::Identifier(ident) = &static_member.object {
                if ident.name.as_str() == "require"
                    && static_member.property.name.as_str() == "resolve"
                {
                    self.in_require_resolve = true;
                }
            }
        }
    }

    fn visit_import_declaration(&mut self, decl: &ImportDeclaration<'a>) {
        // import ... from 'package-name' 감지
        let value = decl.source.value.to_string();
        self.extract_package_name_from_string(&value);
    }

    fn visit_import_expression(&mut self, expr: &ImportExpression<'a>) {
        // import('package-name') 감지
        match &expr.source {
            Expression::StringLiteral(lit) => {
                let value = lit.value.to_string();
                self.extract_package_name_from_string(&value);
            }
            _ => {}
        }
    }
}
