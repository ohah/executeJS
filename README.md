# ExecuteJS

ExecuteJS는 JavaScript 코드를 실행할 수 있는 Tauri 기반 데스크톱 애플리케이션입니다.

## 구조

이 프로젝트는 모노레포 구조로 구성되어 있습니다:

```
executeJS/
├── apps/
│   └── executeJS/          # Tauri 애플리케이션
│       ├── src/            # React 프론트엔드
│       └── src-tauri/      # Rust 백엔드
├── packages/               # 공유 패키지들
├── crates/                 # Rust 크레이트들
├── Cargo.toml              # Rust 워크스페이스 설정
├── package.json            # Node.js 워크스페이스 설정
└── pnpm-workspace.yaml     # pnpm 워크스페이스 설정
```

## 개발 환경 설정

### 필수 요구사항

- Node.js 22 LTS (`.nvmrc` 파일 참조)
- pnpm 10+
- Rust 1.70+
- Tauri CLI
- VSCode (권장)

### 설치

```bash
# 의존성 설치
pnpm install

# Rust 의존성 설치 (자동으로 실행됨)
cargo build
```

## 주요 기능

- **JavaScript 코드 실행**: 실시간 JavaScript 코드 실행 및 결과 확인
- **실행 히스토리**: 최근 실행한 코드들의 히스토리 관리
- **코드 저장/불러오기**: JavaScript 코드를 파일로 저장하고 불러오기
- **로그 시스템**: 개발 및 디버깅을 위한 로깅 기능
- **Tauri 플러그인**: 파일 시스템, HTTP, 클립보드 등 다양한 네이티브 기능

### 개발 서버 실행

```bash
# Tauri 개발 서버 실행
pnpm tauri:dev

# 또는 개별 앱 실행
pnpm --filter @executeJS/app dev
```

### 빌드

```bash
# 전체 프로젝트 빌드
pnpm build

# Tauri 앱 빌드
pnpm tauri:build
```

## 스크립트

- `pnpm dev` - 개발 서버 실행
- `pnpm build` - 프로덕션 빌드
- `pnpm test` - 테스트 실행
- `pnpm lint` - 린트 검사
- `pnpm format` - 전체 코드 포맷팅 (Prettier + rustfmt)
- `pnpm format:rust` - Rust 코드 포맷팅
- `pnpm tauri:dev` - Tauri 개발 서버
- `pnpm tauri:build` - Tauri 앱 빌드
- `pnpm docs:dev` - 문서 개발 서버
- `pnpm docs:build` - 문서 빌드
- `pnpm docs:preview` - 문서 미리보기
- `pnpm clean` - 빌드 파일 정리

## 기술 스택

- **Frontend**: React 19, TypeScript, Vite
- **Backend**: Rust, Tauri v2
- **Package Manager**: pnpm
- **Linting**: oxlint
- **Formatting**: Prettier (JS/TS), rustfmt (Rust)
- **IDE**: VSCode (권장 확장 프로그램 포함)
- **Tauri Plugins**: HTTP, File System, Store, Clipboard, Opener, DevTools

## VSCode 설정

프로젝트에는 VSCode 설정이 포함되어 있습니다:

- **권장 확장 프로그램**: oxc, rust-analyzer, tauri-vscode 등
- **자동 포맷팅**: 저장 시 자동으로 Prettier와 rustfmt 실행
- **자동 수정**: 저장 시 oxc 린터 자동 수정
- **언어별 탭 설정**: Rust(4스페이스), JS/TS(2스페이스)

## CI/CD

GitHub Actions를 통한 자동화된 품질 관리 (폴더별 조건부 실행):

### 워크플로우 구조

- **javascript-lint.yml**: 프론트엔드 코드 변경 시 oxlint + Prettier 검사
- **rust-lint.yml**: Rust 코드 변경 시 rustfmt + clippy 검사
- **frontend-test.yml**: 프론트엔드 코드 변경 시 vitest + TypeScript 타입 검사
- **rust-test.yml**: Rust 코드 변경 시 cargo test 실행
- **build.yml**: 전체 프로젝트 변경 시 빌드 검증

### 조건부 실행

각 워크플로우는 관련 폴더의 변경사항이 있을 때만 실행됩니다:

- **프론트엔드 관련**: `apps/executeJS/src/**`, `apps/executeJS/package.json` 등
- **Rust 관련**: `apps/executeJS/src-tauri/**`, `crates/**`, `Cargo.toml` 등
- **전체 빌드**: 모든 앱 관련 파일 변경 시

이를 통해 불필요한 CI 실행을 방지하고 빠른 피드백을 제공합니다.

## 문서화

프로젝트는 RSPress를 사용한 현대적인 문서화 시스템을 제공합니다:

- **온라인 문서**: [GitHub Pages](https://ohah.github.io/executeJS/) (자동 배포)
- **로컬 개발**: `pnpm docs:dev`로 로컬에서 문서 개발
- **자동 배포**: `docs/` 폴더 변경 시 자동으로 문서 업데이트

### 문서 구조

- **가이드**: 시작하기, 개발 가이드
- **API 참조**: Tauri 명령어 및 타입 정의
- **예제**: 사용법 및 코드 예제

### 문서 개발

```bash
# 문서 개발 서버 실행
pnpm docs:dev

# 문서 빌드
pnpm docs:build

# 빌드된 문서 미리보기
pnpm docs:preview
```

## 개발자 가이드

### AI 에이전트 설정

- **agent.md**: 프로젝트 개요 및 AI 에이전트 가이드
- **.cursorrules**: Cursor IDE용 코딩 규칙 및 컨벤션

### 코드 품질

- **oxlint**: JavaScript/TypeScript 린팅
- **Prettier**: 코드 포맷팅
- **rustfmt**: Rust 코드 포맷팅
- **clippy**: Rust 린팅
- **Vitest**: 프론트엔드 테스트
- **cargo test**: Rust 테스트
