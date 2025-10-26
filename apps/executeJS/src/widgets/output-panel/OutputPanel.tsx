import React from 'react';
import { Box, Text, Flex, Spinner } from '@radix-ui/themes';
import { CheckCircledIcon, CrossCircledIcon } from '@radix-ui/react-icons';
import type { OutputPanelProps } from '../../shared/types';

export const OutputPanel: React.FC<OutputPanelProps> = ({
  result,
  isExecuting,
}) => {
  const currentResult = result.get();
  const currentIsExecuting = isExecuting.get();

  if (currentIsExecuting) {
    return (
      <Box className="h-full w-full p-4 bg-gray-2">
        <Flex align="center" gap="2" className="text-gray-11">
          <Spinner size="2" />
          <Text size="2">코드를 실행 중입니다...</Text>
        </Flex>
      </Box>
    );
  }

  if (!currentResult) {
    return (
      <Box className="h-full w-full p-4 bg-gray-2">
        <Flex align="center" justify="center" className="h-full">
          <Text size="3" className="text-gray-9">
            Cmd+Enter를 눌러 코드를 실행하세요
          </Text>
        </Flex>
      </Box>
    );
  }

  return (
    <Box className="h-full w-full bg-gray-2 flex flex-col">
      {/* 결과 헤더 */}
      <Flex
        className="px-4 py-2 border-b border-gray-6 bg-gray-3"
        align="center"
        gap="2"
      >
        {currentResult.success ? (
          <CheckCircledIcon className="text-green-9" width="16" height="16" />
        ) : (
          <CrossCircledIcon className="text-red-9" width="16" height="16" />
        )}
        <Text size="2" className="font-medium">
          {currentResult.success ? '실행 성공' : '실행 실패'}
        </Text>
        <Text size="1" className="text-gray-9 ml-auto">
          {new Date(currentResult.timestamp).toLocaleTimeString()}
        </Text>
      </Flex>

      {/* 결과 내용 */}
      <Box className="flex-1 p-4 overflow-auto">
        <pre className="text-sm font-mono text-gray-12 whitespace-pre-wrap break-words">
          {currentResult.success ? currentResult.result : currentResult.error}
        </pre>
      </Box>
    </Box>
  );
};
