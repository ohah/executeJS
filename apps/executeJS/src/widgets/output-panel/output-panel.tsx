import React from 'react';
import { Box, Text, Flex, Spinner } from '@radix-ui/themes';
import { CheckCircledIcon, CrossCircledIcon } from '@radix-ui/react-icons';
import type { OutputPanelProps } from '../../shared/types';

export const OutputPanel: React.FC<OutputPanelProps> = ({
  result,
  isExecuting,
}) => {
  console.log('OutputPanel Debug:', {
    result,
    isExecuting,
  });

  if (isExecuting) {
    return (
      <Box className="h-full w-full p-4 bg-gray-2">
        <Flex align="center" gap="2" className="text-gray-11">
          <Spinner size="2" />
          <Text size="2">코드를 실행 중입니다...</Text>
        </Flex>
      </Box>
    );
  }

  if (!result) {
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
    <Box className="h-full w-full bg-gray-2 p-4">
      {/* 실행 결과 헤더 */}
      <Flex align="center" gap="2" className="mb-4">
        {result.success ? (
          <CheckCircledIcon className="text-green-9" width="16" height="16" />
        ) : (
          <CrossCircledIcon className="text-red-9" width="16" height="16" />
        )}
        <Text size="3" className="font-medium">
          {result.success ? '실행 성공' : '실행 실패'}
        </Text>
        <Text size="2" className="text-gray-9 ml-auto">
          {new Date(result.timestamp).toLocaleTimeString()}
        </Text>
      </Flex>

      {/* 코드 */}
      <Box className="mb-4">
        <Text size="2" className="text-gray-11 mb-2 font-medium">
          실행한 코드:
        </Text>
        <pre className="text-sm font-mono text-gray-12 bg-gray-4 p-3 rounded border">
          {result.code}
        </pre>
      </Box>

      {/* 결과 */}
      <Box>
        <Text size="2" className="text-gray-11 mb-2 font-medium">
          실행 결과:
        </Text>
        <pre className="text-sm font-mono text-gray-12 whitespace-pre-wrap break-words bg-gray-4 p-3 rounded border min-h-[100px]">
          {result.success ? result.result : result.error}
        </pre>
      </Box>
    </Box>
  );
};
