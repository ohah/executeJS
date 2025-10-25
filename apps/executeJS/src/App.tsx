import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import './App.css';

interface JsExecutionResult {
  code: string;
  result: string;
  timestamp: string;
  success: boolean;
  error?: string;
}

interface AppInfo {
  name: string;
  version: string;
  description: string;
  author: string;
}

function App() {
  const [greetMsg, setGreetMsg] = useState('');
  const [name, setName] = useState('');
  const [jsCode, setJsCode] = useState('');
  const [result, setResult] = useState<JsExecutionResult | null>(null);
  const [history, setHistory] = useState<JsExecutionResult[]>([]);
  const [appInfo, setAppInfo] = useState<AppInfo | null>(null);

  useEffect(() => {
    // 앱 정보 로드
    loadAppInfo();
    // 실행 히스토리 로드
    loadHistory();
  }, []);

  async function loadAppInfo() {
    try {
      const info = await invoke<AppInfo>('get_app_info');
      setAppInfo(info);
    } catch (error) {
      console.error('앱 정보 로드 실패:', error);
    }
  }

  async function loadHistory() {
    try {
      const historyData = await invoke<JsExecutionResult[]>(
        'get_js_execution_history'
      );
      setHistory(historyData);
    } catch (error) {
      console.error('히스토리 로드 실패:', error);
    }
  }

  async function greet() {
    setGreetMsg(await invoke('greet', { name }));
  }

  async function executeJS() {
    try {
      const executionResult = await invoke<JsExecutionResult>('execute_js', {
        code: jsCode,
      });
      setResult(executionResult);
      // 히스토리 새로고침
      loadHistory();
    } catch (error) {
      setResult({
        code: jsCode,
        result: '',
        timestamp: new Date().toISOString(),
        success: false,
        error: error as string,
      });
    }
  }

  async function clearHistory() {
    try {
      await invoke('clear_js_execution_history');
      setHistory([]);
    } catch (error) {
      console.error('히스토리 삭제 실패:', error);
    }
  }

  async function saveCode() {
    const filename = prompt('저장할 파일명을 입력하세요:', 'code.js');
    if (filename) {
      try {
        const message = await invoke<string>('save_js_code', {
          code: jsCode,
          filename,
        });
        alert(message);
      } catch (error) {
        alert(`저장 실패: ${error}`);
      }
    }
  }

  async function loadCode() {
    const filename = prompt('불러올 파일명을 입력하세요:', 'code.js');
    if (filename) {
      try {
        const code = await invoke<string>('load_js_code', { filename });
        setJsCode(code);
      } catch (error) {
        alert(`불러오기 실패: ${error}`);
      }
    }
  }

  return (
    <div className="container">
      <header className="app-header">
        <h1>ExecuteJS</h1>
        {appInfo && (
          <div className="app-info">
            <p>
              v{appInfo.version} - {appInfo.description}
            </p>
          </div>
        )}
      </header>

      <div className="row">
        <div>
          <input
            id="greet-input"
            onChange={(e) => setName(e.currentTarget.value)}
            placeholder="이름을 입력하세요..."
          />
          <button type="button" onClick={() => greet()}>
            인사하기
          </button>
        </div>
      </div>

      {greetMsg && <p className="greet-message">{greetMsg}</p>}

      <div className="row">
        <div className="code-section">
          <h3>JavaScript 코드 실행</h3>
          <div className="code-controls">
            <button type="button" onClick={() => executeJS()}>
              실행
            </button>
            <button type="button" onClick={() => saveCode()}>
              저장
            </button>
            <button type="button" onClick={() => loadCode()}>
              불러오기
            </button>
            <button type="button" onClick={() => clearHistory()}>
              히스토리 삭제
            </button>
          </div>
          <textarea
            value={jsCode}
            onChange={(e) => setJsCode(e.target.value)}
            placeholder="JavaScript 코드를 입력하세요..."
            rows={10}
            cols={50}
          />
        </div>
      </div>

      {result && (
        <div
          className={`result-section ${result.success ? 'success' : 'error'}`}
        >
          <h3>실행 결과:</h3>
          <div className="result-info">
            <span className="timestamp">
              {new Date(result.timestamp).toLocaleString()}
            </span>
            <span className={`status ${result.success ? 'success' : 'error'}`}>
              {result.success ? '성공' : '실패'}
            </span>
          </div>
          <pre>{result.success ? result.result : result.error}</pre>
        </div>
      )}

      {history.length > 0 && (
        <div className="history-section">
          <h3>실행 히스토리 ({history.length}개)</h3>
          <div className="history-list">
            {history
              .slice(-5)
              .reverse()
              .map((item, index) => (
                <div
                  key={index}
                  className={`history-item ${item.success ? 'success' : 'error'}`}
                >
                  <div className="history-header">
                    <span className="timestamp">
                      {new Date(item.timestamp).toLocaleString()}
                    </span>
                    <span
                      className={`status ${item.success ? 'success' : 'error'}`}
                    >
                      {item.success ? '성공' : '실패'}
                    </span>
                  </div>
                  <div className="history-code">{item.code}</div>
                  <div className="history-result">
                    {item.success ? item.result : item.error}
                  </div>
                </div>
              ))}
          </div>
        </div>
      )}
    </div>
  );
}

export default App;
