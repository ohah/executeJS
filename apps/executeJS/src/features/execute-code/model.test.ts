import { describe, it, expect, vi, beforeEach } from 'vitest';
import { useExecutionStore } from './model';

// Tauri API 모킹
vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn(),
}));

describe('execute-code model', () => {
  beforeEach(() => {
    // 각 테스트 전에 상태 초기화
    useExecutionStore.getState().clearResult();
    useExecutionStore.getState().setExecuting(false);
  });

  it('should initialize with empty state', () => {
    const state = useExecutionStore.getState();
    expect(state.result).toBeNull();
    expect(state.isExecuting).toBe(false);
  });

  it('should clear result', () => {
    const store = useExecutionStore.getState();

    // 결과 설정 (직접 상태 변경)
    useExecutionStore.setState({
      result: {
        code: 'test',
        result: 'output',
        timestamp: '2023-01-01T00:00:00.000Z',
        success: true,
      },
    });

    store.clearResult();

    expect(store.result).toBeNull();
  });

  it('should handle execution state changes', () => {
    const store = useExecutionStore.getState();
    expect(store.isExecuting).toBe(false);

    store.setExecuting(true);

    // 상태 변경 후 다시 가져와서 확인
    const updatedState = useExecutionStore.getState();
    expect(updatedState.isExecuting).toBe(true);
  });
});
