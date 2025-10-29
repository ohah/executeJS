import { JsExecutionResult } from '@/shared';
import { create } from 'zustand';
import { persist } from 'zustand/middleware';

interface Tab {
  id: string;
  playgroundId: Playground['id'];
  title: string;
  active: boolean;
}

interface Playground {
  id: string;
  result: JsExecutionResult | null;
  isExecuting: boolean;
  setExecuting: (executing: boolean) => void;
  // executeCode: (code: string) => void;
  // clearResult: () => void;
}

interface PlaygroundState {
  tabs: Tab[];
  playgrounds: Record<Playground['id'], Playground>;
  addTab: () => void;
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
      playgrounds: {},

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

          return {
            tabs: [...prevTabs, newTab],
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
    }
  )
);
