import { useRef, useState } from 'react';

import { Cross2Icon } from '@radix-ui/react-icons';

import { Tab } from '@/features/playground';
import { useClickOutside } from '@/shared';
import { TabContextMenu } from '@/pages/playground';

import { TabTitleModal } from './tab-title-modal';

interface TabButtonProps {
  tab: Tab;
  isActive: boolean;
  contextMenu: TabContextMenu | null;
  onActiveTab: (id: Tab['id']) => void;
  onCloseTab: (id: Tab['id']) => void;
  onContextMenu: (event: React.MouseEvent, id: Tab['id']) => void;
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

  const [openChangeTabTitleModal, setOpenChangeTabTitleModal] =
    useState<boolean>(false);

  useClickOutside(ref, onCloseContextMenu);

  const handleOpenChangeTabTitleModal = () => {
    onCloseContextMenu();
    setOpenChangeTabTitleModal(true);
  };

  return (
    <>
      <div className={`shrink-0 p-1`}>
        <div
          className={`group flex items-center rounded-sm hover:bg-[rgba(255,255,255,0.1)] ${isActive ? 'bg-[rgba(255,255,255,0.1)]' : 'bg-transparent'}`}
        >
          <button
            type="button"
            onClick={() => onActiveTab(id)}
            onContextMenu={(event) => onContextMenu(event, id)}
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
        {contextMenu && contextMenu.id === id && (
          <div
            ref={ref}
            style={{ left: contextMenu.x, top: contextMenu.y }}
            className="absolute w-50 p-2 border border-slate-700 rounded-md bg-slate-900"
          >
            <ul>
              <li>
                <button
                  type="button"
                  className="w-full py-1 px-2 rounded-sm cursor-pointer text-left hover:bg-slate-800"
                  onClick={handleOpenChangeTabTitleModal}
                >
                  Change tab title
                </button>
              </li>
              <li>
                <button
                  type="button"
                  className="w-full py-1 px-2 rounded-sm cursor-pointer text-left hover:bg-slate-800"
                  onClick={() => onCloseTab(id)}
                >
                  Close tab
                </button>
              </li>
            </ul>
          </div>
        )}
      </div>

      {openChangeTabTitleModal && (
        <TabTitleModal
          tab={tab}
          onClose={() => setOpenChangeTabTitleModal(false)}
        />
      )}
    </>
  );
};
