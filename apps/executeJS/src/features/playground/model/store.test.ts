import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import { usePlaygroundStore } from './store';
import type { JsExecutionResult } from '@/shared';

// Tauri API 모킹
vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn(),
}));

// localStorage 모킹
const localStorageMock = (() => {
  let store: Record<string, string> = {};

  return {
    getItem: (key: string) => store[key] || null,
    setItem: (key: string, value: string) => {
      store[key] = value.toString();
    },
    removeItem: (key: string) => {
      delete store[key];
    },
    clear: () => {
      store = {};
    },
  };
})();

Object.defineProperty(window, 'localStorage', {
  value: localStorageMock,
});

describe('playground store', () => {
  beforeEach(() => {
    // 각 테스트 전에 localStorage 초기화
    localStorage.clear();
    // 스토어 상태 초기화
    usePlaygroundStore.setState({
      tabs: [
        {
          id: 'first-playground-tab',
          playgroundId: 'first-playground',
          title: 'New Tab',
        },
      ],
      activeTabId: 'first-playground-tab',
      tabHistory: ['first-playground-tab'],
      playgrounds: new Map([
        [
          'first-playground',
          { id: 'first-playground', result: null, isExecuting: false },
        ],
      ]),
    });
  });

  afterEach(() => {
    localStorage.clear();
  });

  describe('초기 상태', () => {
    it('초기 상태가 올바르게 설정되어야 함', () => {
      const state = usePlaygroundStore.getState();

      expect(state.tabs).toHaveLength(1);
      expect(state.tabs[0].id).toBe('first-playground-tab');
      expect(state.tabs[0].playgroundId).toBe('first-playground');
      expect(state.tabs[0].title).toBe('New Tab');
      expect(state.activeTabId).toBe('first-playground-tab');
      expect(state.tabHistory).toEqual(['first-playground-tab']);
      expect(state.playgrounds.size).toBe(1);
      expect(state.playgrounds.get('first-playground')).toEqual({
        id: 'first-playground',
        result: null,
        isExecuting: false,
      });
    });
  });

  describe('addTab', () => {
    it('새 탭을 추가해야 함', () => {
      const store = usePlaygroundStore.getState();
      const initialTabsCount = store.tabs.length;

      store.addTab();

      const updatedState = usePlaygroundStore.getState();
      expect(updatedState.tabs).toHaveLength(initialTabsCount + 1);
      expect(updatedState.tabs[updatedState.tabs.length - 1].title).toBe(
        'New Tab'
      );
    });

    it('새 탭이 자동으로 활성화되어야 함', () => {
      const store = usePlaygroundStore.getState();
      const initialActiveTabId = store.activeTabId;

      store.addTab();

      const updatedState = usePlaygroundStore.getState();
      const newTabId = updatedState.activeTabId;
      expect(newTabId).not.toBe(initialActiveTabId);
      expect(newTabId).toMatch(/^playground-tab-\d+$/);
    });

    it('새 플레이그라운드가 생성되어야 함', () => {
      const store = usePlaygroundStore.getState();
      const initialPlaygroundsCount = store.playgrounds.size;

      store.addTab();

      const updatedState = usePlaygroundStore.getState();
      expect(updatedState.playgrounds.size).toBe(initialPlaygroundsCount + 1);

      const newTab = updatedState.tabs[updatedState.tabs.length - 1];
      const newPlayground = updatedState.playgrounds.get(newTab.playgroundId);
      expect(newPlayground).toEqual({
        id: newTab.playgroundId,
        result: null,
        isExecuting: false,
      });
    });

    it('탭 히스토리에 새 탭이 추가되어야 함', () => {
      const store = usePlaygroundStore.getState();

      store.addTab();

      const updatedState = usePlaygroundStore.getState();
      const newTabId = updatedState.activeTabId;
      expect(updatedState.tabHistory).toContain(newTabId);
      expect(updatedState.tabHistory[updatedState.tabHistory.length - 1]).toBe(
        newTabId
      );
    });
  });

  describe('closeTab', () => {
    it('탭을 닫을 수 있어야 함', () => {
      const store = usePlaygroundStore.getState();

      // 새 탭 추가
      store.addTab();
      const tabsBeforeClose = usePlaygroundStore.getState().tabs.length;

      // 첫 번째 탭 닫기
      store.closeTab('first-playground-tab');

      const updatedState = usePlaygroundStore.getState();
      expect(updatedState.tabs).toHaveLength(tabsBeforeClose - 1);
      expect(
        updatedState.tabs.find((tab) => tab.id === 'first-playground-tab')
      ).toBeUndefined();
    });

    it('탭을 닫으면 해당 플레이그라운드도 삭제되어야 함', () => {
      const store = usePlaygroundStore.getState();

      // 새 탭 추가
      store.addTab();
      const stateBeforeClose = usePlaygroundStore.getState();
      const closingTab = stateBeforeClose.tabs.find(
        (tab) => tab.id === 'first-playground-tab'
      );

      if (closingTab) {
        const playgroundId = closingTab.playgroundId;
        expect(stateBeforeClose.playgrounds.has(playgroundId)).toBe(true);

        // 탭 닫기
        store.closeTab('first-playground-tab');

        const updatedState = usePlaygroundStore.getState();
        expect(updatedState.playgrounds.has(playgroundId)).toBe(false);
      }
    });

    it('마지막 탭은 닫을 수 없어야 함', () => {
      const store = usePlaygroundStore.getState();
      const tabsBeforeClose = store.tabs.length;

      // 마지막 탭 닫기 시도
      store.closeTab('first-playground-tab');

      const updatedState = usePlaygroundStore.getState();
      expect(updatedState.tabs).toHaveLength(tabsBeforeClose);
      expect(updatedState.tabs[0].id).toBe('first-playground-tab');
    });

    it('탭을 닫으면 히스토리에서도 제거되어야 함', () => {
      const store = usePlaygroundStore.getState();

      // 여러 탭 추가
      store.addTab();
      store.addTab();
      const stateBeforeClose = usePlaygroundStore.getState();
      const closingTabId = stateBeforeClose.tabs[0].id;

      expect(stateBeforeClose.tabHistory).toContain(closingTabId);

      // 탭 닫기
      store.closeTab(closingTabId);

      const updatedState = usePlaygroundStore.getState();
      expect(updatedState.tabHistory).not.toContain(closingTabId);
    });

    it('탭을 닫으면 마지막 활성 탭이 활성화되어야 함', () => {
      const store = usePlaygroundStore.getState();

      // 탭 추가
      store.addTab();
      store.addTab();
      const tab2Id = usePlaygroundStore.getState().activeTabId;

      // 첫 번째 탭으로 전환
      store.setActiveTab('first-playground-tab');

      // 탭 닫기 (현재 활성 탭)
      store.closeTab('first-playground-tab');

      const updatedState = usePlaygroundStore.getState();
      // 마지막 활성 탭이 활성화되어야 함
      expect(updatedState.activeTabId).toBe(tab2Id);
    });
  });

  describe('setActiveTab', () => {
    it('탭을 활성화할 수 있어야 함', () => {
      const store = usePlaygroundStore.getState();

      // 새 탭 추가
      store.addTab();

      // 첫 번째 탭으로 전환
      store.setActiveTab('first-playground-tab');

      const updatedState = usePlaygroundStore.getState();
      expect(updatedState.activeTabId).toBe('first-playground-tab');
    });

    it('활성화된 탭이 히스토리 끝에 추가되어야 함', () => {
      const store = usePlaygroundStore.getState();

      // 새 탭 추가
      store.addTab();

      // 첫 번째 탭으로 전환
      store.setActiveTab('first-playground-tab');

      const updatedState = usePlaygroundStore.getState();
      expect(updatedState.tabHistory[updatedState.tabHistory.length - 1]).toBe(
        'first-playground-tab'
      );
    });

    it('이미 활성화된 탭을 활성화하면 상태가 변경되지 않아야 함', () => {
      const store = usePlaygroundStore.getState();
      const currentActiveTabId = store.activeTabId;
      const currentTabHistory = [...store.tabHistory];

      // 같은 탭 활성화
      store.setActiveTab(currentActiveTabId);

      const updatedState = usePlaygroundStore.getState();
      expect(updatedState.activeTabId).toBe(currentActiveTabId);
      expect(updatedState.tabHistory).toEqual(currentTabHistory);
    });
  });

  describe('setTabTitle', () => {
    it('탭 제목을 변경할 수 있어야 함', () => {
      const store = usePlaygroundStore.getState();
      const newTitle = 'My Custom Tab';

      store.setTabTitle({ tabId: 'first-playground-tab', title: newTitle });

      const updatedState = usePlaygroundStore.getState();
      const updatedTab = updatedState.tabs.find(
        (tab) => tab.id === 'first-playground-tab'
      );
      expect(updatedTab?.title).toBe(newTitle);
    });

    it('다른 탭의 제목은 변경되지 않아야 함', () => {
      const store = usePlaygroundStore.getState();

      // 새 탭 추가
      store.addTab();
      const stateBeforeTitleChange = usePlaygroundStore.getState();
      const otherTab = stateBeforeTitleChange.tabs.find(
        (tab) => tab.id !== 'first-playground-tab'
      );

      store.setTabTitle({
        tabId: 'first-playground-tab',
        title: 'Changed Title',
      });

      const updatedState = usePlaygroundStore.getState();
      if (otherTab) {
        const stillOtherTab = updatedState.tabs.find(
          (tab) => tab.id === otherTab.id
        );
        expect(stillOtherTab?.title).toBe(otherTab.title);
      }
    });
  });

  describe('executeCode', () => {
    it('코드 실행 시 isExecuting이 true가 되어야 함', async () => {
      const store = usePlaygroundStore.getState();
      const { invoke } = await import('@tauri-apps/api/core');

      // 비동기 호출 지연
      vi.mocked(invoke).mockImplementation(
        () =>
          new Promise((resolve) => {
            setTimeout(() => {
              resolve({
                code: 'console.log("test")',
                result: 'test',
                timestamp: new Date().toISOString(),
                success: true,
              });
            }, 100);
          })
      );

      const executePromise = store.executeCode({
        playgroundId: 'first-playground',
        code: 'console.log("test")',
      });

      // 즉시 isExecuting 상태 확인
      const stateDuringExecution = usePlaygroundStore.getState();
      const playground =
        stateDuringExecution.playgrounds.get('first-playground');
      expect(playground?.isExecuting).toBe(true);

      await executePromise;
    });

    it('코드 실행 성공 시 결과가 저장되어야 함', async () => {
      const store = usePlaygroundStore.getState();
      const { invoke } = await import('@tauri-apps/api/core');

      const mockResult: JsExecutionResult = {
        code: 'console.log("Hello")',
        result: 'Hello',
        timestamp: new Date().toISOString(),
        success: true,
      };

      vi.mocked(invoke).mockResolvedValue(mockResult);

      await store.executeCode({
        playgroundId: 'first-playground',
        code: 'console.log("Hello")',
      });

      const updatedState = usePlaygroundStore.getState();
      const playground = updatedState.playgrounds.get('first-playground');
      expect(playground?.result).toEqual(mockResult);
      expect(playground?.isExecuting).toBe(false);
    });

    it('코드 실행 실패 시 에러 결과가 저장되어야 함', async () => {
      const store = usePlaygroundStore.getState();
      const { invoke } = await import('@tauri-apps/api/core');

      const mockError = {
        result: 'Error output',
        error: 'Syntax Error',
      };

      vi.mocked(invoke).mockRejectedValue(mockError);

      await store.executeCode({
        playgroundId: 'first-playground',
        code: 'invalid code',
      });

      const updatedState = usePlaygroundStore.getState();
      const playground = updatedState.playgrounds.get('first-playground');

      expect(playground?.result).toBeDefined();
      expect(playground?.result?.success).toBe(false);
      expect(playground?.result?.error).toBe('Syntax Error');
      expect(playground?.result?.result).toBe('Error output');
      expect(playground?.isExecuting).toBe(false);
    });

    it('에러 객체에 result나 error가 없을 때 기본값이 사용되어야 함', async () => {
      const store = usePlaygroundStore.getState();
      const { invoke } = await import('@tauri-apps/api/core');

      vi.mocked(invoke).mockRejectedValue({});

      await store.executeCode({
        playgroundId: 'first-playground',
        code: 'test',
      });

      const updatedState = usePlaygroundStore.getState();
      const playground = updatedState.playgrounds.get('first-playground');

      expect(playground?.result).toBeDefined();
      expect(playground?.result?.success).toBe(false);
      expect(playground?.result?.error).toBe('알 수 없는 오류');
      expect(playground?.result?.result).toBe('');
      expect(playground?.isExecuting).toBe(false);
    });

    it('존재하지 않는 플레이그라운드에 대해 실행해도 에러가 발생하지 않아야 함', async () => {
      const store = usePlaygroundStore.getState();
      const { invoke } = await import('@tauri-apps/api/core');

      vi.mocked(invoke).mockResolvedValue({
        code: 'test',
        result: 'test',
        timestamp: new Date().toISOString(),
        success: true,
      });

      await expect(
        store.executeCode({
          playgroundId: 'non-existent-playground',
          code: 'test',
        })
      ).resolves.not.toThrow();

      const updatedState = usePlaygroundStore.getState();
      expect(updatedState.playgrounds.has('non-existent-playground')).toBe(
        false
      );
    });
  });
});
