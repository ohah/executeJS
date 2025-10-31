import { Cross2Icon } from '@radix-ui/react-icons';

import { Tab } from '@/features/playground';

interface TabButtonProps {
  tab: Tab;
  isActive: boolean;
  onActiveTab: (id: Tab['id']) => void;
  onCloseTab: (id: Tab['id']) => void;
}

export const TabButton: React.FC<TabButtonProps> = ({
  tab,
  isActive,
  onActiveTab,
  onCloseTab,
}) => {
  const { id, title } = tab;

  return (
    <div className={`shrink-0 p-1 border-slate-800`}>
      <div
        className={`group flex items-center rounded-sm hover:bg-[rgba(255,255,255,0.1)] ${isActive ? 'bg-[rgba(255,255,255,0.1)]' : 'bg-transparent'}`}
      >
        <button
          type="button"
          onClick={() => onActiveTab(id)}
          onContextMenu={(event) => {
            event.preventDefault();
            // TODO: 탭 우클릭 메뉴 로직 @bori
            console.log('우클릭 메뉴 -', id);
          }}
          className={`group-hover:text-gray-50 w-40 px-2 truncate cursor-pointer ${isActive ? 'text-gray-50' : 'text-gray-500'}`}
        >
          {title}
        </button>
        <button
          type="button"
          onClick={() => onCloseTab(id)}
          className="h-full p-2 rounded-r-sm rounded-br-sm hover:bg-[rgba(255,255,255,0.1)] transition-colors cursor-pointer"
        >
          <Cross2Icon
            className={`group-hover:text-gray-50 ${isActive ? 'text-gray-50' : 'text-gray-500'}`}
          />
        </button>
      </div>
    </div>
  );
};
