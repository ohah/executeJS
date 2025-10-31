import { JsExecutionResult } from '@/shared';
import { create } from 'zustand';
import { persist, createJSONStorage } from 'zustand/middleware';
import { PLAYGROUND_STORAGE_KEY } from './const';

export interface Tab {
  id: string;
  playgroundId: Playground['id'];
  title: string;
}

export interface Playground {
  id: string;
  result: JsExecutionResult | null;
  isExecuting: boolean;
  // clearResult: () => void;
}

interface PlaygroundState {
  tabs: Tab[];
  activeTabId: Tab['id'];
  tabHistory: Tab['id'][];
  playgrounds: Map<Playground['id'], Playground>;
  addTab: () => void;
  closeTab: (tabId: Tab['id']) => void;
  setActiveTab: (tabId: Tab['id']) => void;
  executeCode: (params: {
    playgroundId: Playground['id'];
    code: string;
  }) => void;
}

const INITIAL_TAB_TITLE = '✨New Playground';
const INITIAL_TAB_ID = 'first-playground-tab';
const INITIAL_PLAYGROUND_ID = 'first-playground';

const initialTab: Tab = {
  id: INITIAL_TAB_ID,
  playgroundId: INITIAL_PLAYGROUND_ID,
  title: INITIAL_TAB_TITLE,
};

export const usePlaygroundStore = create<PlaygroundState>()(
  persist(
    (set) => ({
      tabs: [initialTab],
      activeTabId: INITIAL_TAB_ID,
      tabHistory: [INITIAL_TAB_ID],
      playgrounds: new Map([
        [
          INITIAL_PLAYGROUND_ID,
          { id: INITIAL_PLAYGROUND_ID, result: null, isExecuting: false },
        ],
      ]),

      // 탭 추가
      addTab: () => {
        set((state) => {
          const date = new Date().valueOf();

          const newTabId = `playground-tab-${date}`;
          const newPlaygroundId = `playground-${date}`;

          const newTab: Tab = {
            id: newTabId,
            playgroundId: newPlaygroundId,
            title: INITIAL_TAB_TITLE,
          };

          const newPlaygrounds = new Map(state.playgrounds);
          newPlaygrounds.set(newPlaygroundId, {
            id: newPlaygroundId,
            result: null,
            isExecuting: false,
          });

          return {
            tabs: [...state.tabs, newTab],
            activeTabId: newTabId,
            tabHistory: [...state.tabHistory, newTabId],
            playgrounds: newPlaygrounds,
          };
        });
      },

      // 탭 닫기
      closeTab: (tabId: Tab['id']) => {
        set((state) => {
          const closingTab = state.tabs.find((tab) => tab.id === tabId);
          const tabsLength = state.tabs.length;

          if (!closingTab || tabsLength === 1) return state;

          const tabs = state.tabs.filter((tab) => tab.id !== tabId);
          const tabHistory = state.tabHistory.filter((id) => id !== tabId);
          const lastActiveTabId =
            tabHistory[tabHistory.length - 1] || tabs[0].id;
          const playgrounds = new Map(state.playgrounds);

          playgrounds.delete(closingTab.playgroundId);

          return {
            tabs,
            activeTabId: lastActiveTabId,
            tabHistory,
            playgrounds,
          };
        });
      },

      // 탭 활성화
      setActiveTab: (tabId: Tab['id']) => {
        set((state) => {
          const lastTabId = state.tabHistory[state.tabHistory.length - 1];

          if (lastTabId === tabId) {
            return state;
          }

          return {
            activeTabId: tabId,
            tabHistory: [...state.tabHistory, tabId],
          };
        });
      },

      // 플레이그라운드 별 코드 실행
      executeCode: async ({
        playgroundId,
        code,
      }: {
        playgroundId: Playground['id'];
        code: string;
      }) => {
        set((state) => {
          const playgrounds = new Map(state.playgrounds);
          const playground = playgrounds.get(playgroundId);

          if (playground) {
            playgrounds.set(playgroundId, { ...playground, isExecuting: true });
          }

          return { playgrounds };
        });

        try {
          // Tauri 백엔드의 execute_js 명령어 호출
          const { invoke } = await import('@tauri-apps/api/core');
          const result = await invoke<JsExecutionResult>('execute_js', {
            code,
          });

          console.log('executeCode result -', result);

          set((state) => {
            const playgrounds = new Map(state.playgrounds);
            const playground = playgrounds.get(playgroundId);

            if (playground) {
              playgrounds.set(playgroundId, {
                ...playground,
                result: result,
                isExecuting: false,
              });
            }

            return { playgrounds };
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

          set((state) => {
            const playgrounds = new Map(state.playgrounds);
            const playground = playgrounds.get(playgroundId);

            if (playground) {
              playgrounds.set(playgroundId, {
                ...playground,
                result: result,
                isExecuting: false,
              });
            }

            return { playgrounds };
          });
        }
      },
    }),
    {
      name: PLAYGROUND_STORAGE_KEY,
      storage: createJSONStorage(() => localStorage, {
        replacer: (_, value) => {
          if (value instanceof Map) {
            return {
              _type: 'map',
              value: Array.from(value.entries()),
            };
          }
          return value;
        },
        reviver: (_, value: any) => {
          if (value && value._type === 'map') {
            return new Map(value.value);
          }
          return value;
        },
      }),
    }
  )
);
