# 🚀 ExecuteJS: Deno Core 기반 JavaScript 런타임 구현

## 📋 PR 개요

ExecuteJS 프로젝트에 Deno Core를 기반으로 한 JavaScript 런타임을 구현하여, 데스크톱 애플리케이션에서 JavaScript 코드를 실행할 수 있는 기능을 추가했습니다.

## 🎯 주요 기능

### ✅ JavaScript 런타임

- **Deno Core 0.323** 기반 V8 JavaScript 엔진
- **Chrome DevTools 수준**의 `console.log()` 출력
- **`alert()`** 함수 지원
- **실제 JavaScript 엔진** 수준의 문법 오류 감지
- **변수 할당 및 계산** 지원

### ✅ npm 모듈 시뮬레이션

- **lodash**: `map`, `filter`, `reduce`, `find`, `chunk` 함수
- **moment**: `now`, `format` 함수
- **uuid**: `v4` 함수
- **require()** 함수를 통한 모듈 로딩
- **Node.js 스타일** 모듈 시스템 지원 (`module.exports`, `exports`)

### ✅ Tauri 2.0 호환성

- **Send 트레이트** 문제 해결 (`tokio::task::spawn_blocking` 사용)
- **Tauri 2.0** 완전 호환
- **스레드 안전** 출력 버퍼링

## 🏗️ 아키텍처

### 핵심 컴포넌트

```
apps/executeJS/src-tauri/src/
├── deno_runtime.rs          # Deno Core 런타임 구현
├── bootstrap.js             # JavaScript API 정의
├── js_executor.rs           # 실행 결과 관리
└── commands.rs              # Tauri 명령어
```

### 실행 흐름

1. **초기화**: `DenoExecutor::new()` - 출력 버퍼 설정
2. **실행**: `execute_script()` - 별도 스레드에서 Deno Core 실행
3. **API 연결**: `bootstrap.js` - console.log, alert 등 커스텀 API
4. **결과 처리**: 출력 버퍼에서 결과 수집 및 반환

## 📁 변경된 파일

### 새로 추가된 파일

- `apps/executeJS/src-tauri/src/deno_runtime.rs` - Deno Core 런타임 구현
- `apps/executeJS/src-tauri/src/bootstrap.js` - JavaScript API 정의

### 수정된 파일

- `apps/executeJS/src-tauri/Cargo.toml` - Deno Core 의존성 추가
- `apps/executeJS/src-tauri/src/js_executor.rs` - Deno 런타임 통합
- `apps/executeJS/src-tauri/src/commands.rs` - async 함수로 변경
- `.gitignore` - Tauri 생성 파일 무시 설정
- `.cursorrules` - 아키텍처 문서화

## 🧪 테스트 결과

### 기본 JavaScript 실행

```javascript
console.log('Hello World'); // ✅ "Hello World"
alert('Hello Alert'); // ✅ "[ALERT] Hello Alert"
let a = 5;
console.log(a); // ✅ "5"
let x = 1;
let y = 2;
console.log(x + y); // ✅ "3"
```

### npm 모듈 사용

```javascript
const _ = require('lodash');
const numbers = [1, 2, 3, 4, 5];
const doubled = _.map(numbers, (n) => n * 2);
console.log('Lodash test:', doubled); // ✅ "[2, 4, 6, 8, 10]"
```

### 문법 오류 감지

```javascript
alert('adf'(;  // ✅ 문법 오류로 실행 실패
```

## 🔧 기술적 구현

### Send 트레이트 문제 해결

```rust
// 별도 스레드에서 Deno Core 실행 (Send 트레이트 문제 해결)
let result = tokio::task::spawn_blocking(move || {
    let mut js_runtime = JsRuntime::new(RuntimeOptions {
        module_loader: Some(Rc::new(FsModuleLoader)),
        extensions: vec![executejs_runtime::init_ops()],
        ..Default::default()
    });
    // ... 실행 로직
}).await?;
```

### 커스텀 op 함수

```rust
#[op2(fast)]
#[string]
fn op_console_log(#[string] message: String) -> Result<(), AnyError> {
    // 출력 버퍼에 메시지 추가
    Ok(())
}
```

### npm 모듈 시뮬레이션

```javascript
globalThis.require = (moduleName) => {
  const modules = {
    'lodash': { map, filter, reduce, find, chunk },
    'moment': { now, format },
    'uuid': { v4 }
  };
  return modules[moduleName] || throw new Error(...);
};
```

## 📊 성능 및 안정성

- ✅ **스레드 안전**: Mutex를 사용한 출력 버퍼 관리
- ✅ **메모리 효율**: 각 실행마다 새로운 JsRuntime 인스턴스
- ✅ **오류 처리**: 실제 JavaScript 엔진 수준의 오류 감지
- ✅ **테스트 격리**: 테스트 간 전역 상태 충돌 방지

## 🚀 사용 예시

### 기본 사용법

```javascript
// 변수 할당 및 계산
let a = 10;
let b = 20;
console.log('합계:', a + b); // "합계: 30"

// 배열 처리
const numbers = [1, 2, 3, 4, 5];
const sum = numbers.reduce((acc, n) => acc + n, 0);
console.log('합계:', sum); // "합계: 15"
```

### npm 모듈 사용

```javascript
// lodash 사용
const _ = require('lodash');
const data = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
const chunks = _.chunk(data, 3);
console.log('청크:', chunks); // [[1,2,3], [4,5,6], [7,8,9], [10]]

// moment 사용
const moment = require('moment');
console.log('현재 시간:', moment.now());

// uuid 사용
const uuid = require('uuid');
console.log('UUID:', uuid.v4());
```

## 🔮 향후 계획

- [ ] **실제 npm 다운로드**: Tauri 호환성 문제 해결 후 실제 npm 패키지 다운로드
- [ ] **ES6 import 지원**: `import` 문법으로 모듈 로딩
- [ ] **더 많은 npm 모듈**: axios, express 등 추가 모듈 지원
- [ ] **파일 시스템 API**: fs, path 등 Node.js API 지원

## 📝 커밋 히스토리

- `afa9e5b` - feat: npm 모듈 시뮬레이션 완성 및 버전 업데이트
- `9928908` - chore: gen 디렉토리를 git에서 제거하고 .gitignore에 추가
- `2e1ad25` - feat: npm 모듈 시뮬레이션 구현
- `[이전 커밋들]` - Deno Core 런타임 구현 및 Tauri 통합

## 🎉 결론

ExecuteJS는 이제 Deno Core 기반의 강력한 JavaScript 런타임을 제공합니다. Chrome DevTools 수준의 출력과 npm 모듈 시뮬레이션을 통해 사용자가 데스크톱에서 JavaScript 코드를 편리하게 실행하고 테스트할 수 있습니다.

**주요 성과:**

- ✅ Deno Core 0.323 기반 V8 JavaScript 엔진 통합
- ✅ Tauri 2.0 완전 호환
- ✅ npm 모듈 시뮬레이션 (lodash, moment, uuid)
- ✅ Chrome DevTools 수준의 console.log 및 alert 지원
- ✅ 실제 JavaScript 엔진 수준의 문법 오류 감지
- ✅ 스레드 안전 및 메모리 효율적인 구현
