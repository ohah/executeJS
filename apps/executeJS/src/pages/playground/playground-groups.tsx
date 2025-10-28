import { useState } from 'react';
import { PlaygroundPage } from './playground-page';
import { PlusIcon } from '@radix-ui/react-icons';

interface Tab {
  id: string;
  title: string;
}

const initialTabs: Tab[] = [{ id: 'playground-1', title: 'Playground 1' }];

export const PlaygroundGroups: React.FC = () => {
  const [tabs, setTabs] = useState<Tab[]>(initialTabs);

  return (
    <div>
      <div className="flex items-center border-b border-slate-800">
        <div className="flex">
          {tabs.map(({ id, title }) => {
            return (
              <div key={id} className="p-2 border-r border-slate-800">
                {/* TODO: 탭 최대 너비에 따른 제목 ellipsis 처리 */}
                <span>{title}</span>
              </div>
            );
          })}
        </div>

        <button type="button" onClick={() => {}} className="p-2 cursor-pointer">
          <PlusIcon />
        </button>
      </div>
      <PlaygroundPage />
    </div>
  );
};
