# ExecuteJS Agent

ExecuteJS 프로젝트의 AI 에이전트 설정 및 가이드입니다.

## 프로젝트 개요

ExecuteJS는 JavaScript 코드를 안전하게 실행할 수 있는 Tauri 기반 데스크톱 애플리케이션입니다.

### 기술 스택

- **Frontend**: React 19, TypeScript, Vite, Tailwind CSS 4.x
- **Backend**: Rust, Tauri 2.0
- **Testing**: Vitest, Testing Library
- **Linting**: oxlint, Prettier, rustfmt, clippy
- **Documentation**: RSPress
- **Package Manager**: pnpm
- **CI/CD**: GitHub Actions
- **Architecture**: Feature-Sliced Design (FSD)

### 모노레포 구조

```
executeJS/
├── apps/
│   └── executeJS/          # Tauri 애플리케이션
│       ├── src/            # React 프론트엔드
│       └── src-tauri/      # Rust 백엔드
├── packages/               # 공유 패키지들
├── crates/                 # Rust 크레이트들
├── docs/                   # 문서화 (RSPress)
└── .github/workflows/      # CI/CD 워크플로우
```

## 개발 가이드

### 필수 요구사항

- Node.js 22 LTS (`.nvmrc` 파일 참조)
- pnpm 10+
- Rust 1.70+
- Tauri CLI

### 주요 명령어

```bash
# 개발
pnpm tauri:dev              # Tauri 개발 서버
pnpm docs:dev               # 문서 개발 서버

# 빌드
pnpm build                  # 전체 빌드
pnpm tauri:build           # Tauri 앱 빌드
pnpm docs:build            # 문서 빌드

# 테스트
pnpm test                  # 프론트엔드 테스트
cargo test                 # Rust 테스트

# 품질 관리
pnpm lint                  # 린트 검사
pnpm format                # 코드 포맷팅
pnpm type-check            # 타입 검사
```

## 코딩 컨벤션

### JavaScript/TypeScript

- **린터**: oxlint (eslint 대신 사용)
- **포맷터**: Prettier
- **타입**: TypeScript strict 모드
- **테스트**: Vitest + Testing Library
- **아키텍처**: Feature-Sliced Design (FSD)
- **스타일링**: Tailwind CSS 4.x
- **경로 별칭**: `@/*` → `./src/*`

### Rust

- **포맷터**: rustfmt (기본 설정 사용)
- **린터**: clippy
- **테스트**: cargo test
- **탭 크기**: 4 스페이스

### 커밋 컨벤션

```
type(scope): description

feat: 새로운 기능 추가
fix: 버그 수정
docs: 문서 수정
style: 코드 포맷팅
refactor: 코드 리팩토링
test: 테스트 추가/수정
chore: 빌드 설정 변경
```

## CI/CD 워크플로우

### 폴더별 조건부 실행

- **javascript-lint.yml**: `apps/executeJS/src/**` 변경 시
- **rust-lint.yml**: `apps/executeJS/src-tauri/**`, `crates/**` 변경 시
- **frontend-test.yml**: 프론트엔드 코드 변경 시
- **rust-test.yml**: Rust 코드 변경 시
- **build.yml**: 전체 프로젝트 변경 시
- **docs.yml**: `docs/**` 변경 시

### 브랜치 정책

- **main**: 메인 브랜치 (배포용)
- **PR**: 모든 변경사항은 PR을 통해 검토

## 아키텍처

### Feature-Sliced Design (FSD)

프로젝트는 FSD 아키텍처를 따릅니다:

```
src/
├── app/                    # 앱 초기화 및 프로바이더
├── pages/                  # 페이지 레벨 컴포넌트
├── widgets/                # 복합 UI 블록
├── features/               # 비즈니스 로직 기능
└── shared/                 # 공유 유틸리티
    ├── ui/                 # UI 컴포넌트
    └── types/              # 타입 정의
```

#### FSD 레이어 규칙

- **app** → pages, widgets, features, shared
- **pages** → widgets, features, shared
- **widgets** → features, shared
- **features** → shared
- **shared** → (다른 레이어 import 금지)

#### 파일 명명 규칙

- React 컴포넌트: kebab-case (예: `editor-page.tsx`)
- 각 레이어에 `index.ts` 파일로 export 정리
- `export * from` 패턴 사용

#### 경로 별칭

- `@/*` → `./src/*` (TypeScript, Vite 설정)
- 절대 경로 import 사용 권장

### 프론트엔드 (React)

- **상태 관리**: React Hooks
- **빌드 도구**: Vite
- **테스팅**: Vitest + Testing Library
- **스타일링**: Tailwind CSS 4.x

### 백엔드 (Rust + Tauri)

- **프레임워크**: Tauri 2.0
- **로깅**: tracing
- **에러 처리**: anyhow, thiserror
- **플러그인**: HTTP, File System, Store, Clipboard, Opener, DevTools

### 주요 기능

- JavaScript 코드 실행 (시뮬레이션)
- 실행 기록 관리
- 코드 저장/로드
- 앱 정보 조회

## 파일 구조

### 중요한 설정 파일

- `package.json`: 루트 패키지 설정
- `pnpm-workspace.yaml`: pnpm 워크스페이스 설정
- `Cargo.toml`: Rust 워크스페이스 설정
- `rustfmt.toml`: Rust 포맷터 설정
- `oxlint.json`: JavaScript 린터 설정
- `.vscode/`: VSCode 설정

### Tauri 설정

- `apps/executeJS/src-tauri/tauri.conf.json`: Tauri 앱 설정
- `apps/executeJS/src-tauri/Cargo.toml`: Tauri 앱 의존성
- `apps/executeJS/src-tauri/src/`: Rust 백엔드 소스

### 문서화

- `docs/`: RSPress 문서 사이트
- `docs/docs/`: 문서 소스 파일
- `docs/rspress.config.ts`: RSPress 설정
- **배포**: GitHub Pages (https://ohah.github.io/executeJS/)

## 주의사항

1. **의존성 관리**: 모든 의존성은 `latest` 버전 사용
2. **코드 품질**: 린트와 포맷팅을 통과해야 함
3. **테스트**: 새로운 기능은 테스트 포함 필요
4. **문서화**: API 변경 시 문서 업데이트 필요
5. **CI/CD**: 모든 워크플로우가 성공해야 함
6. **FSD 아키텍처**: 레이어 간 의존성 규칙 준수 필수
7. **파일 명명**: kebab-case 사용 (예: `editor-page.tsx`)
8. **Import 경로**: `@` 별칭 사용 권장
9. **Export 정리**: 각 레이어의 `index.ts`에서 `export * from` 패턴 사용

## 문제 해결

### 일반적인 문제

- **빌드 실패**: `pnpm install` 후 `cargo build` 실행
- **테스트 실패**: `pnpm test` 및 `cargo test` 개별 실행
- **린트 오류**: `pnpm lint:fix` 및 `cargo fmt` 실행
- **타입 오류**: `pnpm type-check` 실행

### 디버깅

- **프론트엔드**: 브라우저 개발자 도구 사용
- **백엔드**: `tracing` 로그 확인
- **Tauri**: `tauri dev` 실행 시 콘솔 로그 확인
