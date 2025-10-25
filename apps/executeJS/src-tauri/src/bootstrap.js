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

// 기본적인 전역 객체들 정의
if (typeof globalThis.window === 'undefined') {
  globalThis.window = globalThis;
}

if (typeof globalThis.global === 'undefined') {
  globalThis.global = globalThis;
}
