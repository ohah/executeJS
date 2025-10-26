import React from 'react';
import { Flex, Text, IconButton } from '@radix-ui/themes';
import { Cross2Icon, PlusIcon } from '@radix-ui/react-icons';
import type { TabBarProps } from '../../shared/types';

export const TabBar: React.FC<TabBarProps> = ({
  tabs,
  activeTabId,
  onTabSelect,
  onTabClose,
  onTabAdd,
}) => {
  const tabsArray = tabs.get();
  const currentActiveTabId = activeTabId.get();

  return (
    <Flex
      className="bg-gray-3 border-b border-gray-6 h-10"
      align="center"
      gap="0"
    >
      {/* 탭 목록 */}
      <Flex className="flex-1 overflow-x-auto" align="center">
        {tabsArray.map((tab: any) => (
          <Flex
            key={tab.id}
            className={`
              px-3 py-2 cursor-pointer border-r border-gray-6 min-w-0 flex-1 max-w-48
              transition-colors duration-150
              ${
                tab.id === currentActiveTabId
                  ? 'bg-gray-2 text-gray-12'
                  : 'bg-gray-3 text-gray-11 hover:bg-gray-4'
              }
            `}
            align="center"
            gap="2"
            onClick={() => onTabSelect(tab.id)}
          >
            <Text size="2" className="truncate flex-1" title={tab.name}>
              {tab.name}
              {tab.isDirty && '*'}
            </Text>

            {tabsArray.length > 1 && (
              <IconButton
                size="1"
                variant="ghost"
                className="opacity-0 group-hover:opacity-100 hover:bg-gray-6"
                onClick={(e) => {
                  e.stopPropagation();
                  onTabClose(tab.id);
                }}
              >
                <Cross2Icon width="12" height="12" />
              </IconButton>
            )}
          </Flex>
        ))}
      </Flex>

      {/* 새 탭 추가 버튼 */}
      <IconButton
        size="2"
        variant="ghost"
        className="mx-2 hover:bg-gray-6"
        onClick={onTabAdd}
        title="새 탭 추가"
      >
        <PlusIcon width="16" height="16" />
      </IconButton>
    </Flex>
  );
};
