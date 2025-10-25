// ExecuteJS 커스텀 런타임 bootstrap
const { core } = Deno;
const { ops } = core;

// console 객체 정의
globalThis.console = {
  log: (...args) => {
    const message = args.map(arg => {
      if (typeof arg === 'object') {
        return JSON.stringify(arg, null, 2);
      }
      return String(arg);
    }).join(' ');
    ops.op_console_log(message);
  },
  
  error: (...args) => {
    const message = args.map(arg => {
      if (typeof arg === 'object') {
        return JSON.stringify(arg, null, 2);
      }
      return String(arg);
    }).join(' ');
    ops.op_custom_print(message, true);
  },
  
  warn: (...args) => {
    const message = args.map(arg => {
      if (typeof arg === 'object') {
        return JSON.stringify(arg, null, 2);
      }
      return String(arg);
    }).join(' ');
    ops.op_custom_print(`[WARN] ${message}`, false);
  },
  
  info: (...args) => {
    const message = args.map(arg => {
      if (typeof arg === 'object') {
        return JSON.stringify(arg, null, 2);
      }
      return String(arg);
    }).join(' ');
    ops.op_custom_print(`[INFO] ${message}`, false);
  }
};

// alert 함수 정의
globalThis.alert = (message) => {
  ops.op_alert(String(message));
};

// print 함수 정의 (Deno.core.print 대체)
globalThis.print = (message, isErr = false) => {
  ops.op_custom_print(String(message), isErr);
};

// npm 모듈 지원을 위한 require 함수 정의 (시뮬레이션)
globalThis.require = (moduleName) => {
  // 간단한 npm 모듈 시뮬레이션
  const modules = {
    'lodash': {
      map: (array, iteratee) => {
        if (!Array.isArray(array)) {
          throw new Error('First argument must be an array');
        }
        return array.map(iteratee);
      },
      filter: (array, predicate) => {
        if (!Array.isArray(array)) {
          throw new Error('First argument must be an array');
        }
        return array.filter(predicate);
      },
      reduce: (array, iteratee, accumulator) => {
        if (!Array.isArray(array)) {
          throw new Error('First argument must be an array');
        }
        return array.reduce(iteratee, accumulator);
      },
      find: (array, predicate) => {
        if (!Array.isArray(array)) {
          throw new Error('First argument must be an array');
        }
        return array.find(predicate);
      },
      chunk: (array, size) => {
        if (!Array.isArray(array)) {
          throw new Error('First argument must be an array');
        }
        const chunks = [];
        for (let i = 0; i < array.length; i += size) {
          chunks.push(array.slice(i, i + size));
        }
        return chunks;
      }
    },
    'moment': {
      now: () => new Date(),
      format: (date, format) => {
        if (!(date instanceof Date)) {
          date = new Date(date);
        }
        return date.toISOString();
      }
    },
    'uuid': {
      v4: () => {
        return 'xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx'.replace(/[xy]/g, function(c) {
          const r = Math.random() * 16 | 0;
          const v = c == 'x' ? r : (r & 0x3 | 0x8);
          return v.toString(16);
        });
      }
    }
  };

  if (modules[moduleName]) {
    return modules[moduleName];
  }

  throw new Error(`Cannot find module '${moduleName}'. Available modules: ${Object.keys(modules).join(', ')}`);
};

// 기본적인 전역 객체들 정의
if (typeof globalThis.window === 'undefined') {
  globalThis.window = globalThis;
}

if (typeof globalThis.global === 'undefined') {
  globalThis.global = globalThis;
}

// Node.js 스타일 모듈 시스템 지원
if (typeof globalThis.module === 'undefined') {
  globalThis.module = { exports: {} };
}

if (typeof globalThis.exports === 'undefined') {
  globalThis.exports = globalThis.module.exports;
}
