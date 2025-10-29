import { Cross2Icon, PlusIcon } from '@radix-ui/react-icons';

import { usePlaygroundStore } from '@/features/playground';

import { PlaygroundPage } from './playground-page';

export const PlaygroundGroups: React.FC = () => {
  const { tabs, addTab } = usePlaygroundStore();

  const handleCloseTab = (tabId: string) => {
    // setTabs((prevTabs) => {
    //   if (prevTabs.length === 1) {
    //     return prevTabs; // 최소 하나의 탭은 유지
    //   }
    //   return prevTabs.filter((tab) => tab.id !== tabId);
    // });
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
                {/* TODO: 탭 최대 너비에 따른 제목 ellipsis 처리 @bori */}
                <button
                  type="button"
                  onClick={() => {
                    // TODO: 탭 전환 로직 @bori
                  }}
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
          onClick={addTab}
          className="shrink-0 p-2 ml-1 rounded-sm hover:bg-[rgba(255,255,255,0.2)] transition-colors cursor-pointer"
        >
          <PlusIcon />
        </button>
      </div>

      {/* TODO: 활성화된 탭에 따른 플레이그라운드 렌더링 @bori */}
      <PlaygroundPage />
    </div>
  );
};
