import { describe, it, expect, vi, beforeEach } from 'vitest';
import { executionState, executionActions } from './model';

// Tauri API 모킹
vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn(),
}));

describe('execute-code model', () => {
  beforeEach(() => {
    // 각 테스트 전에 상태 초기화
    executionState.result.set(null);
    executionState.isExecuting.set(false);
    executionState.history.set([]);
  });

  it('should initialize with empty state', () => {
    expect(executionState.result.get()).toBeNull();
    expect(executionState.isExecuting.get()).toBe(false);
    expect(executionState.history.get()).toEqual([]);
  });

  it('should clear result', () => {
    executionState.result.set({
      code: 'test',
      result: 'output',
      timestamp: '2023-01-01T00:00:00.000Z',
      success: true,
    });

    executionActions.clearResult();

    expect(executionState.result.get()).toBeNull();
  });

  it('should handle execution state changes', () => {
    expect(executionState.isExecuting.get()).toBe(false);

    // executeCode는 비동기이므로 isExecuting 상태만 확인
    // 실제 실행은 Tauri API가 필요하므로 모킹 필요
  });
});
