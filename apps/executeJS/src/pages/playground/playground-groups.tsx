import { useState } from 'react';
import { PlaygroundPage } from './playground-page';
import { Cross2Icon, PlusIcon } from '@radix-ui/react-icons';

interface Tab {
  id: string;
  title: string;
}

const initialTabs: Tab[] = [{ id: 'playground-1', title: 'Playground 1' }];

export const PlaygroundGroups: React.FC = () => {
  const [tabs, setTabs] = useState<Tab[]>(initialTabs);

  const handleCloseTab = (tabId: string) => {
    setTabs((prevTabs) => {
      if (prevTabs.length === 1) {
        return prevTabs; // 최소 하나의 탭은 유지
      }

      return prevTabs.filter((tab) => tab.id !== tabId);
    });
  };

  const handleAddTab = () => {
    setTabs((prevTabs) => {
      const newTabId = `playground-${tabs.length + 1}`;
      const newTab: Tab = {
        id: newTabId,
        title: `Playground ${tabs.length + 1}`,
      };

      return [...prevTabs, newTab];
    });
  };

  return (
    <div className="overflow-hidden w-screen h-screen">
      <div className="overflow-x-auto flex items-center border-b border-slate-800">
        <div className="flex shrink-0">
          {tabs.map(({ id, title }) => {
            return (
              <div
                key={id}
                className="shrink-0 flex items-center p-2 border-r border-slate-800"
              >
                {/* TODO: 탭 최대 너비에 따른 제목 ellipsis 처리 */}
                <button
                  type="button"
                  onClick={() => {
                    // TODO: 탭 전환 로직 @bori
                  }}
                  className="pr-2"
                >
                  {title}
                </button>
                <button
                  type="button"
                  onClick={() => handleCloseTab(id)}
                  className="p-1 rounded-sm hover:bg-[rgba(255,255,255,0.2)] transition-colors cursor-pointer"
                >
                  <Cross2Icon />
                </button>
              </div>
            );
          })}
        </div>

        <button
          type="button"
          onClick={handleAddTab}
          className="shrink-0 p-2 cursor-pointer"
        >
          <PlusIcon />
        </button>
      </div>
      <PlaygroundPage />
    </div>
  );
};
