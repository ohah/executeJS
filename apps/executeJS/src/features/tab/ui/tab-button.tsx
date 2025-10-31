import { Cross2Icon } from '@radix-ui/react-icons';
import { useRef } from 'react';

import { Tab } from '@/features/playground';
import { useClickOutside } from '@/shared';

interface TabButtonProps {
  tab: Tab;
  isActive: boolean;
  contextMenu: { x: number; y: number } | null;
  onActiveTab: (id: Tab['id']) => void;
  onCloseTab: (id: Tab['id']) => void;
  onContextMenu: (event: React.MouseEvent) => void;
  onCloseContextMenu: () => void;
}

export const TabButton: React.FC<TabButtonProps> = ({
  tab,
  isActive,
  contextMenu,
  onActiveTab,
  onCloseTab,
  onContextMenu,
  onCloseContextMenu,
}) => {
  const { id, title } = tab;

  const ref = useRef<HTMLDivElement>(null);

  useClickOutside(ref, onCloseContextMenu);

  return (
    <div className={`shrink-0 p-1`}>
      <div
        className={`group flex items-center rounded-sm hover:bg-[rgba(255,255,255,0.1)] ${isActive ? 'bg-[rgba(255,255,255,0.1)]' : 'bg-transparent'}`}
      >
        <button
          type="button"
          onClick={() => onActiveTab(id)}
          onContextMenu={onContextMenu}
          className={`group-hover:text-gray-50 w-40 pl-3 pr-2 truncate text-left cursor-pointer select-none ${isActive ? 'text-gray-50' : 'text-gray-500'}`}
        >
          {title}
        </button>
        <button
          type="button"
          onClick={() => onCloseTab(id)}
          className="h-full p-2 rounded-r-sm hover:bg-[rgba(255,255,255,0.1)] transition-colors cursor-pointer"
        >
          <Cross2Icon
            className={`group-hover:text-gray-50 ${isActive ? 'text-gray-50' : 'text-gray-500'}`}
          />
        </button>
      </div>

      {contextMenu && (
        <div
          ref={ref}
          style={{ left: contextMenu.x, top: contextMenu.y }}
          className="absolute w-50 py-1 px-2 border border-slate-700 rounded-sm bg-slate-900"
        >
          우클릭 메뉴
        </div>
      )}
    </div>
  );
};
