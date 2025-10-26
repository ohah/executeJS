import { create } from 'zustand';
import { persist } from 'zustand/middleware';
import type { JsExecutionResult } from '../../shared/types';

// 실행 상태 인터페이스
interface ExecutionState {
  result: JsExecutionResult | null;
  isExecuting: boolean;
  executeCode: (code: string) => void;
  setExecuting: (executing: boolean) => void;
  clearResult: () => void;
}

// 실행 상태 관리
export const useExecutionStore = create<ExecutionState>()(
  persist(
    (set) => ({
      result: null,
      isExecuting: false,

      // 코드 실행
      executeCode: async (code: string) => {
        console.log('executeCode called -', code);

        set({ isExecuting: true });

        try {
          // Tauri 백엔드의 execute_js 명령어 호출
          const { invoke } = await import('@tauri-apps/api/core');
          const result = await invoke<JsExecutionResult>('execute_js', {
            code,
          });

          console.log('executeCode result -', result);

          set({
            result,
            isExecuting: false,
          });
        } catch (error) {
          const result: JsExecutionResult = {
            code,
            result: '',
            timestamp: new Date().toISOString(),
            success: false,
            error: error instanceof Error ? error.message : '알 수 없는 오류',
          };

          set({
            result,
            isExecuting: false,
          });
        }
      },

      // 실행 상태 설정
      setExecuting: (executing: boolean) => {
        set({ isExecuting: executing });
      },

      // 결과 초기화
      clearResult: () => {
        set({ result: null });
      },
    }),
    {
      name: 'executejs-execution-storage',
      partialize: (state) => ({
        result: state.result,
      }),
    }
  )
);
