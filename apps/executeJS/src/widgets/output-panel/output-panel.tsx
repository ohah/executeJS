import React from 'react';
import { PlayIcon } from '@radix-ui/react-icons';
import type { OutputPanelProps } from '../../shared/types';

export const OutputPanel: React.FC<OutputPanelProps> = ({
  result,
  isExecuting,
}) => {
  if (isExecuting) {
    return (
      <div className="h-full w-full p-6 bg-slate-900">
        <div className="flex items-center justify-center h-full">
          <div className="flex items-center gap-3 text-slate-400">
            <div className="animate-spin w-5 h-5 border-2 border-blue-500 border-t-transparent rounded-full"></div>
            <span className="text-sm font-medium">코드를 실행 중입니다...</span>
          </div>
        </div>
      </div>
    );
  }

  if (!result) {
    return (
      <div className="h-full w-full p-6 bg-slate-900">
        <div className="flex items-center justify-center h-full">
          <div className="text-center">
            <div className="w-16 h-16 mx-auto mb-4 bg-slate-800 rounded-lg flex items-center justify-center">
              <PlayIcon className="w-8 h-8 text-slate-500" />
            </div>
            <h3 className="text-lg font-medium text-slate-300 mb-2">코드를 실행해보세요</h3>
            <p className="text-sm text-slate-500">
              Cmd+Enter를 눌러 JavaScript 코드를 실행할 수 있습니다
            </p>
          </div>
        </div>
      </div>
    );
  }

  return (
    <div className="h-full w-full bg-slate-900 overflow-y-auto">
      {/* 실행 결과만 표시 - 화면 전체 사용 */}
      <div className="h-full flex items-center justify-center p-6">
        <div className="w-full">
          <pre className="text-sm font-mono whitespace-pre-wrap break-words overflow-x-auto h-full flex items-center justify-center">
            <code className={result.success ? 'text-green-400' : 'text-red-400'}>
              {result.success ? result.result : result.error}
            </code>
          </pre>
        </div>
      </div>
    </div>
  );
};
