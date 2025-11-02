import { PlusIcon } from '@radix-ui/react-icons';

import { TabButton } from '@/features/tab';
import { Tab, usePlaygroundStore } from '@/features/playground';
import { PlaygroundWidget } from '@/widgets/playground';
import { useState } from 'react';

export interface TabContextMenu {
  id: Tab['id'];
  x: number;
  y: number;
}

export const PlaygroundGroups: React.FC = () => {
  const { tabs, activeTabId, addTab, closeTab, setActiveTab, playgrounds } =
    usePlaygroundStore();

  const [contextMenu, setContextMenu] = useState<TabContextMenu | null>(null);

  const handleContextMenu = (event: React.MouseEvent, tabId: string) => {
    event.preventDefault();

    setContextMenu({ id: tabId, x: event.clientX, y: event.clientY });
  };

  const handleCloseContextMenu = () => setContextMenu(null);

  return (
    <div className="overflow-hidden w-screen h-screen">
      <div className="overflow-x-auto flex items-center border-b border-slate-800">
        <div className="flex shrink-0">
          {tabs.map((tab) => {
            const { id } = tab;
            const isActive = id === activeTabId;

            return (
              <TabButton
                key={id}
                tab={tab}
                isActive={isActive}
                contextMenu={contextMenu}
                onActiveTab={setActiveTab}
                onCloseTab={closeTab}
                onContextMenu={handleContextMenu}
                onCloseContextMenu={handleCloseContextMenu}
              />
            );
          })}
        </div>

        <button
          type="button"
          onClick={addTab}
          className="shrink-0 p-2 ml-1 rounded-sm hover:bg-[rgba(255,255,255,0.2)] transition-colors cursor-pointer"
        >
          <PlusIcon />
        </button>
      </div>

      {tabs.map((tab) => {
        const { playgroundId, id } = tab;
        const active = tab.id === activeTabId;
        const playground = playgrounds.get(playgroundId);

        if (!active || !playground) return null;

        return <PlaygroundWidget key={id} playground={playground} />;
      })}
    </div>
  );
};
