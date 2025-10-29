import { JsExecutionResult } from '@/shared';
import { create } from 'zustand';
import { persist, createJSONStorage } from 'zustand/middleware';

interface Tab {
  id: string;
  playgroundId: Playground['id'];
  title: string;
  active: boolean;
}

export interface Playground {
  id: string;
  result: JsExecutionResult | null;
  isExecuting: boolean;
  // setExecuting: (executing: boolean) => void;
  // executeCode: (code: string) => void;
  // clearResult: () => void;
}

interface PlaygroundState {
  tabs: Tab[];
  playgrounds: Map<Playground['id'], Playground>;
  addTab: () => void;
  closeTab: (tabId: Tab['id']) => void;
}

const INITIAL_TAB_TITLE = '✨New Playground';
const INITIAL_TAB_ID = 'first-playground-tab';
const INITIAL_PLAYGROUND_ID = 'first-playground';

const initialTab: Tab = {
  id: INITIAL_TAB_ID,
  playgroundId: INITIAL_PLAYGROUND_ID,
  active: true,
  title: INITIAL_TAB_TITLE,
};

export const usePlaygroundStore = create<PlaygroundState>()(
  persist(
    (set) => ({
      tabs: [initialTab],
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
            active: true,
            title: INITIAL_TAB_TITLE,
          };

          const prevTabs = state.tabs.map((tab) => ({
            ...tab,
            active: false,
          }));

          const newPlaygrounds = new Map(state.playgrounds);
          newPlaygrounds.set(newPlaygroundId, {
            id: newPlaygroundId,
            result: null,
            isExecuting: false,
          });

          return {
            tabs: [...prevTabs, newTab],
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
          const playgrounds = new Map(state.playgrounds);

          playgrounds.delete(closingTab.playgroundId);

          return {
            tabs,
            playgrounds,
          };
        });
      },
    }),
    {
      name: 'executejs-playground-store',
      partialize: (state) => ({
        tabs: state.tabs,
        playgrounds: state.playgrounds,
      }),
      storage: createJSONStorage(() => localStorage, {
        replacer: (key, value) => {
          if (value instanceof Map) {
            return {
              _type: 'map',
              value: Array.from(value.entries()),
            };
          }
          return value;
        },
        reviver: (key, value: any) => {
          if (value && value._type === 'map') {
            return new Map(value.value);
          }
          return value;
        },
      }),
    }
  )
);
