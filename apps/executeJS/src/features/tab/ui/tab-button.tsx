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
    <div
      className="shrink-0 flex items-center p-2 border-r border-slate-800"
      // TODO: 활성화된 탭 스타일링 개선 @bori
      style={{
        backgroundColor: isActive ? 'rgba(255, 255, 255, 0.1)' : 'transparent',
      }}
    >
      {/* TODO: 탭 최대 너비에 따른 제목 ellipsis 처리 @bori */}
      <button
        type="button"
        onClick={() => onActiveTab(id)}
        onContextMenu={(event) => {
          event.preventDefault();
          // TODO: 탭 우클릭 메뉴 로직 @bori
          console.log('우클릭 메뉴 -', id);
        }}
        className="pr-2"
      >
        {title}
      </button>
      <button
        type="button"
        onClick={() => onCloseTab(id)}
        className="p-1 rounded-sm hover:bg-[rgba(255,255,255,0.2)] transition-colors cursor-pointer"
      >
        <Cross2Icon />
      </button>
    </div>
  );
};
