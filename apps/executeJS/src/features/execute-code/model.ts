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
        } catch (error: any) {
          //TODO: @ohah 에러 처리 더 명확하게 할 것
          const result: JsExecutionResult = {
            code,
            result: error?.result ?? '',
            timestamp: new Date().toISOString(),
            success: false,
            error: error?.error ?? '알 수 없는 오류',
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
      onRehydrateStorage: () => (state, error) => {
        if (error) {
          console.error('hydration failed:', error);
          // persist 실패 시 localStorage 정리
          localStorage.removeItem('executejs-execution-storage');
          
          return;
        }
        
        if (state) {
          console.log('hydration success');
          // 앱 재시작 시 실행 중이었던 상태를 초기화
          state.setExecuting(false);
        }
      },
    }
  )
);
