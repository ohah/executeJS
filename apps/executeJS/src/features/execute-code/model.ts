import { observable } from '@legendapp/state';
import type { JsExecutionResult } from '../../shared/types';

// 코드 실행 상태
export const executionState = observable({
  result: null as JsExecutionResult | null,
  isExecuting: false,
  history: [] as JsExecutionResult[],
});

// Tauri command 호출을 위한 API 함수들
export const executeApi = {
  // JavaScript 코드 실행
  executeCode: async (code: string): Promise<JsExecutionResult> => {
    // Tauri command 호출
    const { invoke } = await import('@tauri-apps/api/core');

    try {
      const result = await invoke<JsExecutionResult>('execute_js', { code });
      return result;
    } catch (error) {
      return {
        code,
        result: '',
        timestamp: new Date().toISOString(),
        success: false,
        error: error as string,
      };
    }
  },

  // 실행 히스토리 가져오기
  getHistory: async (): Promise<JsExecutionResult[]> => {
    const { invoke } = await import('@tauri-apps/api/core');

    try {
      return await invoke<JsExecutionResult[]>('get_js_execution_history');
    } catch (error) {
      console.error('Failed to load execution history:', error);
      return [];
    }
  },

  // 히스토리 삭제
  clearHistory: async (): Promise<void> => {
    const { invoke } = await import('@tauri-apps/api/core');

    try {
      await invoke('clear_js_execution_history');
    } catch (error) {
      console.error('Failed to clear execution history:', error);
    }
  },
};

// 코드 실행 액션들
export const executionActions = {
  // 코드 실행
  executeCode: async (code: string) => {
    if (executionState.isExecuting.get()) return;

    executionState.isExecuting.set(true);
    executionState.result.set(null);

    try {
      const result = await executeApi.executeCode(code);
      executionState.result.set(result);

      // 히스토리에 추가 (성공한 경우만)
      if (result.success) {
        const currentHistory = executionState.history.get();
        const newHistory = [result, ...currentHistory];
        // 히스토리 최대 50개로 제한
        if (newHistory.length > 50) {
          newHistory.splice(50);
        }
        executionState.history.set(newHistory);
      }
    } catch (error) {
      executionState.result.set({
        code,
        result: '',
        timestamp: new Date().toISOString(),
        success: false,
        error: error instanceof Error ? error.message : 'Unknown error',
      });
    } finally {
      executionState.isExecuting.set(false);
    }
  },

  // 결과 초기화
  clearResult: () => {
    executionState.result.set(null);
  },

  // 히스토리 로드
  loadHistory: async () => {
    try {
      const history = await executeApi.getHistory();
      executionState.history.set(history);
    } catch (error) {
      console.error('Failed to load execution history:', error);
    }
  },

  // 히스토리 삭제
  clearHistory: async () => {
    try {
      await executeApi.clearHistory();
      executionState.history.set([]);
    } catch (error) {
      console.error('Failed to clear execution history:', error);
    }
  },
};
