import { PlusIcon } from '@radix-ui/react-icons';

import { TabButton } from '@/features/tab';
import { usePlaygroundStore } from '@/features/playground';
import { PlaygroundWidget } from '@/widgets/playground';

export const PlaygroundGroups: React.FC = () => {
  const { tabs, activeTabId, addTab, closeTab, setActiveTab, playgrounds } =
    usePlaygroundStore();

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
                onActiveTab={setActiveTab}
                onCloseTab={closeTab}
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
