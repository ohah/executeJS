import { Cross2Icon, PlusIcon } from '@radix-ui/react-icons';

import { usePlaygroundStore } from '@/features/playground';
import { PlaygroundWidget } from '@/widgets/playground';

export const PlaygroundGroups: React.FC = () => {
  const { tabs, addTab, closeTab, setActiveTab, playgrounds } =
    usePlaygroundStore();

  return (
    <div className="overflow-hidden w-screen h-screen">
      <div className="overflow-x-auto flex items-center border-b border-slate-800">
        <div className="flex shrink-0">
          {tabs.map(({ id, title, active }) => {
            return (
              <div
                key={id}
                className="shrink-0 flex items-center p-2 border-r border-slate-800"
                // TODO: 활성화된 탭 스타일링 개선 @bori
                style={{
                  backgroundColor: active
                    ? 'rgba(255, 255, 255, 0.1)'
                    : 'transparent',
                }}
              >
                {/* TODO: 탭 최대 너비에 따른 제목 ellipsis 처리 @bori */}
                <button
                  type="button"
                  onClick={() => setActiveTab(id)}
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
                  onClick={() => closeTab(id)}
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

      {tabs.map((tab) => {
        const { active, playgroundId, id } = tab;
        const playground = playgrounds.get(playgroundId);

        if (!active || !playground) return null;

        return <PlaygroundWidget key={id} playground={playground} />;
      })}
    </div>
  );
};
